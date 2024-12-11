use std::{collections::VecDeque, fs};

fn print_grid(grid: &Vec<Vec<u32>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let a = grid[y][x];
            print!(" {a}");
        }
        print!("\n");
    }
}

fn bfs(grid: &Vec<Vec<u32>>, start: (usize, usize)) -> usize {
    let mut reachable = vec![vec![false; grid[0].len()]; grid.len()];
    let mut trails = 0;

    let mut queue = vec![start];
    reachable[start.0][start.1] = true;
    while queue.len() > 0 {
        let (y, x) = queue.pop().unwrap();
        if grid[y][x] == 9 {
            trails += 1;
        }
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (ny, nx) = (y as i32 + dy, x as i32 + dx);
            if ny < 0 || ny >= grid.len() as i32 || nx < 0 || nx >= grid[0].len() as i32 {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);
            if reachable[ny][nx] {
                continue;
            }
            if grid[ny][nx] == grid[y][x] + 1 {
                reachable[ny][nx] = true;
                queue.push((ny, nx));
            }
        }
    }

    trails
}

fn bfs2(grid: &Vec<Vec<u32>>, start: (usize, usize)) -> usize {
    let mut ways_reachable = vec![vec![0; grid[0].len()]; grid.len()];
    ways_reachable[start.0][start.1] = 1;

    let mut queue = VecDeque::from([start]);
    while queue.len() > 0 {
        let (y, x) = queue.pop_front().unwrap();
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (ny, nx) = (y as i32 + dy, x as i32 + dx);
            if ny < 0 || ny >= grid.len() as i32 || nx < 0 || nx >= grid[0].len() as i32 {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);
            if grid[ny][nx] == grid[y][x] + 1 {
                ways_reachable[ny][nx] += ways_reachable[y][x];
                if !queue.contains(&(ny, nx)) {
                    queue.push_back((ny, nx));
                }
            }
        }
    }

    ways_reachable
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(move |(x, _)| grid[y][*x] == 9)
                .map(|(_, c)| *c)
        })
        .sum::<u32>() as usize
}

fn run(input: &str) {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(11))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let trailhead_sum = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .map(move |(x, _)| (y, x))
        })
        .map(|(y, x)| bfs(&grid, (y, x)))
        .sum::<usize>();

    println!("Trailhead sum part 1: {trailhead_sum}");

    let trailhead_rating_sum = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .map(move |(x, _)| (y, x))
        })
        .map(|(y, x)| bfs2(&grid, (y, x)))
        .sum::<usize>();

    println!("Trailhead sum part 2: {trailhead_rating_sum}");
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
        let input = "0123
1234
8765
9876";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        run(&input);
    }

    #[test]
    fn sample_input_3() {
        let input = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";

        run(&input);
    }
}
