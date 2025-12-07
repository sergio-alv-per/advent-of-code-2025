use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::{dec_int, newline};
use winnow::combinator::{repeat, separated_pair, terminated};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range(i64, i64);

impl Range {
    pub fn contains(&self, other: i64) -> bool {
        self.0 <= other && other <= self.1
    }
}
struct Problem {
    ranges: Vec<Range>,
    queries: Vec<i64>,
}

fn parse_range(input: &mut &str) -> Result<Range> {
    (dec_int, "-", dec_int)
        .map(|(range_start, _, range_end)| Range(range_start, range_end))
        .parse_next(input)
}

fn parse_ranges(input: &mut &str) -> Result<Vec<Range>> {
    repeat(1.., (parse_range, newline).map(|(range, _)| range)).parse_next(input)
}

fn parse_query(input: &mut &str) -> Result<i64> {
    dec_int.parse_next(input)
}

fn parse_queries(input: &mut &str) -> Result<Vec<i64>> {
    repeat(1.., terminated(parse_query, newline)).parse_next(input)
}

fn parse_problem(input: &mut &str) -> Result<Problem> {
    separated_pair(parse_ranges, newline, parse_queries)
        .map(|(ranges, queries)| Problem { ranges, queries })
        .parse_next(input)
}

fn consolidate_ranges(first_range: &Range, second_range: &Range) -> Option<Range> {
    // Check if two ranges can be joined into a single one
    // If they can, return the joined range. Else return None

    // First case, second range inside first range
    if first_range.contains(second_range.0) && first_range.contains(second_range.1) {
        Some(*first_range)
    // Second case, partial overlap
    } else if first_range.contains(second_range.0) && !first_range.contains(second_range.1) {
        Some(Range(first_range.0, second_range.1))
    // Third case, ranges do not overlap at all
    } else {
        None
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

fn query_in_a_range(query: i64, ranges: &[Range]) -> bool {
    match ranges.binary_search_by_key(&query, |range| range.0) {
        Ok(_) => true,
        Err(insertion_index) => {
            if insertion_index > 0 && insertion_index <= ranges.len() {
                let possible_containing_range = ranges.get(insertion_index - 1).unwrap();
                possible_containing_range.contains(query)
            } else {
                false
            }
        }
    }
}

fn solve(input: &str) -> usize {
    let Problem {
        mut ranges,
        queries,
    } = parse_input!(parse_problem, input);

    let consolidated_ranges = consolidate_all_ranges(&mut ranges);

    queries
        .iter()
        .filter(|&&q| query_in_a_range(q, consolidated_ranges))
        .count()
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

    test_with_files!("05", solve, 3, 885);
}
