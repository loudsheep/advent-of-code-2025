use std::collections::HashSet;

use advent_of_code_2025::read_lines;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    target_lights: Vec<u8>,
    buttons: Vec<Vec<usize>>,
}

fn parse_input(lines: Vec<String>) -> Vec<Machine> {
    let re_diagram = Regex::new(r"\[([.#]+)\]").unwrap();
    let re_buttons = Regex::new(r"\(([\d,]+)\)").unwrap();

    lines.iter().map(|line| {
        let caps = re_diagram.captures(line).expect("Invalid diagram");
        let target_lights: Vec<u8> = caps[1].chars().map(|c| if c == '#' { 1 } else { 0 }).collect();

        let buttons: Vec<Vec<usize>> = re_buttons.captures_iter(line)
            .map(|cap| {
                cap[1].split(',')
                    .map(|s| s.trim().parse::<usize>().expect("Invalid button index"))
                    .collect()
            })
            .collect();

        Machine { target_lights, buttons }
    }).collect()
}

fn solve_machine(machine: &Machine) -> Option<usize> {
    let num_rows = machine.target_lights.len();
    let num_cols = machine.buttons.len();

    let mut matrix = vec![vec![0u8; num_cols + 1]; num_rows];

    for r in 0..num_rows {
        for c in 0..num_cols {
            if machine.buttons[c].contains(&r) {
                matrix[r][c] = 1;
            }
            matrix[r][num_cols] = machine.target_lights[r];
        }
    }

    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();

    for col in 0..num_cols {
        // break if all rows are processed
        if pivot_row >= num_rows {
            break;
        }

        if let Some(row) = (pivot_row..num_rows).find(|&r| matrix[r][col] == 1) {
            matrix.swap(pivot_row, row);

            for i in 0..num_rows {
                if i != pivot_row && matrix[i][col] == 1 {
                    for j in col..=num_cols {
                        matrix[i][j] ^= matrix[pivot_row][j];
                    }
                }
            }
            pivot_cols.push(col);
            pivot_row += 1;
        }
    }

    for row in &matrix {
        if row[..num_cols].iter().all(|&x| x == 0) && row[num_cols] == 1 {
            // impossible case
            return None;
        }
    }

    let pivot_set: HashSet<_> = pivot_cols.iter().cloned().collect();
    let free_cols: Vec<_> = (0..num_cols).filter(|c| !pivot_set.contains(c)).collect();

    let mut min_presses = usize::MAX;

    // test all combinations of free variables (2^n)
    let possible_combinations = 1 << free_cols.len();
    for i in 0..possible_combinations {
        let mut solution = vec![0u8; num_cols];

        for (idx, &col) in free_cols.iter().enumerate() {
            // use bits of i to set free variables
            solution[col] = ((i >> idx) & 1) as u8;
        }

        for (r, &pivot_col) in pivot_cols.iter().enumerate() {
            let row = &matrix[r];
            let mut val = row[num_cols];

            for &fc in &free_cols {
                if row[fc] == 1 {
                    val ^= solution[fc];
                }
            }
            solution[pivot_col] = val;
        }

        let presses = solution.iter().map(|&x| x as usize).sum();
        if presses < min_presses {
            min_presses = presses;
        }
    }

    Some(min_presses)
}

fn main() {
    let lines = read_lines("input/day10.txt").expect("Input expected");

    let machines = parse_input(lines);
    let mut total_presses = 0;
    for machine in &machines {
        if let Some(presses) = solve_machine(machine) {
            total_presses += presses;
        } else {
            println!("No solution for machine: {:?}", machine);
        }
    }
    println!("Total button presses: {}", total_presses);
}
