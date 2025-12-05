use advent_of_code_2025::read_lines;

fn parse_range(range: &str) -> (u64, u64) {
    let mut parts = range.split('-');
    let start = parts
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .expect("Expected start of range");
    let end = parts
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .expect("Expected end of range");
    (start, end)
}

fn is_in_range(num: u64, start: u64, end: u64) -> bool {
    if num < start || num > end {
        return false
    }
    true
}

fn count_valid_ingredients(
    ranges: &[&str],
    ingredients: &[&str],
) -> u64 {
    let parsed_ranges: Vec<(u64, u64)> = ranges.iter().map(|r| parse_range(r)).collect();
    let mut valid_count = 0;
    for ingredient in ingredients {
        let num = ingredient
            .parse::<u64>()
            .expect("Expected ingredient to be a number");
        if parsed_ranges.iter().any(|(start, end)| is_in_range(num, *start, *end)) {
            valid_count += 1;
        }
    }
    valid_count
}

fn collapse_overlapping_ranges(
    ranges: &[(u64, u64)],
) -> Vec<(u64, u64)> {
    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort_by_key(|r| r.0);
    let mut collapsed: Vec<(u64, u64)> = Vec::new();

    for range in sorted_ranges {
        if let Some(last) = collapsed.last_mut() {
            // if current range starts before or just after the last range ends, merge them
            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
                continue;
            }
        }
        collapsed.push(range);
    }
    collapsed
}

fn count_all_ingriedients_in_ranges(collapsed: Vec<(u64, u64)>) -> u64 {
    collapsed.iter().map(|(start, end)| end - start + 1).sum()
}

fn main() {
    let lines = read_lines("input/day5.txt").expect("Expected input");
    let blank_line_index = lines
        .iter()
        .position(|line| line.is_empty())
        .expect("Expected blank line");
    let lines_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    let (ranges, ingredients) = (&lines_refs[..blank_line_index], &lines_refs[blank_line_index + 1..]);

    let valid_ingredient_count = count_valid_ingredients(ranges, ingredients);
    println!("Number of valid ingredients: {}", valid_ingredient_count);

    let parsed_ranges: Vec<(u64, u64)> = ranges.iter().map(|r| parse_range(r)).collect();
    let collapsed_ranges = collapse_overlapping_ranges(&parsed_ranges);
    let total_ingredient_count = count_all_ingriedients_in_ranges(collapsed_ranges);
    println!("Total number of ingredients in ranges: {}", total_ingredient_count);
}
