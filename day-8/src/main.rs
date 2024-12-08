use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

use itertools::Itertools;

fn print_grid(grid: &Vec<Vec<char>>, antinodes: &HashSet<(isize, isize)>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if antinodes.contains(&(y as isize, x as isize)) {
                print!("#");
            } else {
                let a = grid[y][x];
                print!("{a}");
            }
        }
        print!("\n");
    }
}

fn run(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let antennas = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (y, x, c)))
        .filter(|(_, _, c)| **c != '.')
        .fold(
            HashMap::<char, Vec<(usize, usize)>>::new(),
            |mut acc, (y, x, c)| {
                acc.entry(*c).or_default().push((y, x));
                acc
            },
        );

    let antinodes = antennas
        .values()
        .flat_map(|antenna_positions| {
            antenna_positions
                .iter()
                .tuple_combinations()
                .flat_map(|((ya, xa), (yb, xb))| {
                    let xdiff = (*xa as isize) - (*xb as isize);
                    let ydiff = (*ya as isize) - (*yb as isize);

                    let antinode_a = ((*ya as isize) + ydiff, (*xa as isize) + xdiff);
                    let antinode_b = ((*yb as isize) - ydiff, (*xb as isize) - xdiff);

                    vec![antinode_a, antinode_b]
                })
        })
        .filter(|(y, x)| {
            0 <= *x && 0 <= *y && *x < grid[0].len() as isize && *y < grid.len() as isize
        })
        .collect::<HashSet<_>>();

    let amount_of_antinodes = antinodes.len();
    println!("Part 1: {amount_of_antinodes}");

    let antinodes = antennas
        .values()
        .flat_map(|antenna_positions| {
            antenna_positions
                .iter()
                .tuple_combinations()
                .flat_map(|((ya, xa), (yb, xb))| {
                    let xdiff = (*xa as isize) - (*xb as isize);
                    let ydiff = (*ya as isize) - (*yb as isize);

                    (0..grid.len()).flat_map(move |i| {
                        vec![
                            (
                                (*ya as isize) + (i as isize * ydiff),
                                (*xa as isize) + (i as isize * xdiff),
                            ),
                            (
                                (*yb as isize) - (i as isize * ydiff),
                                (*xb as isize) - (i as isize * xdiff),
                            ),
                        ]
                    })
                })
        })
        .filter(|(y, x)| {
            0 <= *x && 0 <= *y && *x < grid[0].len() as isize && *y < grid.len() as isize
        })
        .collect::<HashSet<_>>();

    print_grid(&grid, &antinodes);

    let amount_of_antinodes = antinodes.len();
    println!("Part 2: {amount_of_antinodes}");
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
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

        run(&input);
    }
}
