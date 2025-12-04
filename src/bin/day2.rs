use advent_of_code_2025::{parse_csv_line, read_line};

fn parse_range(s: &str) -> (i64, i64) {
    let parts: Vec<&str> = s.split('-').collect();
    let start = parts[0].parse::<i64>().unwrap();
    let end = parts[1].parse::<i64>().unwrap();
    (start, end)
}

fn is_invalid_id(id: i64) -> bool {
    let id_str = id.to_string();

    for i in 1..(id_str.len() / 2 + 1) {
        if id_str.len() % i != 0 {
            continue;
        }

        let segment = &id_str[0..i];
        let mut repeated = String::new();
        for _ in 0..(id_str.len() / i) {
            repeated.push_str(segment);
        }
        if repeated == id_str {
            return true;
        }
    }

    false
}

fn check_ids_in_range(start: i64, end: i64) -> i64 {
    let mut invalid_sum = 0;
    for id in start..=end {
        if is_invalid_id(id) {
            invalid_sum += id;
        }
    }
    invalid_sum
}

fn main() {
    let line = read_line("input/day2.txt").expect("File must exist and have one line with ranges");
    let ranges = parse_csv_line(&line);

    let mut total_invalid_sum = 0;
    for range_str in ranges {
        let (start, end) = parse_range(&range_str);
        let invalid_sum = check_ids_in_range(start, end);
        total_invalid_sum += invalid_sum;
    }
    println!("Total invalid sum: {}", total_invalid_sum);
}
