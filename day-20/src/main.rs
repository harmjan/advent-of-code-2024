use std::fs;

fn find_character(grid: &Vec<Vec<char>>, target: char) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .filter_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, c)| if *c == target { Some((y, x)) } else { None })
                .next()
        })
        .next()
        .unwrap()
}

fn run(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = find_character(&grid, 'S');

    let mut cost = vec![vec![None; grid[0].len()]; grid.len()];
    let mut position = start;
    let mut step = 0;
    loop {
        cost[position.0][position.1] = Some(step);
        step += 1;

        let mut new_position = None;
        for (dy, dx) in [(-1isize, 0), (0, 1), (1, 0), (0, -1)] {
            let ny = (position.0 as isize + dy) as usize;
            let nx = (position.1 as isize + dx) as usize;
            if grid[ny][nx] != '#' && cost[ny][nx] == None {
                new_position = Some((ny, nx));
                break;
            }
        }
        match new_position {
            Some(new_pos) => position = new_pos,
            None => break,
        }
    }

    let at_least_100 = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (y, x, *c)))
        .filter(|(y, x, _)| *y > 0 && *x > 0 && *y < grid.len() - 1 && *x < grid.len() - 1)
        .filter(|(_, _, c)| *c == '#')
        .filter_map(|(y, x, _)| {
            let neighbour_costs = [(-1isize, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .filter_map(|(dy, dx)| {
                    let ny = (y as isize + dy) as usize;
                    let nx = (x as isize + dx) as usize;
                    cost[ny][nx]
                })
                .collect::<Vec<_>>();

            if neighbour_costs.len() >= 2 {
                Some(
                    *neighbour_costs.iter().max().unwrap()
                        - *neighbour_costs.iter().min().unwrap()
                        - 2,
                )
            } else {
                None
            }
        })
        .filter(|n| *n >= 100)
        .count();

    println!("Save at least 100 picoseconds with 2 picosecond cheat: {at_least_100}");

    let at_least_100 = cost
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (y, x, *c)))
        .filter_map(|(y, x, c)| c.map(|c| (y, x, c)))
        .flat_map(|(start_y, start_x, start_cost)| {
            let cost = &cost;
            (2..21)
                .flat_map(|steps| (0..(steps + 1)).map(move |dy| (dy, steps - dy)))
                .flat_map(move |(dy, dx)| {
                    [(1, 1), (1, -1), (-1, 1), (-1, -1)]
                        .iter()
                        .filter_map(move |(cy, cx)| {
                            let end_y = start_y as isize + (dy * cy);
                            let end_x = start_x as isize + (dx * cx);
                            if (dy == 0 && *cy == -1)
                                || (dx == 0 && *cx == -1)
                                || end_y < 0
                                || end_x < 0
                                || end_y >= cost.len() as isize
                                || end_x >= cost.len() as isize
                            {
                                None
                            } else if let Some(end_cost) = cost[end_y as usize][end_x as usize] {
                                if end_cost < start_cost {
                                    None
                                } else {
                                    Some(end_cost - start_cost - dx - dy)
                                }
                            } else {
                                None
                            }
                        })
                })
        })
        .filter(|n| *n >= 100)
        .count();

    println!("Save at least 100 picoseconds with 20 picosecond cheat: {at_least_100}");
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
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        run(&input);
    }
}
