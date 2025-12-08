use aoc_2025::parse_input;
use std::collections::HashMap;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::newline;
use winnow::combinator::{alt, fail, repeat, terminated};

enum Square {
    Empty,
    Start,
    Splitter,
}
struct Grid(Vec<Vec<Square>>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coords(usize, usize);

impl Coords {
    fn left(&self) -> Coords {
        Coords(self.0, self.1 - 1)
    }

    fn right(&self) -> Coords {
        Coords(self.0, self.1 + 1)
    }

    fn below(&self) -> Coords {
        Coords(self.0 + 1, self.1)
    }
}

impl Grid {
    fn get(&self, coords: Coords) -> Option<&Square> {
        let Coords(i, j) = coords;
        self.0.get(i)?.get(j)
    }

    fn start(&self) -> Coords {
        for (i, row) in self.0.iter().enumerate() {
            for (j, s) in row.iter().enumerate() {
                if let Square::Start = s { return Coords(i, j) }
            }
        }

        panic!("no starting square found");
    }
}

fn parse_square(input: &mut &str) -> Result<Square> {
    alt((
        '.'.map(|_| Square::Empty),
        '^'.map(|_| Square::Splitter),
        'S'.map(|_| Square::Start),
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

fn parse_problem(input: &mut &str) -> Result<Grid> {
    parse_grid.map(Grid).parse_next(input)
}

fn expand_laser(laser_start: Coords, grid: &Grid, saved_paths: &mut HashMap<Coords, i128>) -> i128 {
    let mut current_coords = laser_start;
    while let Some(s) = grid.get(current_coords) {
        match s {
            Square::Empty | Square::Start => current_coords = current_coords.below(),
            Square::Splitter => {
                return number_of_paths_from_splitter(current_coords, grid, saved_paths);
            }
        }
    }

    1
}

fn number_of_paths_from_splitter(
    splitter: Coords,
    grid: &Grid,
    saved_paths: &mut HashMap<Coords, i128>,
) -> i128 {
    if let Some(&n_paths) = saved_paths.get(&splitter) {
        n_paths
    } else {
        let mut n_paths = 0i128;

        n_paths += expand_laser(splitter.left(), grid, saved_paths);
        n_paths += expand_laser(splitter.right(), grid, saved_paths);

        saved_paths.insert(splitter, n_paths);
        n_paths
    }
}

fn solve(input: &str) -> i128 {
    let grid = parse_input!(parse_problem, input);
    let start = grid.start();
    let mut saved_paths: HashMap<Coords, i128> = HashMap::new();

    expand_laser(start, &grid, &mut saved_paths)
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

    test_with_files!("07", solve, 40, 6479180385864);
}
