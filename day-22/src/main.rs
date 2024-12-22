use std::{collections::BTreeMap, fs};

use itertools::Itertools;

fn prng(mut secret: u128) -> u128 {
    secret ^= secret << 6;
    secret %= 16777216;
    secret ^= secret >> 5;
    secret %= 16777216;
    secret ^= secret << 11;
    secret %= 16777216;
    secret
}

fn run(input: &str) {
    let initial_secret_numbers = input
        .lines()
        .map(|n| n.parse::<u128>().unwrap())
        .collect::<Vec<_>>();

    let secret_sum = initial_secret_numbers
        .iter()
        .map(|secret_start| {
            let mut secret = *secret_start;
            for _ in 0..2000 {
                secret = prng(secret);
            }
            secret
        })
        .sum::<u128>();
    println!("Secret sum: {secret_sum}");

    let mut lookup = BTreeMap::<(i8, i8, i8, i8), Vec<Option<u128>>>::new();
    for (i, secret_start) in initial_secret_numbers.iter().enumerate() {
        let mut secret = *secret_start;

        (0..2001)
            .map(|_| {
                let a = (secret % 10) as i8;
                secret = prng(secret);
                a
            })
            .tuple_windows()
            .map(|(a, b)| (b, b - a))
            .tuple_windows()
            .for_each(|((_, a), (_, b), (_, c), (bananas, d))| {
                let entry = lookup
                    .entry((a, b, c, d))
                    .or_insert(vec![None; initial_secret_numbers.len()]);
                if entry[i].is_none() {
                    entry[i] = Some(bananas as u128);
                }
            });
    }

    let most_bananas = lookup
        .values()
        .map(|v| v.into_iter().filter_map(|a| *a).sum::<u128>())
        .max()
        .unwrap();
    println!("Most bananas possible: {most_bananas}");
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
        let input = "1
10
100
2024";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "123";

        run(&input);
    }

    #[test]
    fn sample_input_3() {
        let input = "1
2
3
2024";

        run(&input);
    }

    #[test]
    fn custom_test() {
        let mut secret = 123u128;
        let seq = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for n in seq {
            secret = prng(secret);
            assert_eq!(n, secret);
        }
    }
}
