use std::collections::HashSet;

use advent_of_code_2025::read_lines;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    buttons: Vec<Vec<usize>>,
    target_joltage: Vec<i64>,
}

// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
fn parse_input(lines: Vec<String>) -> Vec<Machine> {
    let re_buttons = Regex::new(r"\(([\d,]+)\)").unwrap();
    let re_joltage = Regex::new(r"\{([\d,]+)\}").unwrap();

    lines
        .iter()
        .map(|line| {
            let buttons: Vec<Vec<usize>> = re_buttons
                .captures_iter(line)
                .map(|cap| {
                    cap[1]
                        .split(',')
                        .map(|s| s.trim().parse::<usize>().expect("Invalid button index"))
                        .collect()
                })
                .collect();

            let target_joltage: Vec<i64> = if let Some(cap) = re_joltage.captures(line) {
                cap[1]
                    .split(',')
                    .map(|s| s.trim().parse::<i64>().expect("Invalid joltage"))
                    .collect()
            } else {
                Vec::new()
            };

            Machine {
                buttons,
                target_joltage,
            }
        })
        .collect()
}

fn solve_machine(machine: &Machine) -> Option<usize> {
    let num_rows = machine.target_joltage.len();
    let num_cols = machine.buttons.len();

    type Rat = (i64, i64);

    fn simplify((n, d): Rat) -> Rat {
        if d == 0 {
            return (0, 0);
        }
        let common = gcd(n.abs(), d.abs());
        let (n, d) = (n / common, d / common);
        if d < 0 { (-n, -d) } else { (n, d) }
    }

    fn sub((n1, d1): Rat, (n2, d2): Rat) -> Rat {
        simplify((n1 * d2 - n2 * d1, d1 * d2))
    }

    fn mul((n1, d1): Rat, (n2, d2): Rat) -> Rat {
        simplify((n1 * n2, d1 * d2))
    }

    fn div((n1, d1): Rat, (n2, d2): Rat) -> Rat {
        simplify((n1 * d2, d1 * n2))
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    let mut matrix = vec![vec![(0, 1); num_cols + 1]; num_rows];

    for r in 0..num_rows {
        for c in 0..num_cols {
            if machine.buttons[c].contains(&r) {
                matrix[r][c] = (1, 1);
            }
        }
        matrix[r][num_cols] = (machine.target_joltage[r], 1);
    }

    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();

    for col in 0..num_cols {
        if pivot_row >= num_rows {
            break;
        }

        let mut pivot_cand = None;
        for r in pivot_row..num_rows {
            if matrix[r][col].0 != 0 {
                pivot_cand = Some(r);
                break;
            }
        }

        if let Some(r) = pivot_cand {
            matrix.swap(pivot_row, r);

            let pivot_val = matrix[pivot_row][col];
            for k in col..=num_cols {
                matrix[pivot_row][k] = div(matrix[pivot_row][k], pivot_val);
            }

            for i in 0..num_rows {
                if i != pivot_row {
                    let factor = matrix[i][col];
                    if factor.0 != 0 {
                        for k in col..=num_cols {
                            matrix[i][k] = sub(matrix[i][k], mul(factor, matrix[pivot_row][k]));
                        }
                    }
                }
            }
            pivot_cols.push(col);
            pivot_row += 1;
        }
    }

    let pivot_set: HashSet<_> = pivot_cols.iter().cloned().collect();
    let free_cols: Vec<_> = (0..num_cols).filter(|c| !pivot_set.contains(c)).collect();

    let mut min_total_presses = usize::MAX;

    let search_limit = machine.target_joltage.iter().cloned().max().unwrap_or(0) as usize;

    let mut assignment = vec![0i64; num_cols];

    fn search(
        idx: usize,
        free_cols: &Vec<usize>,
        limit: usize,
        assignment: &mut Vec<i64>,
        matrix: &Vec<Vec<Rat>>,
        pivot_cols: &Vec<usize>,
        min_presses: &mut usize,
    ) {
        if idx == free_cols.len() {
            let mut valid = true;

            for (r_idx, &p_col) in pivot_cols.iter().enumerate() {
                let (mut tn, mut td) = matrix[r_idx][matrix[0].len() - 1];

                for &f_col in free_cols {
                    let (cn, cd) = matrix[r_idx][f_col];

                    let sub_n = cn * assignment[f_col];
                    let sub_d = cd;

                    let new_n = tn * sub_d - sub_n * td;
                    let new_d = td * sub_d;
                    tn = new_n;
                    td = new_d;
                }

                if td == 0 || tn % td != 0 {
                    valid = false;
                    break;
                }
                let val = tn / td;

                if val < 0 {
                    valid = false;
                    break;
                }
                assignment[p_col] = val;
            }

            if valid {
                for r in pivot_cols.len()..matrix.len() {
                    let (tn, _) = matrix[r][matrix[0].len() - 1];
                    if tn != 0 {
                        return;
                    }
                }

                let total: i64 = assignment.iter().sum();
                if (total as usize) < *min_presses {
                    *min_presses = total as usize;
                }
            }
            return;
        }

        let col = free_cols[idx];
        for val in 0..=limit {
            assignment[col] = val as i64;
            search(
                idx + 1,
                free_cols,
                limit,
                assignment,
                matrix,
                pivot_cols,
                min_presses,
            );
        }
    }

    search(
        0,
        &free_cols,
        search_limit,
        &mut assignment,
        &matrix,
        &pivot_cols,
        &mut min_total_presses,
    );

    if min_total_presses == usize::MAX {
        None
    } else {
        Some(min_total_presses)
    }
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
