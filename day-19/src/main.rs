use std::fs;

fn is_possible(towels: &Vec<&str>, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }

    for towel in towels.iter() {
        if pattern.starts_with(*towel) {
            if is_possible(towels, &pattern[towel.len()..]) {
                return true;
            }
        }
    }

    false
}

fn ways_possible(towels: &Vec<&str>, pattern: &str, cache: &mut Vec<Option<usize>>) -> usize {
    if let Some(cached_value) = cache[pattern.len()] {
        cached_value
    } else {
        let sum = towels
            .iter()
            .map(|towel| {
                if towel.len() > pattern.len() {
                    0
                } else if pattern.starts_with(*towel) {
                    ways_possible(towels, &pattern[towel.len()..], cache)
                } else {
                    0
                }
            })
            .sum();
        cache[pattern.len()] = Some(sum);
        sum
    }
}

fn run(input: &str) {
    let (towels_input, pattern_input) = input.split_once("\n\n").unwrap();
    let towels = towels_input.split(", ").collect::<Vec<_>>();
    let patterns = pattern_input.lines().collect::<Vec<_>>();

    let possible = patterns
        .iter()
        .filter(|pattern| is_possible(&towels, pattern))
        .count();
    println!("Amount possible: {possible}");

    let possible = patterns
        .iter()
        .map(|pattern| {
            let mut cache = vec![None; pattern.len() + 1];
            cache[0] = Some(1);
            ways_possible(&towels, pattern, &mut cache)
        })
        .sum::<usize>();
    println!("Ways possible: {possible}");
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
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        run(&input);
    }
}
