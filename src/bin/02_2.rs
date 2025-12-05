use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::dec_int;
use winnow::combinator::separated;

#[derive(Debug)]
struct IdRange(i64, i64);

fn parse_id_range(input: &mut &str) -> Result<IdRange> {
    (dec_int, "-", dec_int)
        .map(|(start, _, end): (i64, &str, i64)| IdRange(start, end))
        .parse_next(input)
}

fn parse_id_ranges(input: &mut &str) -> Result<Vec<IdRange>> {
    separated(0.., parse_id_range, ",").parse_next(input)
}

fn solve(input: &str) -> i64 {
    let id_ranges = parse_input!(parse_id_ranges, input);

    let mut sum = 0i64;
    for IdRange(start, finish) in id_ranges {
        for middle_number in start..=finish {
            let middle_number_str = middle_number.to_string();
            for possible_pattern_length in 1..=middle_number_str.len() / 2 {
                if middle_number_str.len() % possible_pattern_length != 0 {
                    continue;
                }

                let pattern_bytes =
                    middle_number_str.as_str()[0..possible_pattern_length].as_bytes();

                if middle_number_str
                    .as_bytes()
                    .chunks(possible_pattern_length)
                    .all(|chunk| chunk == pattern_bytes)
                {
                    sum += middle_number;
                    break;
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

    test_with_files!("02", solve, 4174379265, 43287141963);
}
