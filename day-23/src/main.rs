use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

use itertools::Itertools;

fn max_clique<'a>(
    connections: &HashMap<&'a str, BTreeSet<&'a str>>,
    in_clique: BTreeSet<&'a str>,
    mut possible_clique: BTreeSet<&'a str>,
) -> BTreeSet<&'a str> {
    if possible_clique.is_empty() {
        return in_clique;
    }

    let mut largest_clique = BTreeSet::new();
    while let Some(node) = possible_clique.pop_first() {
        let node_set = max_clique(
            connections,
            in_clique
                .union(&BTreeSet::from_iter([node]))
                .copied()
                .collect(),
            possible_clique
                .intersection(&connections[node])
                .copied()
                .collect(),
        );

        largest_clique = if node_set.len() > largest_clique.len() {
            node_set
        } else {
            largest_clique
        };
    }

    largest_clique
}

fn run(input: &str) {
    let connections = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .fold(HashMap::<&str, BTreeSet<&str>>::new(), |mut acc, (a, b)| {
            acc.entry(a).or_default().insert(b);
            acc
        });

    let interconnected_computers = connections
        .iter()
        .filter(|(computer, _)| computer.chars().next().unwrap() == 't')
        .flat_map(|(computer_a, local_connections)| {
            local_connections
                .iter()
                .permutations(2)
                .filter(|computers| connections[computers[0]].contains(computers[1]))
                .map(|mut computers| {
                    computers.push(computer_a);
                    computers.sort();
                    (*computers[0], *computers[1], *computers[2])
                })
        })
        .unique()
        .count();
    println!("Interconnected computers: {interconnected_computers}");

    println!("Amount of computers: {}", connections.len());

    let mut banned_set = BTreeSet::new();
    let network = connections
        .iter()
        .map(|(computer, local_connections)| {
            let max_c = max_clique(
                &connections,
                BTreeSet::from_iter([*computer]),
                local_connections.difference(&banned_set).copied().collect(),
            );
            banned_set.insert(computer);
            max_c
        })
        .max_by_key(|network| network.len())
        .unwrap()
        .into_iter()
        .join(",");

    println!("Network {network}");
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
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "ka-co
ta-co
de-co
ta-ka
de-ta
ka-de
ja-co
ha-rm
ha-ja";

        run(&input);
    }
}
