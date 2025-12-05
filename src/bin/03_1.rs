use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::newline;
use winnow::combinator::{repeat, separated};
use winnow::token::one_of;

fn parse_single_digit_int(input: &mut &str) -> Result<i32> {
    one_of('0'..='9')
        .map(|single_digit: char| single_digit.to_digit(10).unwrap() as i32) // TODO: UGLY!
        .parse_next(input)
}

fn parse_bank(input: &mut &str) -> Result<Vec<i32>> {
    repeat(1.., parse_single_digit_int).parse_next(input)
}

fn parse_banks(input: &mut &str) -> Result<Vec<Vec<i32>>> {
    separated(0.., parse_bank, newline).parse_next(input)
}

fn solve(input: &str) -> i32 {
    let banks = parse_input!(parse_banks, input);

    let mut sum = 0;

    for bank in banks {
        for first_digit in (1..=9).rev() {
            if let Some(pos) = bank.iter().position(|&d| d == first_digit)
                && pos < bank.len() - 1
            {
                let second = bank[pos + 1..].iter().max().unwrap();
                let max_joltage = first_digit * 10 + second;
                sum += max_joltage;
                break;
            }
        }
    }
    sum
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

    test_with_files!("03", solve, 357, 17166);
}
