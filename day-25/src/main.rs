use std::fs;

fn run(input: &str) {
    let (keys, locks) =
        input
            .split("\n\n")
            .fold((Vec::new(), Vec::new()), |(mut keys, mut locks), block| {
                let matrix = block.lines().collect::<Vec<_>>();
                let mut sequence = [0; 5];

                for x in 0..5 {
                    sequence[x] = (1..6)
                        .filter(|y| matrix[*y].chars().nth(x).unwrap() == '#')
                        .count();
                }

                if block.chars().next().unwrap() == '#' {
                    keys.push(sequence);
                } else {
                    locks.push(sequence);
                }

                (keys, locks)
            });

    println!("Amount of keys:  {}", keys.len());
    println!("Amount of locks: {}", locks.len());

    let lock_key_pairs = keys
        .iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| (0..5).all(|column| key[column] + lock[column] <= 5))
                .count()
        })
        .sum::<usize>();
    println!("Amount of lock/keys pairs that fit: {lock_key_pairs}");
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
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

        run(&input);
    }
}
