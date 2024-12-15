use std::fs;

fn print_grid(grid: &Vec<Vec<char>>, position: (usize, usize)) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let a = if position == (y, x) { '@' } else { grid[y][x] };
            print!("{a}");
        }
        print!("\n");
    }
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .filter_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, c)| if *c == '@' { Some((y, x)) } else { None })
                .next()
        })
        .next()
        .unwrap()
}

fn calculate_gps(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if ['O', '['].contains(c) {
                        Some(y * 100 + x)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn simulate(
    grid: &mut Vec<Vec<char>>,
    move_: char,
    mut position: (usize, usize),
) -> (usize, usize) {
    let (dx, dy): (isize, isize) = match move_ {
        '<' => (-1, 0),
        'v' => (0, 1),
        '>' => (1, 0),
        '^' => (0, -1),
        _ => unreachable!(),
    };

    let mut steps: isize = 1;

    loop {
        let (ny, nx) = (
            (position.0 as isize + dy * steps) as usize,
            (position.1 as isize + dx * steps) as usize,
        );
        let space = grid[ny][nx];
        if space == '#' {
            break;
        } else if space == '.' {
            for a in (1..(steps + 1)).rev() {
                grid[(position.0 as isize + dy * a) as usize]
                    [(position.1 as isize + dx * a) as usize] = grid
                    [(position.0 as isize + dy * (a - 1)) as usize]
                    [(position.1 as isize + dx * (a - 1)) as usize]
            }
            position = (
                (position.0 as isize + dy) as usize,
                (position.1 as isize + dx) as usize,
            );
            break;
        }
        steps += 1;
    }

    position
}

fn scan_boxes(
    grid: &mut Vec<Vec<char>>,
    position: (usize, usize),
    direction: (isize, isize),
    prev_value: Option<char>,
) -> bool {
    let (dy, dx) = direction;
    let (ny, nx) = (
        (position.0 as isize + dy) as usize,
        (position.1 as isize + dx) as usize,
    );
    let space = grid[ny][nx];

    let possible = match space {
        '#' => false,
        '.' => true,
        '[' => {
            if dy == 0 {
                scan_boxes(grid, (ny, nx), direction, prev_value.map(|_| space))
            } else {
                scan_boxes(grid, (ny, nx), direction, prev_value.map(|_| space))
                    && scan_boxes(grid, (ny, nx + 1), direction, prev_value.map(|_| ']'))
            }
        }
        ']' => {
            if dy == 0 {
                scan_boxes(grid, (ny, nx), direction, prev_value.map(|_| space))
            } else {
                scan_boxes(grid, (ny, nx), direction, prev_value.map(|_| space))
                    && scan_boxes(grid, (ny, nx - 1), direction, prev_value.map(|_| '['))
            }
        }
        _ => unreachable!(),
    };

    if let Some(prev) = prev_value {
        if possible {
            grid[position.0][position.1] = grid[ny][nx];
            grid[ny][nx] = prev;
        }
    }

    possible
}

fn simulate2(grid: &mut Vec<Vec<char>>, move_: char, position: (usize, usize)) -> (usize, usize) {
    let direction: (isize, isize) = match move_ {
        '<' => (0, -1),
        'v' => (1, 0),
        '>' => (0, 1),
        '^' => (-1, 0),
        _ => unreachable!(),
    };

    if scan_boxes(grid, position, direction, None) {
        scan_boxes(
            grid,
            position,
            direction,
            Some(grid[position.0][position.1]),
        );
        let (dy, dx) = direction;
        (
            (position.0 as isize + dy) as usize,
            (position.1 as isize + dx) as usize,
        )
    } else {
        position
    }
}

fn run(input: &str) {
    let (grid_input, moves_input) = input.split_once("\n\n").unwrap();

    let mut grid = grid_input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let moves = moves_input
        .chars()
        .filter(|move_| ['<', 'v', '>', '^'].contains(move_))
        .collect::<Vec<_>>();

    let mut position = find_start(&grid);
    grid[position.0][position.1] = '.';

    for move_ in moves.iter() {
        position = simulate(&mut grid, *move_, position);
    }

    print_grid(&grid, position);

    let gps = calculate_gps(&grid);

    println!("Sum of GPS part 1: {gps}");

    let mut grid = grid_input
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| {
                    match c {
                        '#' => ['#', '#'],
                        '.' => ['.', '.'],
                        'O' => ['[', ']'],
                        '@' => ['@', '.'],
                        _ => unreachable!(),
                    }
                    .into_iter()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut position = find_start(&grid);
    grid[position.0][position.1] = '.';

    for move_ in moves.iter() {
        position = simulate2(&mut grid, *move_, position);
    }

    print_grid(&grid, position);

    let gps = calculate_gps(&grid);

    println!("Sum of GPS part 2: {gps}");
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
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        run(&input);
    }

    #[test]
    fn custom() {
        let input = "########
#......#
#...O..#
#.@O...#
#......#
#......#
#......#
########

>>v>^^";

        run(&input);
    }
}
