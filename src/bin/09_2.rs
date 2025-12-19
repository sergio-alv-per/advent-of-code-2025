use aoc_2025::parse_input;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::dec_uint;
use winnow::ascii::newline;
use winnow::combinator::terminated;
use winnow::combinator::{repeat, separated_pair};

#[derive(Debug)]
struct Point {
    i: usize,
    j: usize,
}

impl Point {
    fn area_to(&self, other: &Point) -> usize {
        (self.i.abs_diff(other.i) + 1) * (self.j.abs_diff(other.j) + 1)
    }

    fn standarized_boundaires(&self, other: &Point) -> (Point, Point) {
        let upper_left = Point {
            i: min(self.i, other.i),
            j: min(self.j, other.j),
        };
        let lower_right = Point {
            i: max(self.i, other.i),
            j: max(self.j, other.j),
        };
        (upper_left, lower_right)
    }

    fn points_in_rect(&self, other: &Point) -> impl Iterator<Item = Point> {
        let (upper_right, lower_left) = self.standarized_boundaires(other);

        (upper_right.i..=lower_left.i)
            .flat_map(move |i| (upper_right.j..=lower_left.j).map(move |j| Point { i, j }))
    }
}

#[derive(Debug)]
struct BoolGrid {
    vecs: Vec<Vec<bool>>,
}

impl BoolGrid {
    fn like(other: &BoolGrid, init: bool) -> Self {
        BoolGrid {
            vecs: vec![vec![init; other.columns()]; other.rows()],
        }
    }
    fn new(rows: usize, columns: usize, init: bool) -> Self {
        BoolGrid {
            vecs: vec![vec![init; columns]; rows],
        }
    }

    fn rows(&self) -> usize {
        self.vecs.len()
    }

    fn columns(&self) -> usize {
        self.vecs[0].len()
    }

    fn get(&self, point: &Point) -> bool {
        self.vecs[point.i][point.j]
    }

    fn set(&mut self, point: &Point, value: bool) {
        self.vecs[point.i][point.j] = value;
    }

    fn adj(&self, point: &Point) -> Vec<Point> {
        let mut adj_points = Vec::new();
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Some(adj_i) = point.i.checked_add_signed(di)
                && adj_i < self.rows()
                && let Some(adj_j) = point.j.checked_add_signed(dj)
                && adj_j < self.columns()
            {
                adj_points.push(Point { i: adj_i, j: adj_j });
            }
        }

        adj_points
    }
}

impl fmt::Display for BoolGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid_string = String::new();

        for i in 0..self.rows() {
            for j in 0..self.columns() {
                let char_to_print = if self.vecs[i][j] { '#' } else { '.' };
                grid_string.push(char_to_print);
            }
            grid_string.push('\n');
        }

        grid_string.pop();

        write!(f, "{grid_string}")
    }
}

#[derive(Debug)]
struct CoordinateCompressor {
    compress_map: HashMap<usize, usize>,
    decompress_map: HashMap<usize, usize>,
}

impl CoordinateCompressor {
    fn new(coordinate_values: &[usize]) -> Self {
        let mut sorted_deduped_values: Vec<usize> = coordinate_values.to_vec();
        sorted_deduped_values.sort_unstable();
        sorted_deduped_values.dedup();

        let compress: HashMap<usize, usize> = sorted_deduped_values.into_iter().zip(0..).collect();
        let decompress: HashMap<usize, usize> = compress
            .iter()
            .map(|(&i, &compressed_i)| (compressed_i, i))
            .collect();

        CoordinateCompressor {
            compress_map: compress,
            decompress_map: decompress,
        }
    }

    fn compress(&self, val: usize) -> usize {
        self.compress_map[&val]
    }

    fn decompress(&self, val: usize) -> usize {
        self.decompress_map[&val]
    }

    fn len(&self) -> usize {
        self.compress_map.len()
    }
}

#[derive(Debug)]
struct PointCompressor {
    compressor_i: CoordinateCompressor,
    compressor_j: CoordinateCompressor,
}

impl PointCompressor {
    fn new(points: &[Point]) -> Self {
        let i_values: Vec<usize> = points.iter().map(|p| p.i).collect();
        let compressor_i = CoordinateCompressor::new(&i_values);

        let j_values: Vec<usize> = points.iter().map(|p| p.j).collect();
        let compressor_j = CoordinateCompressor::new(&j_values);

        PointCompressor {
            compressor_i,
            compressor_j,
        }
    }

    fn compress(&self, point: &Point) -> Point {
        Point {
            i: self.compressor_i.compress(point.i),
            j: self.compressor_j.compress(point.j),
        }
    }

    fn decompress(&self, point: &Point) -> Point {
        Point {
            i: self.compressor_i.decompress(point.i),
            j: self.compressor_j.decompress(point.j),
        }
    }

    fn rows(&self) -> usize {
        self.compressor_i.len()
    }

    fn columns(&self) -> usize {
        self.compressor_j.len()
    }
}

fn parse_point(input: &mut &str) -> Result<Point> {
    separated_pair(dec_uint, ",", dec_uint)
        .map(|(i, j)| Point { i, j })
        .parse_next(input)
}

fn parse_points(input: &mut &str) -> Result<Vec<Point>> {
    repeat(1.., terminated(parse_point, newline)).parse_next(input)
}

fn flood_fill_interior(is_edge: &BoolGrid) -> BoolGrid {
    let mut is_interior = BoolGrid::like(is_edge, true);
    let mut flood_stack: Vec<Point> = Vec::new();

    for i in 0..is_interior.rows() {
        flood_stack.push(Point { i, j: 0 });
        flood_stack.push(Point {
            i,
            j: is_interior.columns() - 1,
        });
    }

    for j in 0..is_interior.columns() {
        flood_stack.push(Point { i: 0, j });
        flood_stack.push(Point {
            i: is_interior.rows() - 1,
            j,
        });
    }

    while let Some(current_point) = flood_stack.pop() {
        if !is_interior.get(&current_point) || is_edge.get(&current_point) {
            continue;
        }
        is_interior.set(&current_point, false);

        for adj in is_interior.adj(&current_point) {
            flood_stack.push(adj);
        }
    }

    is_interior
}

fn solve(input: &str) -> usize {
    let points = parse_input!(parse_points, input);

    let compressor = PointCompressor::new(&points);
    let compressed_points: Vec<Point> = points.iter().map(|p| compressor.compress(p)).collect();

    let is_edge = {
        let mut is_edge = BoolGrid::new(compressor.rows(), compressor.columns(), false);

        for pi in 0..compressed_points.len() {
            let edge_start = &compressed_points[pi];
            let edge_end = &compressed_points[(&pi + 1) % points.len()];

            for edge_point in edge_start.points_in_rect(edge_end) {
                is_edge.set(&edge_point, true);
            }
        }
        is_edge
    };

    let is_interior = flood_fill_interior(&is_edge);

    println!("{is_interior}");

    let mut max_area: usize = 0;

    for (pi, p1) in compressed_points.iter().enumerate() {
        for p2 in compressed_points.iter().skip(pi + 1) {
            let dcp1 = compressor.decompress(p1);
            let dcp2 = compressor.decompress(p2);

            let new_area = dcp1.area_to(&dcp2);

            if new_area > max_area {
                // check if viable
                if p1.points_in_rect(p2).all(|p| is_interior.get(&p)) {
                    max_area = new_area;
                }
            }
        }
    }

    max_area
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

    test_with_files!("09", solve, 24, 1550760868);
}
