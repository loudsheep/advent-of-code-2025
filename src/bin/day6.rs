use advent_of_code_2025::{read_lines, split_by_whitespaces};

fn solve_colum(problems: &Vec<Vec<String>>, col: usize) -> usize {
    let operation = problems.last().unwrap()[col].as_str();
    match operation {
        "+" => {
            let mut sum = 0;
            for i in 0..(problems.len() - 1) {
                sum += problems[i][col].parse::<usize>().unwrap();
            }
            sum
        },
        "*" => {
            let mut product = 1;
            for i in 0..(problems.len() - 1) {
                product *= problems[i][col].parse::<usize>().unwrap();
            }
            product
        },
        _ => panic!("Unknown operation"),
    }
}

fn sum_of_problems(problems: &Vec<Vec<String>>) -> usize {
    let mut total_sum = 0;
    let num_columns = problems[0].len();
    for col in 0..num_columns {
        total_sum += solve_colum(problems, col);
    }
    total_sum
}

fn main() {
    let lines = read_lines("input/day6.txt").expect("Expected file");
    let problems = lines.iter().map(|l| split_by_whitespaces(l)).collect::<Vec<_>>();

    println!("{}", sum_of_problems(&problems));
}