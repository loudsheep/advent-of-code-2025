use advent_of_code_2025::read_lines;

fn solve(lines: &[String]) -> i32 {
    let mut current_pos = 50;
    let mut password = 0;
    for line in lines {
        if line.starts_with("L") {
            let change = line[1..].parse::<i32>().unwrap_or(0);
            current_pos -= change;
        } else {
            let change = line[1..].parse::<i32>().unwrap_or(0);
            current_pos += change;
        }

        current_pos = current_pos % 100;

        if current_pos == 0 {
            password += 1;
        }
    }
    password
}

fn solve_method_0x434c49434b(lines: &[String]) -> i64 {
    let mut current_abs_pos: i64 = 50;
    let mut password = 0;
    for line in lines {
        let (direction, change_str) = line.split_at(1);
        let change = change_str.parse::<i64>().unwrap_or(0);
        
        let old_pos = current_abs_pos;

        match direction {
            "L" => {
                current_abs_pos -= change;
                let count = ((old_pos - 1) as f64 / 100.0).floor() as i64 
                          - ((current_abs_pos - 1) as f64 / 100.0).floor() as i64;
                password += count;
            }
            "R" => {
                current_abs_pos += change;
                let count = (current_abs_pos as f64 / 100.0).floor() as i64 
                          - (old_pos as f64 / 100.0).floor() as i64;
                password += count;
            }
            _ => {}
        }
    }
    password
}

fn main() {
    let lines = read_lines("input/day1.txt").expect("Could not read lines from file");
    let result = solve(&lines);
    println!("Password: {}", result);

    let result_method_0x434c49434b = solve_method_0x434c49434b(&lines);
    println!("Password (method 0x434c49434b): {}", result_method_0x434c49434b);
}
