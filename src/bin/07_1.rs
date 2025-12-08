use aoc_2025::parse_input;
use std::collections::{HashSet, VecDeque};
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

type Splits = Option<((usize, usize), (usize, usize))>;

impl Grid {
    fn get(&self, i: usize, j: usize) -> Option<&Square> {
        self.0.get(i)?.get(j)
    }

    fn start(&self) -> (usize, usize) {
        for (i, row) in self.0.iter().enumerate() {
            for (j, s) in row.iter().enumerate() {
                if let Square::Start = s {
                    return (i, j);
                }
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

fn expand_laser(i: usize, j: usize, grid: &Grid, visited: &mut HashSet<(usize, usize)>) -> Splits {
    let mut expand_i = i;

    while let Some(s) = grid.get(expand_i, j) {
        if visited.contains(&(expand_i, j)) {
            break;
        }

        visited.insert((expand_i, j));
        match s {
            Square::Empty | Square::Start => expand_i += 1,
            Square::Splitter => {
                return Some(((expand_i, j - 1), (expand_i, j + 1)));
            }
        }
    }

    None
}

fn solve(input: &str) -> i32 {
    let grid = parse_input!(parse_problem, input);
    let start = grid.start();

    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([start]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut splitters = 0;

    while let Some((i, j)) = queue.pop_front() {
        if let Some((left, right)) = expand_laser(i, j, &grid, &mut visited) {
            splitters += 1;
            queue.push_back(left);
            queue.push_back(right);
        }
    }

    // for (i, row) in grid.0.iter().enumerate() {
    //     for (j, s) in row.iter().enumerate() {
    //         if visited.contains(&(i, j)) {
    //             match s {
    //                 Square::Empty => print!("|"),
    //                 Square::Start => print!("V"),
    //                 Square::Splitter => print!("X"),
    //             }
    //         } else {
    //             match s {
    //                 Square::Empty => print!("."),
    //                 Square::Start => print!("S"),
    //                 Square::Splitter => print!("^"),
    //             }
    //         }
    //     }
    //     println!("");
    // }

    splitters
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

    test_with_files!("07", solve, 21, 1539);
}
