use advent_of_code_2025::read_lines;

fn is_column_empty(lines: &Vec<String>, col: usize) -> bool {
    for row in lines {
        if let Some(c) = row.chars().nth(col) {
            if c != ' ' {
                return false;
            }
        }
    }
    true
}

fn solve_colums(lines: &Vec<String>) -> usize {
    let line_len = lines.first().unwrap().len();
    let num_problems_in_column = lines.len() - 1;

    let mut total_sum = 0;
    let mut current_nums = vec![];
    let mut current_operation = '+';

    for col in 0..line_len {
        if is_column_empty(lines, col) {
            if current_operation == '+' {
                let column_sum: usize = current_nums.iter().sum();
                total_sum += column_sum;
            } else if current_operation == '*' {
                let column_product: usize = current_nums.iter().product();
                total_sum += column_product;
            }

            current_nums.clear();
            current_operation = '+';
            continue;
        }

        let mut power = 1;
        let mut column_value = 0;
        for row in (0..num_problems_in_column).rev() {
            if let Some(c) = lines[row].chars().nth(col) {
                if c != ' ' {
                    let digit = c.to_digit(10).unwrap() as usize;
                    column_value += digit * power;
                    power *= 10;
                }
            }
        }

        current_nums.push(column_value);

        if lines.last().unwrap().chars().nth(col) == Some('+') {
            current_operation = '+';
        } else if lines.last().unwrap().chars().nth(col) == Some('*') {
            current_operation = '*';
        }
    }

    if current_operation == '+' {
        let column_sum: usize = current_nums.iter().sum();
        total_sum += column_sum;
    } else if current_operation == '*' {
        let column_product: usize = current_nums.iter().product();
        total_sum += column_product;
    }

    total_sum
}

fn main() {
    let lines = read_lines("input/day6.txt")
        .expect("Expected file")
        .iter()
        .map(|line| line.chars().rev().collect())
        .collect::<Vec<String>>();

    let result = solve_colums(&lines);
    println!("Result: {}", result);
}
