use std::collections::HashSet;

use advent_of_code_2025::read_lines;

fn is_line_empty(line: &String) -> bool {
    line.chars().all(|c| c == '.')
}

fn count_splits(lines: &Vec<String>) -> usize {
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

fn count_timelines(lines: &Vec<String>) -> usize {
    let mut timelines = vec![0; lines[0].len()];
    
    for line in lines {
        if is_line_empty(&line) {
            continue;
        }

        for (i, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if c == '^' && timelines[i] > 0 {
                timelines[i + 1] += timelines[i];
                timelines[i - 1] += timelines[i];

                timelines[i] = 0;
            } else if c == 'S' {
                timelines[i] = 1;
            }
        }
    }

    timelines.iter().sum()
}

fn main() {
    let lines = read_lines("input/day7.txt").expect("EXpected file");
    
    let result = count_splits(&lines);
    println!("Splits: {}", result);

    let result2 = count_timelines(&lines);
    println!("Timelines: {}", result2);
}