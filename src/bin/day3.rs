use advent_of_code_2025::read_lines;

fn find_largest_digit_in_range(input: &str, start: usize, end: usize) -> Option<(usize, char)> {
    let mut max_digit = None;
    let mut max_index = None;

    for (i, c) in input.chars().enumerate().take(end).skip(start) {
        if c.is_digit(10) {
            if max_digit.is_none() || c > max_digit.unwrap() {
                max_digit = Some(c);
                max_index = Some(i);
            }
        }
    }
    if let (Some(index), Some(digit)) = (max_index, max_digit) {
        Some((index, digit))
    } else {
        None
    }
}

fn main() {
    let banks = read_lines("input/day3.txt").expect("Expected correct input");
    let mut sum = 0;
    for bank in banks {
        let (idx, chr) = find_largest_digit_in_range(&bank, 0, bank.len() - 11)
            .expect("Expected at least one digit in the bank string");

        let mut result = vec![chr];
        let mut last_index = idx;

        for i in 1..12 {
            let (index, digit) =
                find_largest_digit_in_range(&bank, last_index + 1, bank.len() - (11 - i))
                    .expect("Expected at least one digit in the bank string");
            result.push(digit);
            last_index = index;
        }

        let number_str: String = result.into_iter().collect();
        let number = number_str.parse::<i64>().expect("Expected valid number");
        sum += number;
    }
    println!("Sum: {}", sum);
}
