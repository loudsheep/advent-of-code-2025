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

fn main() {
    let lines = read_lines("input/day1.txt").expect("Could not read lines from file");
    let result = solve(&lines);
    println!("Password: {}", result);
}
