use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::{dec_int, newline, space0, space1};
use winnow::combinator::{alt, delimited, separated};

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
}

fn parse_number_row(input: &mut &str) -> Result<Vec<i64>> {
    delimited(space0, separated(1.., dec_int::<_, i64, _>, space1), space0).parse_next(input)
}

fn parse_operation(input: &mut &str) -> Result<Operation> {
    alt(('+', '*'))
        .map(|c| match c {
            '+' => Operation::Sum,
            '*' => Operation::Product,
            _ => panic!(),
        })
        .parse_next(input)
}

fn parse_operations_row(input: &mut &str) -> Result<Vec<Operation>> {
    delimited(space0, separated(1.., parse_operation, space1), space0).parse_next(input)
}

fn parse_math_problem(input: &mut &str) -> Result<(Vec<Vec<i64>>, Vec<Operation>)> {
    (
        separated(1.., parse_number_row, newline),
        newline,
        parse_operations_row,
        newline,
    )
        .map(|(number_rows, _, operations_row, _)| (number_rows, operations_row))
        .parse_next(input)
}

fn solve(input: &str) -> i64 {
    let (number_rows, operations_row) = parse_input!(parse_math_problem, input);
    let transposed_numbers: Vec<Vec<i64>> = (0..number_rows[0].len())
        .map(|i| number_rows.iter().map(|inner| inner[i]).collect())
        .collect();

    operations_row
        .iter()
        .zip(&transposed_numbers)
        .map(|(op, col)| match op {
            Operation::Sum => col.iter().sum::<i64>(),
            Operation::Product => col.iter().product(),
        })
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

    test_with_files!("06", solve, 4277556, 4878670269096);
}
