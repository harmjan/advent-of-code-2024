use core::panic;
use std::fs;

use itertools::{repeat_n, Itertools};

/*
<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
  v <<   A >>  ^ A   <   A > A  v  A   <  ^ AA > A   < v  AAA >  ^ A
         <       A       ^   A     >        ^^   A        vvv      A
                 0           2                   9                 A
*/

fn keypad_to_coordinates(c: char) -> (usize, usize) {
    match c {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => panic!(),
    }
}

fn directional_to_coordinates(c: char) -> (usize, usize) {
    match c {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => panic!(),
    }
}

fn directional_sequence(sequence: &Vec<char>, level: usize) -> Vec<char> {
    let mut position = directional_to_coordinates('A');
    sequence
        .iter()
        .flat_map(|c| {
            let target_position = directional_to_coordinates(*c);

            let dy = target_position.0 as isize - position.0 as isize;
            let dx = target_position.1 as isize - position.1 as isize;

            let x_button = if dx >= 0 { '>' } else { '<' };
            let y_button = if dy >= 0 { 'v' } else { '^' };

            let path = repeat_n(x_button, dx.abs() as usize)
                .chain(repeat_n(y_button, dy.abs() as usize))
                .permutations((dx.abs() + dy.abs()) as usize)
                .unique()
                .filter(|path| {
                    let mut position = position;
                    !path
                        .iter()
                        .map(|c| {
                            position = match c {
                                '<' => (position.0, position.1 - 1),
                                '>' => (position.0, position.1 + 1),
                                '^' => (position.0 - 1, position.1),
                                'v' => (position.0 + 1, position.1),
                                _ => panic!(),
                            };
                            position
                        })
                        .contains(&(0, 0))
                })
                .map(|mut path| {
                    path.push('A');
                    if level == 0 {
                        path
                    } else {
                        directional_sequence(&path, level - 1)
                    }
                })
                .min_by(|a, b| a.len().cmp(&b.len()))
                .unwrap();

            position = target_position;

            path.into_iter()
        })
        .collect()
}

fn keypad_sequence(sequence: &Vec<char>, directional_level: usize) -> Vec<char> {
    let mut position = keypad_to_coordinates('A');
    sequence
        .iter()
        .flat_map(|c| {
            let target_position = keypad_to_coordinates(*c);

            let dy = target_position.0 as isize - position.0 as isize;
            let dx = target_position.1 as isize - position.1 as isize;

            let x_button = if dx >= 0 { '>' } else { '<' };
            let y_button = if dy >= 0 { 'v' } else { '^' };

            let path = repeat_n(x_button, dx.abs() as usize)
                .chain(repeat_n(y_button, dy.abs() as usize))
                .permutations((dx.abs() + dy.abs()) as usize)
                .unique()
                .filter(|path| {
                    let mut position = position;
                    !path
                        .iter()
                        .map(|c| {
                            position = match c {
                                '<' => (position.0, position.1 - 1),
                                '>' => (position.0, position.1 + 1),
                                '^' => (position.0 - 1, position.1),
                                'v' => (position.0 + 1, position.1),
                                _ => panic!(),
                            };
                            position
                        })
                        .contains(&(3, 0))
                })
                .map(|mut path| {
                    path.push('A');
                    directional_sequence(&path, directional_level)
                })
                .min_by(|a, b| a.len().cmp(&b.len()))
                .unwrap();

            position = target_position;

            path.into_iter()
        })
        .collect()
}

fn run(input: &str) {
    let codes = input
        .lines()
        .map(|code| code.chars().map(|c| c).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let complexity_sum = codes
        .iter()
        .map(|code| {
            let my_seq = keypad_sequence(code, 1);
            let numeric_part_code = code[..(code.len() - 1)]
                .into_iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            my_seq.len() * numeric_part_code
        })
        .sum::<usize>();
    println!("Sum of complexity codes first historian: {complexity_sum}");
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
        let input = "029A
980A
179A
456A
379A";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "029A";

        run(&input);
    }
}
