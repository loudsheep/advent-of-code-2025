use advent_of_code_2025::read_lines;
use regex::Regex;

const SHAPE_SIZE_X: i32 = 3;
const SHAPE_SIZE_Y: i32 = 3;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Shape {
    coords: Vec<Coord>,
    area: i32,
}

#[derive(Clone)]
struct Present {
    shape_variants: Vec<Shape>,
}

struct Region {
    width: i32,
    height: i32,
    presents_to_place: Vec<usize>,
}

struct FastGrid {
    width: i32,
    height: i32,
    cells: Vec<bool>,
}

impl FastGrid {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            cells: vec![false; (width * height) as usize],
        }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        self.cells[(y * self.width + x) as usize]
    }

    fn set(&mut self, x: i32, y: i32, val: bool) {
        self.cells[(y * self.width + x) as usize] = val;
    }

    fn can_fit(&self, shape: &Shape, start_x: i32, start_y: i32) -> bool {
        if start_x + SHAPE_SIZE_X > self.width || start_y + SHAPE_SIZE_Y > self.height {
            return false;
        }

        for offset in &shape.coords {
            if self.get(start_x + offset.x, start_y + offset.y) {
                return false;
            }
        }
        true
    }

    fn place_shape(&mut self, shape: &Shape, start_x: i32, start_y: i32, val: bool) {
        for offset in &shape.coords {
            self.set(start_x + offset.x, start_y + offset.y, val);
        }
    }
}

fn calculate_present_variants(shape: &Shape) -> Vec<Shape> {
    let mut variants = Vec::new();

    // Original shape
    variants.push(Shape {
        coords: shape.coords.clone(),
        area: shape.area,
    });

    // Rotated shapes (90, 180, 270 degrees), but keep it relative to origin (top-left corner of shape)
    for _ in 0..3 {
        let last_shape = variants.last().unwrap();
        let rotated_coords: Vec<Coord> = last_shape
            .coords
            .iter()
            .map(|coord| Coord {
                x: SHAPE_SIZE_Y - 1 - coord.y,
                y: coord.x,
            })
            .collect();
        variants.push(Shape {
            coords: rotated_coords,
            area: shape.area,
        });
    }

    // Flipped shapes (horizontal flip)
    variants.push(Shape {
        coords: shape
            .coords
            .iter()
            .map(|coord| Coord {
                x: SHAPE_SIZE_X - 1 - coord.x,
                y: coord.y,
            })
            .collect(),
        area: shape.area,
    });

    // Rotated flipped shapes (90, 180, 270 degrees)
    for _ in 0..3 {
        let last_shape = variants.last().unwrap();
        let rotated_coords: Vec<Coord> = last_shape
            .coords
            .iter()
            .map(|coord| Coord {
                x: SHAPE_SIZE_Y - 1 - coord.y,
                y: coord.x,
            })
            .collect();
        variants.push(Shape {
            coords: rotated_coords,
            area: shape.area,
        });
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

    let mut current_shape_coords: Vec<Coord> = Vec::new();
    let mut parsing_shape = false;
    let mut y = 0;
    for line in lines {
        if line == "" {
            parsing_shape = false;
            y = 0;

            if !current_shape_coords.is_empty() {
                let shape = Shape {
                    coords: current_shape_coords.clone(),
                    area: current_shape_coords.len() as i32,
                };
                let variants = calculate_present_variants(&shape);
                let present = Present {
                    shape_variants: variants,
                };
                current_shape_coords.clear();
                presents.push(present);
            }

            continue;
        }

        if parsing_shape {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    current_shape_coords.push(Coord {
                        x: x as i32,
                        y: y as i32,
                    });
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

fn solve_region(
    grid: &mut FastGrid,
    remaining_presents: &[usize],
    all_presents: &Vec<Present>,
) -> bool {
    if remaining_presents.is_empty() {
        return true;
    }

    let current_id = remaining_presents[0];
    let next_presents = &remaining_presents[1..];

    let present = &all_presents[current_id];

    for r in 0..grid.height {
        for c in 0..grid.width {
            for shape_variant in &present.shape_variants {
                if grid.can_fit(shape_variant, c, r) {
                    grid.place_shape(shape_variant, c, r, true);

                    if solve_region(grid, next_presents, all_presents) {
                        return true;
                    }

                    grid.place_shape(shape_variant, c, r, false);
                }
            }
        }
    }

    false
}

fn main() {
    let lines = read_lines("input/day12_example.txt").expect("File expected");

    let (presents, regions) = parse_input(lines);
    let mut count_solved = 0;

    for (_, region) in regions.into_iter().enumerate() {
        let mut grid = FastGrid::new(region.width, region.height);

        let presents_to_solve = region.presents_to_place.clone();
        let total_present_area: i32 = presents_to_solve
            .iter()
            .map(|&id| presents[id].shape_variants[0].area)
            .sum();

        if total_present_area > region.width * region.height {
            continue;
        }

        if solve_region(&mut grid, &region.presents_to_place, &presents) {
            count_solved += 1;
        }
    }

    println!("Number of solvable regions: {}", count_solved);
}
