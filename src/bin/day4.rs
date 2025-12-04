use advent_of_code_2025::read_lines;

fn get_cell(grid: &Vec<Vec<char>>, row: isize, col: isize) -> char {
    if row < 0 || row >= grid.len() as isize {
        return '.';
    }
    if col < 0 || col >= grid[row as usize].len() as isize {
        return '.';
    }
    grid[row as usize][col as usize]
}

fn count_neighbors(grid: &Vec<Vec<char>>, row: isize, col: isize) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;
    for (dr, dc) in directions.iter() {
        if get_cell(grid, row + dr, col + dc) == '@' {
            count += 1;
        }
    }
    count
}

fn count_movable_rolls(grid: &mut Vec<Vec<char>>) -> usize {
    let mut count = 0;

    let mut newgrid = grid.clone();
    
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                let neighbors = count_neighbors(grid, i as isize, j as isize);
                if neighbors < 4 {
                    newgrid[i][j] = '.';
                    count += 1;
                }
            }
        }
    }

    *grid = newgrid;
    count
}

fn main() {
    let lines = read_lines("input/day4.txt").expect("Expected a grid");
    let mut grid = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    let mut total_count = 0;
    loop {
        let movable_rolls = count_movable_rolls(&mut grid);
        if movable_rolls == 0 {
            break;
        }
        total_count += movable_rolls;
    }
    println!("Number of movable rolls: {}", total_count);
}
