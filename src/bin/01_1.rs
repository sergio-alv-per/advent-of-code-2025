use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::{dec_int, newline};
use winnow::combinator::{dispatch, fail, separated};
use winnow::token::take;

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn parse_rotation(input: &mut &str) -> Result<Rotation> {
    dispatch!(take(1usize);
        "L" => dec_int.map(|am| Rotation::Left(am)),
        "R" => dec_int.map(|am| Rotation::Right(am)),
        _ => fail,
    )
    .parse_next(input)
}

fn parse_rotation_list(input: &mut &str) -> Result<Vec<Rotation>> {
    separated(0.., parse_rotation, newline).parse_next(input)
}

fn solve(input: &str) -> i32 {
    let mut input = input;
    let rotations = parse_rotation_list(&mut input).expect("unparsable input");

    let mut initial_rotation = 50;
    let mut password = 0;

    for rot in rotations {
        match rot {
            Rotation::Left(amount) => initial_rotation = (initial_rotation + 100 - amount) % 100,
            Rotation::Right(amount) => initial_rotation = (initial_rotation + amount) % 100,
        }

        if initial_rotation == 0 {
            password += 1;
        }
    }

    password
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

    test_with_files!("01", solve, 3, 1026);
}
