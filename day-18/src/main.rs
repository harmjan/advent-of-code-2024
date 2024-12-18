use std::{collections::VecDeque, fs};

#[derive(Debug)]
struct State {
    cost: usize,
    position: (usize, usize),
}

fn bfs(grid: &Vec<Vec<bool>>) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut cost = vec![vec![None; grid.len()]; grid.len()];
    queue.push_back(State {
        cost: 0,
        position: (0, 0),
    });
    loop {
        let current = match queue.pop_front() {
            Some(s) => s,
            None => break,
        };

        if cost[current.position.0][current.position.1].is_some() {
            continue;
        }
        cost[current.position.0][current.position.1] = Some(current.cost);

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let new_y = current.position.0 as isize + dy;
            let new_x = current.position.1 as isize + dx;
            if new_y < 0
                || new_y >= grid.len() as isize
                || new_x < 0
                || new_x >= grid.len() as isize
            {
                continue;
            }

            let new_y = new_y as usize;
            let new_x = new_x as usize;

            if grid[new_y][new_x] {
                queue.push_back(State {
                    cost: current.cost + 1,
                    position: (new_y, new_x),
                });
            }
        }
    }
    cost[grid.len() - 1][grid.len() - 1]
}

fn run(input: &str) {
    let grid_size = 71;
    let part_1_limit = 1024;
    //let grid_size = 7;
    //let part_1_limit = 12;

    let byte_locations = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut grid = vec![vec![true; grid_size]; grid_size];
    byte_locations.iter().take(part_1_limit).for_each(|(x, y)| {
        grid[*y][*x] = false;
    });
    let min_path_cost_after_part_1 = bfs(&grid).unwrap();

    println!(
        "Cost to go to {},{} = {}",
        grid_size - 1,
        grid_size - 1,
        min_path_cost_after_part_1
    );

    let mut grid = vec![vec![true; grid_size]; grid_size];
    let first_blocked = byte_locations
        .iter()
        .filter_map(|(x, y)| {
            grid[*y][*x] = false;
            match bfs(&grid) {
                Some(_) => None,
                None => Some((*x, *y)),
            }
        })
        .next()
        .unwrap();
    println!("First blocked at {},{}", first_blocked.0, first_blocked.1);
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
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

        run(&input);
    }
}
