use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
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

impl Gate {
    fn other_input(&self, input: &String) -> &String {
        if *input == *self.in_a {
            &self.in_b
        } else {
            &self.in_a
        }
    }
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

fn create_input_map(gates: &Vec<Gate>) -> BTreeMap<String, Vec<usize>> {
    gates
        .iter()
        .enumerate()
        .flat_map(|(i, gate)| [(gate.in_a.clone(), i), (gate.in_b.clone(), i)])
        .fold(
            BTreeMap::new(),
            |mut acc: BTreeMap<String, Vec<_>>, (wire, gate)| {
                acc.entry(wire).or_default().push(gate);
                acc
            },
        )
}

fn create_output_map(gates: &Vec<Gate>) -> BTreeMap<String, usize> {
    gates
        .iter()
        .enumerate()
        .map(|(i, gate)| (gate.out.clone(), i))
        .collect()
}

fn simulate_circuit(
    gates: &Vec<Gate>,
    mut wire_values: BTreeMap<String, bool>,
) -> BTreeMap<String, bool> {
    let gates_from_input = create_input_map(&gates);

    let mut wire_queue = VecDeque::from_iter(
        wire_values
            .iter()
            .map(|(wire, value)| (wire.clone(), *value)),
    );

    while let Some((wire, value)) = wire_queue.pop_front() {
        if let Some(gates_indices) = gates_from_input.get(&wire) {
            for gate_index in gates_indices {
                let gate = &gates[*gate_index];
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

/*
z00 = x00 ^ z00
c00 = x00 & y00

for n in 1..45 {
    a_n = x_n ^ y_n
    b_n = x_n & y_n

    d_n = a_n & c_(n-1)
    z_n = a_n ^ c_(n-1)

    c_n = d_n | b_n
}

z45 = c44
*/

#[derive(Debug, Copy, Clone)]
enum Alias {
    A(usize),
    B(usize),
    C(usize),
    D(usize),
    Z(usize),
}

fn part2(gates: &Vec<Gate>) -> BTreeSet<String> {
    let gates_from_input = create_input_map(&gates);
    let gate_from_output = create_output_map(&gates);

    let mut swaps = BTreeSet::new();
    let mut gate_alias_map = vec![None; gates.len()];

    // Mark all a and b gates
    for i in 0..45 {
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);

        let x_gates = &gates_from_input[&x];

        for gate_index in x_gates {
            let gate = &gates[*gate_index];
            assert_eq!(gate.other_input(&x), &y);

            gate_alias_map[*gate_index] = Some(match gate.operation {
                Operation::XOR => Alias::A(i),
                Operation::AND => Alias::B(i),
                Operation::OR => panic!(),
            });
        }
    }

    // Mark z outputs not bound to XOR
    for i in 1..45 {
        let z = format!("z{:02}", i);
        let z_gate_index = gate_from_output.get(&z).unwrap();
        let z_gate = &gates[*z_gate_index];
        if z_gate.operation != Operation::XOR {
            swaps.insert(z.clone());
        } else {
            gate_alias_map[*z_gate_index] = Some(Alias::Z(i));
        }
    }

    // Mark all wires attached to a XOR that isn't marked as an alias yet
    for (i, gate) in gates.iter().enumerate() {
        if gate.operation == Operation::XOR && gate_alias_map[i].is_none() {
            swaps.insert(gate.out.clone());
        }
    }

    // Mark all wires out of an A XOR that isn't a net attached an AND and XOR a swap
    for (i, gate) in gates.iter().enumerate() {
        if gate.operation == Operation::OR
            || (gate.operation == Operation::XOR && matches!(gate_alias_map[i], Some(Alias::A(_))))
        {
            if let Some(net_gates_indices) = gates_from_input.get(&gate.out) {
                if net_gates_indices.len() != 2 {
                    swaps.insert(gate.out.clone());
                }
            } else {
                swaps.insert(gate.out.clone());
            }
        }

        if gate.operation == Operation::AND {
            if let Some(net_gates_indices) = gates_from_input.get(&gate.out) {
                if net_gates_indices.len() != 1 {
                    swaps.insert(gate.out.clone());
                }
            }
        }
    }

    // I manually confirmed these are actually correct and part of the half adder or last carry
    // digit
    swaps.remove("z00");
    swaps.remove("z45");
    swaps.remove("nqp");

    swaps
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

    let final_wire_values = simulate_circuit(&gates, initial_values);
    println!(
        "Z wire sum: {}",
        extract_number_from_wires(&final_wire_values, 'z')
    );

    {
        let gates_per_half_adder = 2;
        let gates_per_full_adder = 5;
        assert_eq!(
            gates.len(),
            gates_per_half_adder + 44 * gates_per_full_adder
        );

        let xor_per_half_adder = 1;
        let and_per_half_adder = 1;
        let or_per_half_adder = 0;

        let xor_per_full_adder = 2;
        let and_per_full_adder = 2;
        let or_per_full_adder = 1;

        assert_eq!(
            gates
                .iter()
                .filter(|gate| gate.operation == Operation::XOR)
                .count(),
            xor_per_half_adder + 44 * xor_per_full_adder
        );
        assert_eq!(
            gates
                .iter()
                .filter(|gate| gate.operation == Operation::AND)
                .count(),
            and_per_half_adder + 44 * and_per_full_adder
        );
        assert_eq!(
            gates
                .iter()
                .filter(|gate| gate.operation == Operation::OR)
                .count(),
            or_per_half_adder + 44 * or_per_full_adder
        );
    }

    let answer2 = part2(&gates).iter().join(",");
    println!("{}", answer2);
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
