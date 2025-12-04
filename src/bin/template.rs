use std::io::{self, Read};

#[allow(unused_variables)] // REMOVE ON PROBLEM FILES
fn solve(input: &str) -> i32 {
    0
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

    test_with_files!("00", solve, 0, 0); // TODO: CHANGE PLACEHOLDER VALUES
}
