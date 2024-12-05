use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn check(update: &Vec<u32>, page_order_rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut prev_pages = HashSet::<u32>::new();

    for page in update.iter() {
        if let Some(order_rules) = page_order_rules.get(page) {
            if prev_pages.intersection(order_rules).next() != None {
                return false;
            }
        }
        prev_pages.insert(*page);
    }

    true
}

fn run(input: &str) {
    let (page_order_rules_input, updates_input) = {
        let mut input_it = input.split("\n\n");
        (input_it.next().unwrap(), input_it.next().unwrap())
    };

    let page_order_rules: HashMap<u32, HashSet<u32>> = page_order_rules_input
        .lines()
        .map(|line| {
            line.split('|')
                .map(|n| n.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .fold(HashMap::<u32, HashSet<u32>>::new(), |mut acc, (a, b)| {
            if let Some(s) = acc.get_mut(&a) {
                s.insert(b);
            } else {
                let mut s = HashSet::new();
                s.insert(b);
                acc.insert(a, s);
            };
            acc
        });

    let updates = updates_input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let valid_update_middle_page_number_sum: u32 = updates
        .iter()
        .filter(|update| check(update, &page_order_rules))
        .map(|update| update[update.len() / 2])
        .sum();

    println!("Part 1: {valid_update_middle_page_number_sum}");

    let fixed_update_middle_page_number_sum: u32 = updates
        .iter()
        .filter(|update| !check(update, &page_order_rules))
        .map(|update| {
            let page_set = update.iter().copied().collect::<HashSet<_>>();

            update
                .iter()
                .map(|page| {
                    (
                        page,
                        page_order_rules
                            .get(page)
                            .map(|order_rules| order_rules.intersection(&page_set).count())
                            .unwrap_or(0),
                    )
                })
                .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
                .map(|(page, _)| page)
                .collect::<Vec<_>>()
        })
        .map(|update| update[update.len() / 2])
        .sum();

    println!("Part 2: {fixed_update_middle_page_number_sum}");
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
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        run(&input);
    }
}
