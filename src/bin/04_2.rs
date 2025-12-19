use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::newline;
use winnow::combinator::{alt, fail, repeat, terminated};

enum Square {
    PaperRoll,
    Empty,
}

fn parse_square(input: &mut &str) -> Result<Square> {
    alt((
        '.'.map(|_| Square::Empty),
        '@'.map(|_| Square::PaperRoll),
        fail,
    ))
    .parse_next(input)
}

fn parse_row(input: &mut &str) -> Result<Vec<Square>> {
    repeat(1.., parse_square).parse_next(input)
}

fn parse_grid(input: &mut &str) -> Result<Vec<Vec<Square>>> {
    repeat(1.., terminated(parse_row, newline)).parse_next(input)
}

fn roll_in_adjacent(i: usize, j: usize, i_dir: isize, j_dir: isize, grid: &[Vec<Square>]) -> bool {
    let rows = grid.len();
    let cols = grid.first().expect("grid didn't have any rows").len();
    let possible_adj_i = i.checked_add_signed(i_dir).filter(|&adj_i| adj_i < rows);
    let possible_adj_j = j.checked_add_signed(j_dir).filter(|&adj_j| adj_j < cols);

    match (possible_adj_i, possible_adj_j) {
        (Some(adj_i), Some(adj_j)) => match grid[adj_i][adj_j] {
            Square::Empty => false,
            Square::PaperRoll => true,
        },
        _ => false,
    }
}

fn rolls_in_adjacent_squares(i: usize, j: usize, grid: &[Vec<Square>]) -> i32 {
    let mut rolls = 0;

    for i_dir in -1..=1 {
        for j_dir in -1..=1 {
            if (i_dir != 0 || j_dir != 0) && roll_in_adjacent(i, j, i_dir, j_dir, grid) {
                rolls += 1;
            }
        }
    }

    rolls
}

fn solve(input: &str) -> i32 {
    let mut grid = parse_input!(parse_grid, input);
    let mut removable_rolls = 0;

    loop {
        let erasable: Vec<(usize, usize, i32)> = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, sq)| (i, j, sq)))
            .filter_map(|(i, j, sq)| {
                if let Square::PaperRoll = sq {
                    Some((i, j, rolls_in_adjacent_squares(i, j, &grid)))
                } else {
                    None
                }
            })
            .filter(|&(_, _, n_rolls)| n_rolls < 4)
            .collect();
        if erasable.is_empty() {
            break;
        }

        for (i, j, _) in erasable {
            grid[i][j] = Square::Empty;
            removable_rolls += 1;
        }
    }

    removable_rolls
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("error reading input from stdin");
    let solution = solve(&input);
    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::solve;
    use aoc_2025::test_with_files;

    test_with_files!("04", solve, 43, 8899);
}
