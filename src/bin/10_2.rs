use aoc_2025::parse_input;
use std::io::{self, Read};
use std::iter::Sum;
use winnow::Parser;
use winnow::Result;
use winnow::ascii::{dec_uint, newline};
use winnow::combinator::{alt, delimited, repeat, separated, seq, terminated};
use z3::Optimize;
use z3::ast::Int;

struct ProblemRow {
    buttons: Vec<Vec<u16>>,
    joltages: Vec<u16>,
}

fn parse_light(input: &mut &str) -> Result<bool> {
    alt(('.', '#')).map(|c| c == '#').parse_next(input)
}
fn parse_light_diagram(input: &mut &str) -> Result<Vec<bool>> {
    delimited('[', repeat(1.., parse_light), ']').parse_next(input)
}

fn parse_u16_vec(input: &mut &str) -> Result<Vec<u16>> {
    separated(1.., dec_uint::<_, u16, _>, ',').parse_next(input)
}
fn parse_button(input: &mut &str) -> Result<Vec<u16>> {
    delimited('(', parse_u16_vec, ')').parse_next(input)
}

fn parse_buttons(input: &mut &str) -> Result<Vec<Vec<u16>>> {
    separated(1.., parse_button, ' ').parse_next(input)
}

fn parse_joltages(input: &mut &str) -> Result<Vec<u16>> {
    delimited('{', parse_u16_vec, '}').parse_next(input)
}

fn parse_problem_row(input: &mut &str) -> Result<ProblemRow> {
    seq! {ProblemRow{_: parse_light_diagram, _: ' ', buttons: parse_buttons, _: ' ', joltages: parse_joltages}}.parse_next(input)
}

fn parse_problem(input: &mut &str) -> Result<Vec<ProblemRow>> {
    repeat(1.., terminated(parse_problem_row, newline)).parse_next(input)
}

fn min_button_presses(pr: &ProblemRow, problem_i: u64) -> u64 {
    let mut bij: Vec<Vec<bool>> = vec![vec![false; pr.joltages.len()]; pr.buttons.len()];

    for (i, button) in pr.buttons.iter().enumerate() {
        for &j in button {
            bij[i][j as usize] = true;
        }
    }

    let opt = Optimize::new();
    let variables: Vec<Int> = {
        let mut vars = Vec::with_capacity(bij.len());

        for button_i in 0..bij.len() {
            vars.push(Int::new_const(format!("press_{problem_i}_{button_i}")));
        }
        vars
    };

    let total_presses = Int::sum(variables.iter());

    opt.minimize(&total_presses);

    for var in variables.iter() {
        opt.assert(&var.ge(0));
    }

    for (j, &joltage) in pr.joltages.iter().enumerate() {
        let vars_that_activate_this_j: Vec<&Int> = (0..bij.len())
            .filter_map(|i| if bij[i][j] { Some(&variables[i]) } else { None })
            .collect();

        if vars_that_activate_this_j.is_empty() {
            let panic_msg = format!("no buttons activate machine with j={}", j);
            panic!("{panic_msg}");
        }

        opt.assert(&Int::sum(vars_that_activate_this_j.into_iter()).eq(joltage));
    }

    match opt.check(&[]) {
        z3::SatResult::Sat => {
            let model = opt.get_model().unwrap();
            let presses = model.eval(&total_presses, true).unwrap();
            presses.as_u64().unwrap()
        }
        z3::SatResult::Unsat => {
            println!("{opt}");
            panic!("problem not solvable")
        }

        z3::SatResult::Unknown => {
            eprintln!("{}", opt.get_reason_unknown().unwrap());
            panic!("problem result unknown")
        }
    }
}

fn solve(input: &str) -> u64 {
    let problems = parse_input!(parse_problem, input);

    problems
        .iter()
        .enumerate()
        .map(|(i, pr)| {
            println!("solving problem");
            min_button_presses(pr, i.try_into().unwrap())
        })
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

    test_with_files!("10", solve, 33, 16663);
}
