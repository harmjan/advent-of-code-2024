use std::{
    collections::{BinaryHeap, HashMap},
    fs,
    iter::zip,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let (list_a, list_b) = input
        .lines()
        .map(|line| {
            let mut numbers_it = line
                .split_whitespace()
                .map(|str_number| str_number.parse::<u32>().unwrap());

            (numbers_it.next().unwrap(), numbers_it.next().unwrap())
        })
        .fold(
            (BinaryHeap::<u32>::new(), BinaryHeap::<u32>::new()),
            |(mut list_a, mut list_b), (a, b)| {
                list_a.push(a);
                list_b.push(b);
                (list_a, list_b)
            },
        );

    let total_distance: u32 = zip(list_a.iter(), list_b.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("Total distance: {}", total_distance);

    let b_occurences = list_b
        .into_iter()
        .fold(HashMap::<u32, usize>::new(), |mut acc, n| {
            if let Some(entry) = acc.get_mut(&n) {
                *entry += 1;
            } else {
                acc.insert(n, 1);
            }
            acc
        });

    let similarity_score: u32 = list_a
        .into_iter()
        .map(|n| n * (*b_occurences.get(&n).unwrap_or(&0usize) as u32))
        .sum();

    println!("Similarity score: {}", similarity_score);
}
