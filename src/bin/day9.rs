use advent_of_code_2025::{parse_csv_line, read_lines};

fn find_largest_rectangle_area(positions: &Vec<Vec<String>>) -> i64 {
    let mut max_area = 0;
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let (x1, y1) = (
                positions[i][0].parse::<i64>().unwrap(),
                positions[i][1].parse::<i64>().unwrap(),
            );

            let (x2, y2) = (
                positions[j][0].parse::<i64>().unwrap(),
                positions[j][1].parse::<i64>().unwrap(),
            );

            
            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            max_area = max_area.max(area);
        }
    }
    max_area
}

fn main() {
    let lines = read_lines("input/day9.txt")
        .expect("Could not read input file")
        .iter()
        .map(|line| parse_csv_line(line))
        .collect::<Vec<_>>();

    let largest_area = find_largest_rectangle_area(&lines);
    println!("Largest rectangle area: {}", largest_area);
}
