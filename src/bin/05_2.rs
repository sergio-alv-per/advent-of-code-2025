use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::{dec_int, newline};
use winnow::combinator::repeat;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range(i64, i64);

impl Range {
    fn contains(&self, &other: &i64) -> bool {
        self.0 <= other && other <= self.1
    }

    fn number_contained(&self) -> i64 {
        self.1 - self.0 + 1
    }
}

fn parse_range(input: &mut &str) -> Result<Range> {
    (dec_int, "-", dec_int)
        .map(|(range_start, _, range_end)| Range(range_start, range_end))
        .parse_next(input)
}

fn parse_ranges(input: &mut &str) -> Result<Vec<Range>> {
    repeat(1.., (parse_range, newline).map(|(range, _)| range)).parse_next(input)
}

fn consolidate_ranges(first_range: &Range, second_range: &Range) -> Option<Range> {
    // Check if two ranges can be joined into a single one
    // If they can, return the joined range. Else return None

    // First case, second range inside first range
    if first_range.contains(&second_range.0) && first_range.contains(&second_range.1) {
        return Some(first_range.clone());
    // Second case, partial overlap
    } else if first_range.contains(&second_range.0) && !first_range.contains(&second_range.1) {
        return Some(Range(first_range.0, second_range.1));
    // Third case, ranges do not overlap at all
    } else {
        return None;
    }
}

fn consolidate_all_ranges(ranges: &mut Vec<Range>) -> &Vec<Range> {
    ranges.sort();

    let mut range_index = 0usize;

    while range_index < (ranges.len() - 1) {
        let first_range = &ranges[range_index];
        let second_range = &ranges[range_index + 1];

        match consolidate_ranges(first_range, second_range) {
            Some(new_range) => {
                ranges.remove(range_index + 1);
                ranges.remove(range_index);
                ranges.insert(range_index, new_range);
            }
            None => range_index += 1,
        }
    }

    ranges
}

fn solve(input: &str) -> i64 {
    let mut ranges = parse_input!(parse_ranges, input);

    let consolidated_ranges = consolidate_all_ranges(&mut ranges);

    consolidated_ranges
        .iter()
        .map(|r| r.number_contained())
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

    test_with_files!("05", solve, 14, 348115621205535);
}
