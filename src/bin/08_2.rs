use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::dec_int;
use winnow::ascii::newline;
use winnow::combinator::{repeat, terminated};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn square_distance_to(&self, other: &JunctionBox) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn parse_junction_box(input: &mut &str) -> Result<JunctionBox> {
    (dec_int, ",", dec_int, ",", dec_int)
        .map(|(x, _, y, _, z)| JunctionBox { x, y, z })
        .parse_next(input)
}

fn parse_junction_boxes(input: &mut &str) -> Result<Vec<JunctionBox>> {
    repeat(1.., terminated(parse_junction_box, newline)).parse_next(input)
}

fn solve(input: &str) -> i64 {
    let boxes = {
        let mut boxes = parse_input!(parse_junction_boxes, input);
        boxes.sort();
        boxes
    };

    let distances_matrix = {
        let mut distances_matrix = vec![vec![i64::MAX; boxes.len()]; boxes.len()];

        for (i, i_box) in boxes.iter().enumerate() {
            for (j, j_box) in boxes.iter().enumerate().skip(i + 1) {
                distances_matrix[i][j] = i_box.square_distance_to(j_box);
            }
        }
        distances_matrix
    };

    let sorted_distances_vec = {
        let mut distances_vec: Vec<(i64, usize, usize)> = distances_matrix
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &dist)| (dist, i, j)))
            .collect();

        distances_vec.sort();
        distances_vec
    };

    let mut component: Vec<usize> = (0..boxes.len()).collect();

    for &(_, i, j) in &sorted_distances_vec {
        let component_i = component[i];
        let component_j = component[j];

        for c in &mut component {
            if *c == component_j {
                *c = component_i;
            }
        }

        if component.iter().all(|c| *c == component_i) {
            // this connection completed the circuit
            return boxes[i].x * boxes[j].x;
        }
    }

    unreachable!("all boxes should join the same component");
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

    test_with_files!("08", solve, 25272, 8520040659);
}
