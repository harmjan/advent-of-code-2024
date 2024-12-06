use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

enum SimResult {
    Loop,
    Leave(usize),
}

fn simulate(grid: &Vec<Vec<char>>, start: (usize, usize)) -> SimResult {
    let start = (start.0 as isize, start.1 as isize);
    let mut position = start;
    let mut dir = Direction::UP;

    let mut prev_positions = HashSet::<((isize, isize), Direction)>::new();

    loop {
        prev_positions.insert((position, dir));
        loop {
            let new_position = match dir {
                Direction::UP => (position.0 - 1, position.1),
                Direction::RIGHT => (position.0, position.1 + 1),
                Direction::DOWN => (position.0 + 1, position.1),
                Direction::LEFT => (position.0, position.1 - 1),
            };
            match grid
                .get(new_position.0 as usize)
                .map(|row| row.get(new_position.1 as usize))
                .flatten()
            {
                None => {
                    return SimResult::Leave(
                        prev_positions
                            .into_iter()
                            .map(|((y, x), _)| (y, x))
                            .collect::<HashSet<_>>()
                            .len(),
                    )
                }
                Some(&'#') => {
                    dir = match dir {
                        Direction::UP => Direction::RIGHT,
                        Direction::RIGHT => Direction::DOWN,
                        Direction::DOWN => Direction::LEFT,
                        Direction::LEFT => Direction::UP,
                    };
                }
                Some(_) => {
                    position = new_position;
                    break;
                }
            };
        }
        if prev_positions.contains(&(position, dir)) {
            return SimResult::Loop;
        }
    }
}

fn run(input: &str) {
    let mut grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let start_position = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (y, x, c)))
        .filter(|(_, _, c)| **c == '^')
        .map(|(y, x, _)| (y, x))
        .next()
        .unwrap();

    let visited = match simulate(&grid, start_position) {
        SimResult::Leave(visited) => visited,
        SimResult::Loop => panic!("Default input shouldn't loop"),
    };

    println!("Visited: {visited}");

    let (rows, columns) = (grid.len(), grid[0].len());

    let ways = (0..rows)
        .flat_map(|y| (0..columns).map(move |x| (y, x)))
        .filter(|(y, x)| !['#', '^'].contains(&grid[*y][*x]))
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|(y, x)| {
            grid[*y][*x] = '#';

            let tmp_result = match simulate(&grid, start_position) {
                SimResult::Loop => true,
                SimResult::Leave(_) => false,
            };

            grid[*y][*x] = '.';

            tmp_result
        })
        .count();

    println!("Number of ways: {ways}");
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
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        run(&input);
    }
}
