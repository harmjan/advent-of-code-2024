use core::f64;
use std::fs;

use itertools::Itertools;

#[derive(Debug)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

impl Machine {
    fn solve(&self) -> Option<u64> {
        // aa*x = y
        // ab*x + bb = y
        let aa = self.a.1 as f64 / self.a.0 as f64;
        let ab = self.b.1 as f64 / self.b.0 as f64;
        let ap = self.prize.1 as f64 / self.prize.0 as f64;
        let bb = self.prize.1 as f64 - ab * self.prize.0 as f64;
        let x_intersection = bb / (aa - ab);
        let a_presses = (x_intersection / self.a.0 as f64) as u64;
        let b_presses = ((self.prize.0 as f64 - x_intersection) / self.b.0 as f64) as u64;

        assert!((aa - ab).abs() > f64::EPSILON || (aa - ap).abs() > f64::EPSILON);

        for a in a_presses.saturating_sub(2)..(a_presses + 2) {
            for b in b_presses.saturating_sub(2)..(b_presses + 2) {
                if self.a.0 * a + self.b.0 * b == self.prize.0
                    && self.a.1 * a + self.b.1 * b == self.prize.1
                {
                    return Some(a * 3 + b);
                }
            }
        }

        None
    }
}

fn run(input: &str) {
    let machines = input
        .split(|c| [' ', '\n', '+', '=', ','].contains(&c))
        .filter_map(|n| n.parse::<u64>().ok())
        .chunks(6)
        .into_iter()
        .map(|mut chunk_it| {
            let (ay, ax, by, bx, py, px) = chunk_it.next_tuple().unwrap();
            Machine {
                a: (ay, ax),
                b: (by, bx),
                prize: (py, px),
            }
        })
        .collect::<Vec<_>>();

    let tokens = machines.iter().filter_map(|m| m.solve()).sum::<u64>();
    println!("Tokens part 1: {tokens}");

    let tokens = machines
        .into_iter()
        .map(|m| Machine {
            a: m.a,
            b: m.b,
            prize: (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000),
        })
        .filter_map(|m| m.solve())
        .sum::<u64>();
    println!("Tokens part 2: {tokens}");
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
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        run(&input);
    }

    #[test]
    fn custom() {
        let input = "Button A: X+1, Y+1
Button B: X+2, Y+2
Prize: X=11, Y=11";

        run(&input);
    }
}
