use std::collections::HashSet;

use advent_of_code_2025::read_lines;

fn is_line_empty(line: &String) -> bool {
    line.chars().all(|c| c == '.')
}

fn count_splits(lines: Vec<String>) -> usize {
    let mut count = 0;
    let mut beams = HashSet::new();

    for line in lines {
        if is_line_empty(&line) {
            continue;
        }

        for (i, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if c == '^' && beams.contains(&i) {
                beams.remove(&i);

                beams.insert(i + 1);
                beams.insert(i - 1);
                count += 1;
            } else if c == 'S' {
                beams.insert(i);
            }
        }
    }
    count
}

fn main() {
    let lines = read_lines("input/day7.txt").expect("EXpected file");
    
    let result = count_splits(lines);
    println!("Result: {}", result);
}