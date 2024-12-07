use std::fs;

fn is_possible_1(total: u64, sequence: &[u64]) -> bool {
    if sequence.len() == 1 {
        return total == sequence[0];
    }
    if total < sequence[0] {
        return false;
    }
    let mut new_sequence = sequence[1..].iter().copied().collect::<Vec<_>>();
    new_sequence[0] = sequence[0] + sequence[1];
    if is_possible_1(total, &new_sequence) {
        return true;
    }
    new_sequence[0] = sequence[0] * sequence[1];
    return is_possible_1(total, &new_sequence);
}

fn is_possible_2(total: u64, sequence: &[u64]) -> bool {
    if sequence.len() == 1 {
        return total == sequence[0];
    }
    if total < sequence[0] {
        return false;
    }
    let mut new_sequence = sequence[1..].iter().copied().collect::<Vec<_>>();
    new_sequence[0] = sequence[0] + sequence[1];
    if is_possible_2(total, &new_sequence) {
        return true;
    }
    new_sequence[0] = sequence[0] * sequence[1];
    if is_possible_2(total, &new_sequence) {
        return true;
    }
    let mut multiplier = 1;
    while sequence[1] / multiplier > 0 {
        multiplier *= 10;
    }
    new_sequence[0] = sequence[0] * multiplier + sequence[1];

    is_possible_2(total, &new_sequence)
}

fn run(input: &str) {
    let input = input
        .lines()
        .map(|line| {
            let (total_input, sequence_input) = line.split_once(": ").unwrap();
            let total: u64 = total_input.parse().unwrap();
            let sequence: Vec<u64> = sequence_input
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();

            (total, sequence)
        })
        .collect::<Vec<_>>();

    let total_calibration_result: u64 = input
        .iter()
        .filter(|(total, sequence)| is_possible_1(*total, sequence))
        .map(|(total, _)| total)
        .sum();

    println!("Part 1: {total_calibration_result}");

    let total_calibration_result: u64 = input
        .iter()
        .filter(|(total, sequence)| is_possible_2(*total, sequence))
        .map(|(total, _)| total)
        .sum();

    println!("Part 2: {total_calibration_result}");
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    run(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Run with cargo test -- --nocapture
    #[test]
    fn sample_input_1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        run(&input);
    }
}
