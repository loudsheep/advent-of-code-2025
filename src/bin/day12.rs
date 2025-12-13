use std::collections::HashSet;

use advent_of_code_2025::read_lines;
use regex::Regex;

const MAX_SHAPE_DIM: usize = 3;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct BitShape {
    rows: [u64; MAX_SHAPE_DIM],
    width: usize,
    height: usize,
    area: u32,
}

#[derive(Clone, Debug)]
struct Present {
    variants: Vec<BitShape>,
}

struct Region {
    width: i32,
    height: i32,
    presents_to_place: Vec<usize>,
}

struct BitGrid {
    height: usize,
    width: usize,
    rows: Vec<u64>,
}

impl BitGrid {
    fn new(width: usize, height: usize) -> Self {
        BitGrid {
            height,
            width,
            rows: vec![0; height],
        }
    }

    fn can_place(&self, shape: &BitShape, x: usize, y: usize) -> bool {
        if x + shape.width > self.width || y + shape.height > self.height {
            return false;
        }

        if (self.rows[y] & (shape.rows[0] << x)) != 0 {
            return false;
        }
        if shape.height > 1 && (self.rows[y + 1] & (shape.rows[1] << x)) != 0 {
            return false;
        }
        if shape.height > 2 && (self.rows[y + 2] & (shape.rows[2] << x)) != 0 {
            return false;
        }

        true
    }

    fn toggle_shape(&mut self, shape: &BitShape, x: usize, y: usize) {
        self.rows[y] ^= shape.rows[0] << x;
        if shape.height > 1 {
            self.rows[y + 1] ^= shape.rows[1] << x;
        }
        if shape.height > 2 {
            self.rows[y + 2] ^= shape.rows[2] << x;
        }
    }
}

fn parse_to_bitshape(coords: &Vec<(i32, i32)>) -> BitShape {
    if coords.is_empty() {
        return BitShape {
            rows: [0; 3],
            width: 0,
            height: 0,
            area: 0,
        };
    }

    let min_x = coords.iter().map(|c| c.0).min().unwrap();
    let min_y = coords.iter().map(|c| c.1).min().unwrap();

    let mut max_x = 0;
    let mut max_y = 0;

    let mut rows = [0u64; 3];

    for &(cx, cy) in coords {
        let x = (cx - min_x) as usize;
        let y = (cy - min_y) as usize;

        rows[y] |= 1 << x;

        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }

    BitShape {
        rows,
        width: max_x + 1,
        height: max_y + 1,
        area: coords.len() as u32,
    }
}

fn generate_variants(base_coords: &Vec<(i32, i32)>) -> Vec<BitShape> {
    let mut unique_shapes = HashSet::new();
    let mut variants = Vec::new();
    let mut current = base_coords.clone();

    for i in 0..8 {
        let shape = parse_to_bitshape(&current);

        if unique_shapes.insert(shape.clone()) {
            variants.push(shape);
        }

        if i % 4 == 3 {
            current = base_coords.iter().map(|c| (-c.0, c.1)).collect();
        } else {
            current = current.iter().map(|c| (-c.1, c.0)).collect();
        }
    }
    variants
}

fn parse_input(lines: Vec<String>) -> (Vec<Present>, Vec<Region>) {
    // Regex patterns to parse shapes and regions
    // example lines that match this condition: "0:", "1:"
    let re_shape = Regex::new(r"^\d+:$").unwrap();
    // example lines that match this condition: "12x12: 0 1 2", "10x10: 1 2"
    let re_region = Regex::new(r"^(\d+)x(\d+):(( \d+)*)$").unwrap();

    let mut presents: Vec<Present> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    let mut current_shape_coords: Vec<(i32, i32)> = Vec::new();
    let mut parsing_shape = false;
    let mut y = 0;
    for line in lines {
        if line == "" {
            parsing_shape = false;
            y = 0;

            if !current_shape_coords.is_empty() {
                let variants = generate_variants(&current_shape_coords);
                let present = Present { variants };
                presents.push(present);
                current_shape_coords.clear();
            }

            continue;
        }

        if parsing_shape {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    current_shape_coords.push((x as i32, y as i32));
                }
            }
            y += 1;
            continue;
        }

        if re_shape.is_match(&line) {
            parsing_shape = true;
            continue;
        }

        if let Some(caps) = re_region.captures(&line) {
            let width: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let height: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let presents_str = caps.get(3).unwrap().as_str().trim();

            let present_indices: Vec<i32> = presents_str
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let mut all_presents = Vec::new();
            for i in 0..presents.len() {
                for _ in 0..present_indices[i as usize] {
                    all_presents.push(i);
                }
            }

            let region = Region {
                width,
                height,
                presents_to_place: all_presents,
            };
            regions.push(region);
            continue;
        }
    }

    (presents, regions)
}

fn solve(grid: &mut BitGrid, remaining_presents: &[usize], all_presents: &Vec<Present>) -> bool {
    if remaining_presents.is_empty() {
        return true;
    }

    let p_idx = remaining_presents[0];
    let present = &all_presents[p_idx];
    let next_presents = &remaining_presents[1..];

    for y in 0..=(grid.height - 1) {
        for x in 0..=(grid.width - 1) {
            for variant in &present.variants {
                if grid.can_place(variant, x, y) {
                    grid.toggle_shape(variant, x, y);

                    if solve(grid, next_presents, all_presents) {
                        return true;
                    }

                    grid.toggle_shape(variant, x, y);
                }
            }
        }
    }
    false
}

fn main() {
    let lines = read_lines("input/day12.txt").expect("File expected");

    let (presents, regions) = parse_input(lines);
    let mut count_solved = 0;

    for (_, region) in regions.into_iter().enumerate() {
        let mut grid = BitGrid::new(region.width as usize, region.height as usize);

        let presents_to_solve = region.presents_to_place.clone();
        let total_present_area: i32 = presents_to_solve
            .iter()
            .map(|&id| presents[id].variants[0].area as i32)
            .sum();

        if total_present_area > region.width * region.height {
            continue;
        }

        if solve(&mut grid, &region.presents_to_place, &presents) {
            count_solved += 1;
        }
    }

    println!("Number of solvable regions: {}", count_solved);
}
