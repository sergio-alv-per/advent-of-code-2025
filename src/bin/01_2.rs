use aoc_2025::parse_input;
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

fn apply_rotation(angle: &i32, rotation: &Rotation) -> (i32, i32) {
    let clicks = match rotation {
        Rotation::Left(amount) if *angle == 0 => (angle + amount) / 100,
        Rotation::Left(amount) => ((100 - angle) + amount) / 100,
        Rotation::Right(amount) => (angle + amount) / 100,
    };

    let new_angle = match rotation {
        Rotation::Left(amount) => (angle - amount).rem_euclid(100),
        Rotation::Right(amount) => (angle + amount).rem_euclid(100),
    };

    (new_angle, clicks)
}

fn solve(input: &str) -> i32 {
    let rotations = parse_input!(parse_rotation_list, input);

    let password = rotations
        .iter()
        .scan(50, |angle, rotation| {
            let (new_angle, added_clicks) = apply_rotation(angle, &rotation);
            *angle = new_angle;
            Some(added_clicks)
        })
        .sum();

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

    test_with_files!("01", solve, 6, 5923);
}
