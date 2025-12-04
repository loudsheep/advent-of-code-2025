use advent_of_code_2025::read_lines;

fn find_first_largest_digit_and_position_except_last(input: &str, start: usize, end: usize) -> Option<(usize, char)> {
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
        let (index1, digit1) = find_first_largest_digit_and_position_except_last(&bank, 0, bank.len() - 1)
            .expect("Expected at least one digit in the bank string");

        let (_, digit2) = find_first_largest_digit_and_position_except_last(&bank, index1 + 1, bank.len())
            .expect("Expected at least one more digit in the bank string after the first largest digit");

        sum += digit1.to_digit(10).unwrap() * 10 + digit2.to_digit(10).unwrap();
    }
    println!("Sum: {}", sum);
}