use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet, VecDeque},
    fs,
};

fn print_grid(grid: &Vec<Vec<char>>, tiles: &HashSet<(usize, usize)>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let a = if tiles.contains(&(y, x)) {
                'O'
            } else {
                grid[y][x]
            };
            print!("{a}");
        }
        print!("\n");
    }
}

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
enum Direction {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }
    fn forward(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Self::East => (position.0, position.1 + 1),
            Self::South => (position.0 + 1, position.1),
            Self::West => (position.0, position.1 - 1),
            Self::North => (position.0 - 1, position.1),
        }
    }
    fn backward(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Self::East => (position.0, position.1 - 1),
            Self::South => (position.0 - 1, position.1),
            Self::West => (position.0, position.1 + 1),
            Self::North => (position.0 + 1, position.1),
        }
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::East,
            1 => Direction::North,
            2 => Direction::West,
            3 => Direction::South,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = find_character(&grid, 'S');
    let target = find_character(&grid, 'E');

    let mut visited = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|_| [const { None }; 4])
                .collect::<Vec<[Option<usize>; 4]>>()
        })
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: start,
        direction: Direction::East,
    });

    loop {
        let state = match heap.pop() {
            Some(state) => state,
            None => break,
        };

        if visited[state.position.0][state.position.1][state.direction as usize].is_some() {
            continue;
        }
        if grid[state.position.0][state.position.1] == '#' {
            continue;
        }
        visited[state.position.0][state.position.1][state.direction as usize] = Some(state.cost);

        heap.push(State {
            cost: state.cost + 1000,
            position: state.position,
            direction: state.direction.turn_left(),
        });
        heap.push(State {
            cost: state.cost + 1000,
            position: state.position,
            direction: state.direction.turn_right(),
        });
        heap.push(State {
            cost: state.cost + 1,
            position: state.direction.forward(state.position),
            direction: state.direction,
        });
    }

    let min_cost = visited[target.0][target.1].iter().min().unwrap().unwrap();

    println!("Cost of a best path: {min_cost}");

    let mut queue = VecDeque::<State>::new();
    let mut in_best_path = HashSet::<(usize, usize)>::new();
    for (direction, final_cost) in visited[target.0][target.1].iter().enumerate() {
        if final_cost.unwrap() == min_cost {
            queue.push_front(State {
                cost: min_cost,
                position: target,
                direction: direction.into(),
            })
        }
    }

    loop {
        let state = match queue.pop_back() {
            Some(state) => state,
            None => break,
        };

        in_best_path.insert(state.position);

        if visited[state.position.0][state.position.1][state.direction.turn_left() as usize]
            .map(|cost| cost == state.cost.wrapping_sub(1000))
            .unwrap_or(false)
        {
            queue.push_front(State {
                cost: state.cost - 1000,
                position: state.position,
                direction: state.direction.turn_left(),
            });
        }

        if visited[state.position.0][state.position.1][state.direction.turn_right() as usize]
            .map(|cost| cost == state.cost.wrapping_sub(1000))
            .unwrap_or(false)
        {
            queue.push_front(State {
                cost: state.cost - 1000,
                position: state.position,
                direction: state.direction.turn_right(),
            });
        }

        let backward_pos = state.direction.backward(state.position);
        if visited[backward_pos.0][backward_pos.1][state.direction as usize]
            .map(|cost| cost == state.cost.wrapping_sub(1))
            .unwrap_or(false)
        {
            queue.push_front(State {
                cost: state.cost - 1,
                position: backward_pos,
                direction: state.direction,
            });
        }
    }

    print_grid(&grid, &in_best_path);

    println!("Nodes in best paths: {}", in_best_path.len())
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
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        run(&input);
    }
}
