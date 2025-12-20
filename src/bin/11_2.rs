use aoc_2025::parse_input;
use std::collections::HashMap;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::{alpha1, newline};
use winnow::combinator::{repeat, separated, separated_pair, terminated};

#[derive(Debug, Clone)]
struct Graph<'s> {
    mappings: HashMap<&'s str, Vec<&'s str>>,
}

impl<'s> From<Vec<(&'s str, Vec<&'s str>)>> for Graph<'s> {
    fn from(value: Vec<(&'s str, Vec<&'s str>)>) -> Self {
        let mut all_nodes: Vec<&'s str> = Vec::new();
        let mut mappings: HashMap<&'s str, Vec<&'s str>> = value.into_iter().collect();

        // Ensure all nodes are in mappings
        for (&initial, destinies) in &mappings {
            all_nodes.push(initial);
            all_nodes.extend(destinies.iter());
        }

        all_nodes.sort_unstable();
        all_nodes.dedup();

        for &node in &all_nodes {
            mappings.entry(node).or_default();
        }

        Graph { mappings }
    }
}

impl<'s> Graph<'s> {
    fn children(&self, node: &'s str) -> Vec<&'s str> {
        self.mappings
            .get(node)
            .expect("all nodes should be in mappings")
            .clone()
    }
}

fn parse_spaced_labels<'s>(input: &mut &'s str) -> Result<Vec<&'s str>> {
    separated(1.., alpha1, ' ').parse_next(input)
}

fn parse_mapping_line<'s>(input: &mut &'s str) -> Result<(&'s str, Vec<&'s str>)> {
    separated_pair(alpha1, ": ", parse_spaced_labels).parse_next(input)
}

fn parse_graph<'s>(input: &mut &'s str) -> Result<Graph<'s>> {
    repeat(1.., terminated(parse_mapping_line, newline))
        .map(|mappings_vec: Vec<(&'s str, Vec<&'s str>)>| Graph::from(mappings_vec))
        .parse_next(input)
}

fn paths_to_other_recursive<'s>(
    from: &'s str,
    other: &'s str,
    graph: &Graph<'s>,
    n_paths_map: &mut HashMap<&'s str, u64>,
) -> u64 {
    if let Some(n) = n_paths_map.get(from) {
        return *n;
    }

    let n_paths = if from == other {
        1
    } else {
        graph
            .children(from)
            .iter()
            .map(|&c| paths_to_other_recursive(c, other, graph, n_paths_map))
            .sum()
    };

    n_paths_map.insert(from, n_paths);

    n_paths
}

fn paths_to_other<'s>(from: &'s str, other: &'s str, graph: &Graph<'s>) -> u64 {
    let mut n_paths_map = HashMap::new();
    paths_to_other_recursive(from, other, graph, &mut n_paths_map)
}

fn solve(input: &str) -> u64 {
    let graph = parse_input!(parse_graph, input);

    (paths_to_other("svr", "dac", &graph)
        * paths_to_other("dac", "fft", &graph)
        * paths_to_other("fft", "out", &graph))
        + (paths_to_other("svr", "fft", &graph)
            * paths_to_other("fft", "dac", &graph)
            * paths_to_other("dac", "out", &graph))
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

    test_with_files!("11", solve, 2, 0);
}
