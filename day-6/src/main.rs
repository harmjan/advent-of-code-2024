use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn print(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{c}"));
        print!("\n");
    });
    print!("\n");
}

fn simulate(grid: &mut Vec<Vec<char>>, start: (usize, usize)) -> Option<()> {
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
            if grid
                .get(new_position.0 as usize)?
                .get(new_position.1 as usize)?
                == &'#'
            {
                dir = match dir {
                    Direction::UP => Direction::RIGHT,
                    Direction::RIGHT => Direction::DOWN,
                    Direction::DOWN => Direction::LEFT,
                    Direction::LEFT => Direction::UP,
                };
            } else {
                grid[position.0 as usize][position.1 as usize] = 'X';
                position = new_position;
                break;
            }
        }
        if prev_positions.contains(&(position, dir)) {
            return Some(());
        }
    }
}

fn run(input: &str) {
    let original_grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let start_position = original_grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (y, x, c)))
        .filter(|(_, _, c)| **c == '^')
        .map(|(y, x, _)| (y, x))
        .next()
        .unwrap();

    let mut grid = original_grid.clone();
    simulate(&mut grid, start_position);
    let visited = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|c| **c == 'X')
        .count()
        + 1;

    println!("Visited: {visited}");

    let ways = (0..grid.len())
        .flat_map(|y| (0..grid[0].len()).map(move |x| (y, x)))
        .filter(|(y, x)| !['#', '^'].contains(&grid[*y][*x]))
        .filter(|(y, x)| {
            let mut tmp_grid = original_grid.clone();
            tmp_grid[*y][*x] = '#';

            simulate(&mut tmp_grid, start_position).is_some()
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
