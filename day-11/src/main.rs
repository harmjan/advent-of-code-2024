use std::{collections::HashMap, fs};

fn amount_of_stones(
    stone_number: u64,
    steps: usize,
    cache: &mut HashMap<(u64, usize), u64>,
) -> u64 {
    if let Some(result) = cache.get(&(stone_number, steps)) {
        return *result;
    }

    let num_stones = if steps == 0 {
        1
    } else {
        if stone_number == 0 {
            amount_of_stones(1, steps - 1, cache)
        } else {
            let n_str = stone_number.to_string();
            if n_str.len() % 2 == 0 {
                amount_of_stones(
                    n_str[0..n_str.len() / 2].parse::<u64>().unwrap(),
                    steps - 1,
                    cache,
                ) + amount_of_stones(
                    n_str[n_str.len() / 2..].parse::<u64>().unwrap(),
                    steps - 1,
                    cache,
                )
            } else {
                amount_of_stones(stone_number * 2024, steps - 1, cache)
            }
        }
    };

    cache.insert((stone_number, steps), num_stones);

    num_stones
}

fn run(input: &str) {
    let stones = input
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut cache = HashMap::<(u64, usize), u64>::new();
    let amount_of_stones_part_1 = stones
        .iter()
        .map(|stone| amount_of_stones(*stone, 25, &mut cache))
        .sum::<u64>();
    println!("Amount of stones part 1: {amount_of_stones_part_1}");

    let amount_of_stones_part_2 = stones
        .iter()
        .map(|stone| amount_of_stones(*stone, 75, &mut cache))
        .sum::<u64>();
    println!("Amount of stones part 2: {amount_of_stones_part_2}");
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
        let input = "125 17";

        run(&input);
    }
}
