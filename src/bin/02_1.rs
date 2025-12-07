use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::{dec_int, newline};
use winnow::combinator::{separated, terminated};

#[derive(Debug)]
struct IdRange(i64, i64);

fn parse_id_range(input: &mut &str) -> Result<IdRange> {
    (dec_int, "-", dec_int)
        .map(|(start, _, end): (i64, &str, i64)| IdRange(start, end))
        .parse_next(input)
}

fn parse_id_ranges(input: &mut &str) -> Result<Vec<IdRange>> {
    terminated(separated(0.., parse_id_range, ","), newline).parse_next(input)
}

fn solve(input: &str) -> i64 {
    let id_ranges = parse_input!(parse_id_ranges, input);

    let mut sum = 0i64;
    for IdRange(start, finish) in id_ranges {
        for middle_number in start..=finish {
            let middle_number_str = middle_number.to_string();
            if middle_number_str.len() % 2 == 0 {
                let (halve1, halve2) = middle_number_str.split_at(middle_number_str.len() / 2);
                if halve1 == halve2 {
                    sum += middle_number;
                }
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

    test_with_files!("02", solve, 1227775554, 34826702005);
}
