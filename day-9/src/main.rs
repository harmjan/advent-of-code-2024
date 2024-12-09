use std::{cell::RefCell, fs, iter::repeat};

use itertools::Itertools;

#[derive(Debug)]
struct File {
    id: usize,
    length: usize,
    free_space: usize,
}

fn run(input: &str) {
    let files = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let length = chunk.next().unwrap();
            let free_space = chunk.next().unwrap_or(0);
            (length, free_space)
        })
        .enumerate()
        .map(|(id, (length, free_space))| File {
            id,
            length,
            free_space,
        })
        .collect::<Vec<_>>();

    let total_length = files.iter().map(|file| file.length).sum::<usize>();

    let mut reverse_file_it = files
        .iter()
        .rev()
        .flat_map(|file| repeat(file.id).take(file.length));

    let checksum = files
        .iter()
        .flat_map(|file| {
            repeat(file.id).take(file.length).chain(
                reverse_file_it
                    .by_ref()
                    .take(file.free_space)
                    .collect::<Vec<_>>()
                    .into_iter(),
            )
        })
        .take(total_length)
        .enumerate()
        .map(|(position, file_id)| position * file_id)
        .sum::<usize>();

    println!("Checksum part 1: {}", checksum);

    let placed_files = {
        let mut v = Vec::<bool>::new();
        v.resize(files.len(), false);
        RefCell::new(v)
    };

    let checksum = files
        .iter()
        .flat_map(|file| {
            let value = if placed_files.borrow()[file.id] {
                0
            } else {
                placed_files.borrow_mut()[file.id] = true;
                file.id
            };
            let iterator = repeat(value).take(file.length);

            let (free_space, file_it) = files
                .iter()
                .rev()
                .filter(|file| !placed_files.borrow()[file.id])
                .fold(
                    (file.free_space, Vec::new()),
                    |(free_space, mut acc), file| {
                        if free_space >= file.length {
                            acc.push(repeat(file.id).take(file.length));
                            placed_files.borrow_mut()[file.id] = true;
                            (free_space - file.length, acc)
                        } else {
                            (free_space, acc)
                        }
                    },
                );

            iterator.chain(
                file_it
                    .into_iter()
                    .flatten()
                    .chain(repeat(0).take(free_space)),
            )
        })
        .enumerate()
        .map(|(position, file_id)| position * file_id)
        .sum::<usize>();

    println!("Checksum part 2: {}", checksum);
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
        let input = "2333133121414131402";

        run(&input);
    }
}
