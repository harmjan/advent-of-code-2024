use std::{
    collections::{BTreeMap, VecDeque},
    fs,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug)]
enum Operation {
    AND,
    OR,
    XOR,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Operation::AND),
            "OR" => Ok(Operation::OR),
            "XOR" => Ok(Operation::XOR),
            _ => Err(()),
        }
    }
}

impl Operation {
    fn execute(&self, a: bool, b: bool) -> bool {
        match self {
            Operation::OR => a || b,
            Operation::AND => a && b,
            Operation::XOR => a ^ b,
        }
    }
}

#[derive(Debug)]
struct Gate {
    in_a: String,
    in_b: String,
    out: String,
    operation: Operation,
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, op, right, _, res) = line.split_ascii_whitespace().collect_tuple().unwrap();

        Ok(Gate {
            in_a: String::from(left),
            in_b: String::from(right),
            out: String::from(res),
            operation: op.parse()?,
        })
    }
}

fn simulate_circuit(
    gates_from_input: &BTreeMap<String, Vec<&Gate>>,
    mut wire_values: BTreeMap<String, bool>,
) -> BTreeMap<String, bool> {
    let mut wire_queue = VecDeque::from_iter(
        wire_values
            .iter()
            .map(|(wire, value)| (wire.clone(), *value)),
    );

    while let Some((wire, value)) = wire_queue.pop_front() {
        if let Some(gates) = gates_from_input.get(&wire) {
            for gate in gates {
                let other_wire = if gate.in_a == wire {
                    gate.in_b.clone()
                } else {
                    gate.in_a.clone()
                };
                if let Some(other_value) = wire_values.get(&other_wire) {
                    let operation_result = gate.operation.execute(value, *other_value);
                    wire_values.insert(gate.out.clone(), operation_result);
                    wire_queue.push_back((gate.out.clone(), operation_result));
                }
            }
        }
    }

    wire_values
}

fn extract_number_from_wires(wire_values: &BTreeMap<String, bool>, prefix: char) -> u128 {
    wire_values
        .iter()
        .filter(|(wire, _)| wire.chars().next().unwrap() == prefix)
        .rev()
        .fold(0u128, |acc, (_, n)| (acc << 1) | (if *n { 1 } else { 0 }))
}

fn run(input: &str) {
    let (input_static, input_gates) = input.split_once("\n\n").unwrap();

    let initial_values = input_static
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(a, b)| (String::from(a), b == "1"))
        .collect::<BTreeMap<_, _>>();

    let gates = input_gates
        .lines()
        .map(|line| line.parse::<Gate>().unwrap())
        .collect_vec();

    let gates_from_input = gates
        .iter()
        .flat_map(|gate| [(gate.in_a.clone(), gate), (gate.in_b.clone(), gate)])
        .fold(
            BTreeMap::new(),
            |mut acc: BTreeMap<String, Vec<_>>, (wire, gate)| {
                acc.entry(wire).or_default().push(gate);
                acc
            },
        );

    let final_wire_values = simulate_circuit(&gates_from_input, initial_values);
    println!(
        "Z wire sum: {}",
        extract_number_from_wires(&final_wire_values, 'z')
    );
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
        let input = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

        run(&input);
    }
}
