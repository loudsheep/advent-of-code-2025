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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn find_largest_rectangle_inside_polygon(positions: &Vec<Vec<String>>) -> i64 {
    let vertices: Vec<Point> = positions
        .iter()
        .map(|p| Point {
            x: p[0].parse().unwrap(),
            y: p[1].parse().unwrap(),
        })
        .collect();

    let n = vertices.len();

    let mut edges = Vec::new();
    for i in 0..n {
        edges.push((vertices[i], vertices[(i + 1) % n]));
    }

    let mut max_area = 0;
    
    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = vertices[i];
            let p2 = vertices[j];

            if p1.x == p2.x || p1.y == p2.y {
                continue;
            }

            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            let width = max_x - min_x + 1;
            let height = max_y - min_y + 1;
            let current_area = width * height;

            // dont need to check smaller areas
            if current_area <= max_area {
                continue;
            }

            if is_valid_rectangle(min_x, max_x, min_y, max_y, &edges) {
                max_area = current_area;
            }
        }
    }

    max_area
}

fn is_valid_rectangle(
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    edges: &Vec<(Point, Point)>,
) -> bool {
    for &(v1, v2) in edges {
        if v1.x == v2.x {
            if v1.x > min_x && v1.x < max_x {
                let (e_min_y, e_max_y) = if v1.y < v2.y { (v1.y, v2.y) } else { (v2.y, v1.y) };
                let overlap_start = e_min_y.max(min_y);
                let overlap_end = e_max_y.min(max_y);
                if overlap_start < overlap_end {
                    return false;
                }
            }
        }
        else if v1.y == v2.y {
            if v1.y > min_y && v1.y < max_y {
                let (e_min_x, e_max_x) = if v1.x < v2.x { (v1.x, v2.x) } else { (v2.x, v1.x) };
                let overlap_start = e_min_x.max(min_x);
                let overlap_end = e_max_x.min(max_x);
                if overlap_start < overlap_end {
                    return false;
                }
            }
        }
    }

    let test_x2 = min_x + max_x;
    let test_y2 = min_y + max_y;

    let mut inside = false;
    for &(v1, v2) in edges {
        let v1x2 = v1.x * 2;
        let v1y2 = v1.y * 2;
        let v2x2 = v2.x * 2;
        let v2y2 = v2.y * 2;

        // some raycasting
        if (v1y2 > test_y2) != (v2y2 > test_y2) {
            let intersect_x =
                v1x2 as f64 + (test_y2 - v1y2) as f64 * (v2x2 - v1x2) as f64 / (v2y2 - v1y2) as f64;

            if intersect_x > test_x2 as f64 {
                inside = !inside;
            }
        }
    }

    inside
}

fn main() {
    let lines = read_lines("input/day9.txt")
        .expect("Could not read input file")
        .iter()
        .map(|line| parse_csv_line(line))
        .collect::<Vec<_>>();

    let largest_area = find_largest_rectangle_area(&lines);
    println!("Largest rectangle area: {}", largest_area);

    let largest_polygon_area = find_largest_rectangle_inside_polygon(&lines);
    println!(
        "Largest rectangle area inside polygon: {}",
        largest_polygon_area
    );
}
