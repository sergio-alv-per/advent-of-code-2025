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
    light_map: Vec<bool>,
    buttons: Vec<Vec<u16>>,
}

fn parse_light(input: &mut &str) -> Result<bool> {
    alt(('.', '#')).map(|c| c == '#').parse_next(input)
}
fn parse_light_map(input: &mut &str) -> Result<Vec<bool>> {
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
    seq! {ProblemRow{light_map: parse_light_map, _: ' ', buttons: parse_buttons, _: ' ', _: parse_joltages}}.parse_next(input)
}

fn parse_problem(input: &mut &str) -> Result<Vec<ProblemRow>> {
    repeat(1.., terminated(parse_problem_row, newline)).parse_next(input)
}

fn button_machine_activation_matrix(buttons: &[Vec<u16>], n_machines: usize) -> Vec<Vec<bool>> {
    let mut button_machine_matrix: Vec<Vec<bool>> = vec![vec![false; n_machines]; buttons.len()];

    for (i, button) in buttons.iter().enumerate() {
        for &j in button {
            button_machine_matrix[i][j as usize] = true;
        }
    }

    button_machine_matrix
}

fn init_z3_variables(n_variables: usize, problem_i: usize) -> Vec<Int> {
    let mut variables = Vec::with_capacity(n_variables);

    for button_i in 0..n_variables {
        variables.push(Int::new_const(format!("press_{problem_i}_{button_i}")));
    }
    variables
}

fn min_button_presses(pr: &ProblemRow, problem_index: usize) -> u64 {
    let button_machine_matrix = button_machine_activation_matrix(&pr.buttons, pr.light_map.len());

    let optimizer = Optimize::new();
    let button_presses = init_z3_variables(button_machine_matrix.len(), problem_index);

    let total_button_presses = Int::sum(button_presses.iter());

    optimizer.minimize(&total_button_presses);

    for bp in &button_presses {
        let bp_geq_zero = &bp.ge(0);
        optimizer.assert(bp_geq_zero);
    }

    for (j, &light_activated) in pr.light_map.iter().enumerate() {
        let button_presses_affecting_this_machine: Vec<&Int> = (0..button_machine_matrix.len())
            .filter_map(|i| {
                if button_machine_matrix[i][j] {
                    Some(&button_presses[i])
                } else {
                    None
                }
            })
            .collect();

        let sum_button_presses_equals_light_activated =
            Int::sum(button_presses_affecting_this_machine.into_iter())
                .modulo(2)
                .eq(i32::from(light_activated));

        optimizer.assert(&sum_button_presses_equals_light_activated);
    }

    match optimizer.check(&[]) {
        z3::SatResult::Sat => {
            if let Some(model) = optimizer.get_model()
                && let Some(presses) = model.eval(&total_button_presses, true)
                && let Some(presses) = presses.as_u64()
            {
                presses
            } else {
                panic!("solution extraction went wrong")
            }
        }
        z3::SatResult::Unsat | z3::SatResult::Unknown => {
            panic!("problem not solvable")
        }
    }
}

fn solve(input: &str) -> u64 {
    let problems = parse_input!(parse_problem, input);

    problems
        .iter()
        .enumerate()
        .map(|(i, pr)| min_button_presses(pr, i))
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

    test_with_files!("10", solve, 7, 415);
}
