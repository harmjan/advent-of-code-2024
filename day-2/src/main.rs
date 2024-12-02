use std::fs;

fn is_safe(report: &[u32]) -> bool {
    let distances = report
        .windows(2)
        .map(|pair| {
            let (a, b) = (pair[0], pair[1]);
            a as i32 - b as i32
        })
        .collect::<Vec<i32>>();

    let decreasing_or_increasing =
        distances.iter().all(|&d| d >= 0) || distances.iter().all(|&d| d <= 0);

    let safe_difference = distances.iter().map(|&d| d.abs()).all(|d| d >= 1 && d <= 3);

    decreasing_or_increasing && safe_difference
}

fn run(input: &str) {
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let safe_reports_v1 = reports.iter().filter(|&report| is_safe(report)).count();

    let safe_reports_v2 = reports
        .iter()
        .filter(|&report| {
            (0..report.len())
                .map(|i| {
                    let report_v2 = report
                        .iter()
                        .enumerate()
                        .filter_map(|(j, &x)| if i == j { None } else { Some(x) })
                        .collect::<Vec<_>>();

                    is_safe(&report_v2)
                })
                .any(|b| b)
        })
        .count();

    println!("Safe reports V1: {}", safe_reports_v1);
    println!("Safe reports V2: {}", safe_reports_v2);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    run(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Run with cargo test -- --nocapture to see the output
    #[test]
    fn sample_input() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        run(&input);
    }
}
