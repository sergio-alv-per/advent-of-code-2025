use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::{dec_uint, newline};
use winnow::combinator::{alt, repeat, separated, separated_pair, seq, terminated};

type Shape = [[bool; 3]; 3];

#[derive(Debug)]
struct Present {
    shape: Shape,
}

impl Present {
    fn area(&self) -> u64 {
        self.shape
            .iter()
            .flat_map(|row| row.iter())
            .map(|&c| u64::from(c))
            .sum()
    }
}

#[derive(Debug)]
struct Region {
    width: u64,
    height: u64,
    shape_quantities: Vec<u64>,
}

fn parse_id_collon(input: &mut &str) -> Result<u64> {
    terminated(dec_uint, ":").parse_next(input)
}

fn parse_shape_tile(input: &mut &str) -> Result<bool> {
    alt(('.', '#')).map(|c| c == '#').parse_next(input)
}

fn parse_shape_row(input: &mut &str) -> Result<[bool; 3]> {
    (parse_shape_tile, parse_shape_tile, parse_shape_tile)
        .map(|(t1, t2, t3)| [t1, t2, t3])
        .parse_next(input)
}

fn parse_shape(input: &mut &str) -> Result<[[bool; 3]; 3]> {
    seq!(parse_shape_row, _: newline, parse_shape_row, _: newline, parse_shape_row, _: newline)
        .map(|(r1, r2, r3)| [r1, r2, r3])
        .parse_next(input)
}

fn parse_present(input: &mut &str) -> Result<Present> {
    seq!(Present{_: parse_id_collon, _: newline, shape: parse_shape}).parse_next(input)
}

fn parse_presents(input: &mut &str) -> Result<Vec<Present>> {
    separated(1.., parse_present, newline).parse_next(input)
}

fn parse_region_dimensions(input: &mut &str) -> Result<(u64, u64)> {
    separated_pair(dec_uint, 'x', dec_uint).parse_next(input)
}

fn parse_shape_quantities(input: &mut &str) -> Result<Vec<u64>> {
    separated(1.., dec_uint::<_, u64, _>, ' ').parse_next(input)
}

fn parse_region(input: &mut &str) -> Result<Region> {
    separated_pair(parse_region_dimensions, ": ", parse_shape_quantities)
        .map(|(dims, quantities)| Region {
            width: dims.0,
            height: dims.1,
            shape_quantities: quantities,
        })
        .parse_next(input)
}

fn parse_regions(input: &mut &str) -> Result<Vec<Region>> {
    repeat(1.., terminated(parse_region, newline)).parse_next(input)
}

fn parse_problem(input: &mut &str) -> Result<(Vec<Present>, Vec<Region>)> {
    separated_pair(parse_presents, newline, parse_regions).parse_next(input)
}

fn solve(input: &str) -> u64 {
    let (presents, regions) = parse_input!(parse_problem, input);

    // Just count if there is enough room in the tree for all the presents
    regions
        .iter()
        .map(|reg| {
            reg.shape_quantities
                .iter()
                .zip(&presents)
                .map(|(quantity, present)| present.area() * quantity)
                .sum::<u64>()
                <= (reg.height * reg.width)
        })
        .map(u64::from)
        .sum()
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

    test_with_files!("12", solve, 2, 0);
}
