use std::fs;

fn check(
    grid: &Vec<Vec<char>>,
    word: &'static str,
    x: usize,
    y: usize,
    xdiff: isize,
    ydiff: isize,
) -> bool {
    word.chars().enumerate().all(|(i, c)| {
        grid.get((x as isize + xdiff * i as isize) as usize)
            .map_or(false, |row| {
                row.get((y as isize + ydiff * i as isize) as usize)
                    .map_or(false, |c2| *c2 == c)
            })
    })
}

fn check_all_directions(grid: &Vec<Vec<char>>, word: &'static str, x: usize, y: usize) -> usize {
    [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]
    .iter()
    .filter(|(xdiff, ydiff)| check(grid, word, x, y, *xdiff, *ydiff))
    .count()
}

fn check_cross_mas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    (check(grid, "MAS", x.wrapping_sub(1), y.wrapping_sub(1), 1, 1)
        || check(grid, "SAM", x.wrapping_sub(1), y.wrapping_sub(1), 1, 1))
        && (check(grid, "MAS", x.wrapping_sub(1), y + 1, 1, -1)
            || check(grid, "SAM", x.wrapping_sub(1), y + 1, 1, -1))
}

fn run(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let amount_of_xmas: usize = (0..grid.len())
        .flat_map(|x| (0..grid[x].len()).map(move |y| (x, y)))
        .map(|(x, y)| check_all_directions(&grid, "XMAS", x, y))
        .sum();

    println!("XMAS occurrences: {amount_of_xmas}");

    let amount_of_cross_max: usize = (0..grid.len())
        .flat_map(|x| (0..grid[x].len()).map(move |y| (x, y)))
        .filter(|(x, y)| check_cross_mas(&grid, *x, *y))
        .count();

    println!("Cross mas occurrences: {amount_of_cross_max}");
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
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

        run(&input);
    }
}
