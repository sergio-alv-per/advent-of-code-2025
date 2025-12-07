use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::{newline, space0, space1};
use winnow::combinator::{alt, delimited, repeat, separated};
use winnow::token::one_of;

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
}

fn parse_numbers_line(input: &mut &str) -> Result<Vec<char>> {
    repeat(1.., one_of(('0'..='9', ' '))).parse_next(input)
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

fn parse_math_problem(input: &mut &str) -> Result<(Vec<Vec<char>>, Vec<Operation>)> {
    (
        separated(1.., parse_numbers_line, newline),
        newline,
        parse_operations_row,
        newline,
    )
        .map(|(number_rows, _, operations_row, _)| (number_rows, operations_row))
        .parse_next(input)
}

fn char_vec_to_i64(char_vec: &Vec<char>) -> Option<i64> {
    if char_vec.iter().all(|&c| c == ' ') {
        None
    } else {
        Some(
            char_vec
                .iter()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .expect("could not transform char vec to i64"),
        )
    }
}

fn solve(input: &str) -> i64 {
    let (chars_matrix, operations_row) = parse_input!(parse_math_problem, input);

    let transposed_char_matrix: Vec<Vec<char>> = (0..chars_matrix[0].len())
        .map(|i| chars_matrix.iter().map(|inner| inner[i]).collect())
        .collect();

    let numbers_separated_by_none: Vec<Option<i64>> =
        transposed_char_matrix.iter().map(char_vec_to_i64).collect();

    let number_columns: Vec<Vec<i64>> = numbers_separated_by_none
        .split(|x| x.is_none())
        .map(|col| col.iter().filter_map(|&x| x).collect())
        .collect();

    operations_row
        .iter()
        .zip(&number_columns)
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

    test_with_files!("06", solve, 3263827, 8674740488592);
}
