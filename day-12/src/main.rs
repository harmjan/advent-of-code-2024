use std::{cell::RefCell, collections::VecDeque, fs};

fn is_same(grid: &Vec<Vec<char>>, (y, x): (usize, usize), (dy, dx): (isize, isize)) -> bool {
    grid.get((y as isize + dy) as usize)
        .and_then(|row| row.get((x as isize + dx) as usize))
        .copied()
        .unwrap_or(' ')
        == grid[y][x]
}

fn is_visited(visited: &Vec<Vec<bool>>, (y, x): (usize, usize), (dy, dx): (isize, isize)) -> bool {
    visited
        .get((y as isize + dy) as usize)
        .and_then(|row| row.get((x as isize + dx) as usize))
        .copied()
        .unwrap_or(false)
}

fn bfs(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    (y, x): (usize, usize),
) -> (u32, u32, u32) {
    let mut queue = VecDeque::new();
    queue.push_back((y, x));

    let mut area = 1;
    let mut perimeter = 0;

    let mut visited_this_round = vec![vec![false; grid[0].len()]; grid.len()];
    visited_this_round[y][x] = true;

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();

        for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ny = (y as isize + dy) as usize;
            let nx = (x as isize + dx) as usize;
            if is_same(grid, (y, x), (dy, dx)) {
                if !visited_this_round[ny][nx] {
                    visited_this_round[ny][nx] = true;
                    queue.push_back((ny, nx));
                    area += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    let area = area;
    let perimeter = perimeter;
    let mut perimeter_2 = 0;

    queue.push_back((y, x));
    visited[y][x] = true;
    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ny = (y as isize + dy) as usize;
            let nx = (x as isize + dx) as usize;
            if is_visited(&visited_this_round, (y, x), (dy, dx)) {
                if !visited[ny][nx] {
                    visited[ny][nx] = true;
                    queue.push_back((ny, nx));
                }
            } else {
                if dy == -1
                    && (!is_visited(&visited_this_round, (y, x), (0, -1))
                        || is_visited(&visited_this_round, (y, x), (-1, -1)))
                {
                    perimeter_2 += 1;
                } else if dy == 1
                    && (!is_visited(&visited_this_round, (y, x), (0, -1))
                        || is_visited(&visited_this_round, (y, x), (1, -1)))
                {
                    perimeter_2 += 1;
                } else if dx == -1
                    && (!is_visited(&visited_this_round, (y, x), (-1, 0))
                        || is_visited(&visited_this_round, (y, x), (-1, -1)))
                {
                    perimeter_2 += 1;
                } else if dx == 1
                    && (!is_visited(&visited_this_round, (y, x), (-1, 0))
                        || is_visited(&visited_this_round, (y, x), (-1, 1)))
                {
                    perimeter_2 += 1;
                }
            }
        }
    }

    (area, perimeter, perimeter_2)
}

fn run(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let visited = RefCell::new(vec![vec![false; grid[0].len()]; grid.len()]);

    let grid_ref = &grid;
    let visited_ref = &visited;
    let (price_1, price_2) = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let y = y;
            row.iter().enumerate().filter_map(move |(x, _)| {
                if visited_ref.borrow()[y][x] {
                    None
                } else {
                    Some(bfs(grid_ref, &mut visited_ref.borrow_mut(), (y, x)))
                }
            })
        })
        .map(|(area, perimeter, perimeter_2)| (area * perimeter, area * perimeter_2))
        .fold((0, 0), |(acc1, acc2), (a, b)| (acc1 + a, acc2 + b));

    println!("Price part 1: {price_1}");
    println!("Price part 2: {price_2}");
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
        let input = "AAAA
BBCD
BBCC
EEEC";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        run(&input);
    }

    #[test]
    fn sample_input_3() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        run(&input);
    }
}
