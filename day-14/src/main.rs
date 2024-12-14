use std::{cmp::max, fs};

use itertools::Itertools;

fn print_grid(grid: &Vec<Vec<bool>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let a = if grid[y][x] { '.' } else { ' ' };
            print!("{a}");
        }
        print!("\n");
    }
}

#[derive(Debug)]
struct Robot {
    position: (usize, usize),
    velocity: (isize, isize),
}

impl Robot {
    fn coords_after(&self, steps: isize, (xsize, ysize): (isize, isize)) -> (isize, isize) {
        let mut coords = (
            (self.position.0 as isize + self.velocity.0 * steps) % xsize,
            (self.position.1 as isize + self.velocity.1 * steps) % ysize,
        );
        if coords.0 < 0 {
            coords.0 += xsize;
        }
        if coords.1 < 0 {
            coords.1 += ysize;
        }
        coords
    }
}

fn run(input: &str) {
    let robots = input
        .lines()
        .map(|line| {
            let nums: (_, _, _, _) = line
                .split(|c| [' ', '=', ','].contains(&c))
                .filter_map(|n| n.parse::<isize>().ok())
                .next_tuple()
                .unwrap();
            Robot {
                position: (nums.0 as usize, nums.1 as usize),
                velocity: (nums.2, nums.3),
            }
        })
        .collect_vec();

    let steps = 100;
    //let grid_size = (11, 7);
    let grid_size = (101, 103);

    let safety_factors = robots
        .iter()
        .map(|robot| robot.coords_after(steps, grid_size))
        .fold((0, 0, 0, 0), |(mut a, mut b, mut c, mut d), (x, y)| {
            let xmid = grid_size.0 / 2;
            let ymid = grid_size.1 / 2;

            if x < xmid && y < ymid {
                a += 1;
            } else if x > xmid && y < ymid {
                b += 1;
            } else if x < xmid && y > ymid {
                c += 1;
            } else if x > xmid && y > ymid {
                d += 1;
            }

            (a, b, c, d)
        });

    let safety_factor = safety_factors.0 * safety_factors.1 * safety_factors.2 * safety_factors.3;

    println!("Safety factor part 1: {safety_factor}");

    let mut steps = 0;
    loop {
        let mut grid = vec![vec![false; grid_size.1 as usize]; grid_size.0 as usize];
        robots.iter().for_each(|robot| {
            let (y, x) = robot.coords_after(steps, grid_size);
            grid[y as usize][x as usize] = true;
        });
        let ma = grid
            .iter()
            .map(|line| {
                let (ma, _) = line.iter().fold((0i32, 0i32), |(ma, curr), n| {
                    let curr = if *n { curr + 1 } else { 0 };
                    let ma = max(ma, curr);
                    (ma, curr)
                });
                ma
            })
            .max()
            .unwrap();
        if ma > 10 {
            println!("Steps with long line in it: {steps}");
            print_grid(&grid);
            break;
        }
        steps += 1;
    }
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
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        run(&input);
    }
}
