use aoc_2025::parse_input;
use std::cmp::max;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::dec_int;
use winnow::ascii::newline;
use winnow::combinator::terminated;
use winnow::combinator::{repeat, separated_pair};

#[derive(Debug)]
struct Point {
    i: i64,
    j: i64,
}
fn area(p1: &Point, p2: &Point) -> i64 {
    (p1.i - p2.i + 1).abs() * (p1.j - p2.j + 1).abs()
}

fn parse_point(input: &mut &str) -> Result<Point> {
    separated_pair(dec_int, ",", dec_int)
        .map(|(i, j)| Point { i, j })
        .parse_next(input)
}

fn parse_points(input: &mut &str) -> Result<Vec<Point>> {
    repeat(1.., terminated(parse_point, newline)).parse_next(input)
}

fn solve(input: &str) -> i64 {
    let points = parse_input!(parse_points, input);

    let mut max_area = 0;
    for (pi, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(pi + 1) {
            max_area = max(max_area, area(p1, p2));
        }
    }
    max_area
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

    test_with_files!("09", solve, 50, 0);
}
