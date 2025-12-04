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

fn recursive_max_joltage(remaining_bank: &[i32], usable_digits: usize) -> i64 {
    if usable_digits == 0 {
        return 0;
    }
    for starting_digit in (1..=9).rev() {
        if let Some(pos) = remaining_bank.iter().position(|&d| d == starting_digit)
            && pos <= remaining_bank.len() - usable_digits
        {
            let remaining_joltage =
                recursive_max_joltage(&remaining_bank[pos + 1..], usable_digits - 1);
            return starting_digit as i64 * 10i64.pow((usable_digits - 1) as u32)
                + remaining_joltage;
        }
    }
    0
}

fn solve(input: &str) -> i64 {
    let mut input = input;
    let banks = parse_banks(&mut input).expect("unparsable input");

    let mut sum = 0;

    for bank in banks {
        sum += recursive_max_joltage(&bank, 12);
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

    test_with_files!("03", solve, 3121910778619, 169077317650774);
}
