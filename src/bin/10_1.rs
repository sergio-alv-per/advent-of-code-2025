use aoc_2025::parse_input;
use std::io::{self, Read};
use winnow::Parser;
use winnow::Result;
use winnow::ascii::{dec_uint, newline};
use winnow::combinator::{alt, delimited, repeat, separated, seq, terminated};
use z3::Solver;
use z3::ast::BV;
use z3::ast::Bool;

const MAX_LIGHTMAP_SIZE: u32 = 16;
const MAX_BUTTON_PRESSES: u64 = 100;

#[derive(Clone, Copy)]
struct LightDiagram {
    activated_lights: u16,
}

impl From<Vec<bool>> for LightDiagram {
    fn from(value: Vec<bool>) -> Self {
        LightDiagram {
            activated_lights: value.iter().enumerate().fold(0, |acc, (i, &light)| {
                if light { acc | (1 << i) } else { acc }
            }),
        }
    }
}

impl From<Vec<u16>> for LightDiagram {
    fn from(value: Vec<u16>) -> Self {
        LightDiagram {
            activated_lights: value.iter().fold(0, |acc, i| acc | (1 << i)),
        }
    }
}

impl From<u16> for LightDiagram {
    fn from(value: u16) -> Self {
        LightDiagram {
            activated_lights: value,
        }
    }
}

impl From<LightDiagram> for BV {
    fn from(value: LightDiagram) -> Self {
        BV::from_u64(value.activated_lights.into(), MAX_LIGHTMAP_SIZE)
    }
}

struct ProblemRow {
    target_light_diagram: LightDiagram,
    buttons: Vec<LightDiagram>,
    _joltages: Vec<u16>,
}

fn parse_light(input: &mut &str) -> Result<bool> {
    alt(('.', '#')).map(|c| c == '#').parse_next(input)
}
fn parse_light_diagram(input: &mut &str) -> Result<LightDiagram> {
    delimited('[', repeat(1.., parse_light), ']')
        .map(|lights: Vec<bool>| LightDiagram::from(lights))
        .parse_next(input)
}

fn parse_u6_vec(input: &mut &str) -> Result<Vec<u16>> {
    separated(1.., dec_uint::<_, u16, _>, ',').parse_next(input)
}
fn parse_button(input: &mut &str) -> Result<LightDiagram> {
    delimited('(', parse_u6_vec, ')')
        .map(|triggered_lights| LightDiagram::from(triggered_lights))
        .parse_next(input)
}

fn parse_buttons(input: &mut &str) -> Result<Vec<LightDiagram>> {
    separated(1.., parse_button, ' ').parse_next(input)
}

fn parse_joltages(input: &mut &str) -> Result<Vec<u16>> {
    delimited('{', parse_u6_vec, '}').parse_next(input)
}

fn parse_problem_row(input: &mut &str) -> Result<ProblemRow> {
    seq! {ProblemRow{target_light_diagram: parse_light_diagram, _: ' ', buttons: parse_buttons, _: ' ', _joltages: parse_joltages}}.parse_next(input)
}

fn parse_problem(input: &mut &str) -> Result<Vec<ProblemRow>> {
    repeat(1.., terminated(parse_problem_row, newline)).parse_next(input)
}

fn solvable_in_n_button_presses(problem_row: &ProblemRow, n: u64) -> bool {
    let solver = Solver::new();
    let zero_bv = BV::from_u64(0, MAX_LIGHTMAP_SIZE);
    let variables: Vec<BV> = {
        let mut vars = Vec::with_capacity(n.try_into().expect("u64 does not fit in usize"));

        for _ in 0..n {
            vars.push(BV::fresh_const("press", MAX_LIGHTMAP_SIZE));
        }
        vars
    };

    let target_bv = BV::from(problem_row.target_light_diagram);

    solver.assert(
        variables
            .iter()
            .fold(target_bv, |acc, e| acc.bvxor(e))
            .eq(&zero_bv),
    );

    for pushed_button in variables.iter() {
        let equals_some_button = problem_row
            .buttons
            .iter()
            .fold(Bool::from_bool(false), |acc, but| {
                acc | pushed_button.eq(BV::from(but.clone()))
            });

        solver.assert(equals_some_button | pushed_button.eq(&zero_bv));
    }

    match solver.check() {
        z3::SatResult::Unsat => false,
        z3::SatResult::Sat => true,
        z3::SatResult::Unknown => panic!("problem result unknown"),
    }
}

fn min_n_to_solve_problem(pr: &ProblemRow) -> u64 {
    let button_presses_array: Vec<u64> = (0..MAX_BUTTON_PRESSES).collect();
    button_presses_array
        .partition_point(|&n| !solvable_in_n_button_presses(&pr, n))
        .try_into()
        .expect("usize did not fit in u64")
}

fn solve(input: &str) -> u64 {
    let problems = parse_input!(parse_problem, input);

    problems.iter().map(min_n_to_solve_problem).sum()
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

    test_with_files!("10", solve, 7, 415);
}
