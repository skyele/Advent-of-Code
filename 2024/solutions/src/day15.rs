use crate::common::file_helper::read_lines;

pub fn find_blank_line_idx(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .position(|x| x.is_empty())
        .unwrap_or(lines.len())
}

pub fn parse_grid(lines: &Vec<String>, blank_line_idx: usize) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in lines.iter().take(blank_line_idx) {
        let row = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }
    return grid;
}

pub fn find_robot(grid: &Vec<Vec<char>>) -> (i64, i64) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                return (i as i64, j as i64);
            }
        }
    }
    return (-1, -1);
}

pub fn is_valid(grid: &Vec<Vec<char>>, x: i64, y: i64) -> bool {
    return x >= 0
        && x < grid.len() as i64
        && y >= 0
        && y < grid[0].len() as i64
        && grid[x as usize][y as usize] != '#';
}

pub fn move_robot(grid: &mut Vec<Vec<char>>, x: i64, y: i64, dx: i64, dy: i64) -> (i64, i64) {
    let (mut curr_x, mut curr_y) = (x, y);
    let mut find_dot = false;
    while is_valid(grid, curr_x + dx, curr_y + dy) {
        curr_x += dx;
        curr_y += dy;
        // println!("-> curr_x={}, curr_y={}", curr_x, curr_y);

        if grid[curr_x as usize][curr_y as usize] == '.' {
            find_dot = true;
            break;
        }
    }

    // println!("curr_x={}, curr_y={}", curr_x, curr_y);

    if !find_dot {
        return (x, y);
    }

    while curr_x != x || curr_y != y {
        grid[curr_x as usize][curr_y as usize] = 'O';
        curr_x -= dx;
        curr_y -= dy;
        // println!("<- curr_x={}, curr_y={}", curr_x, curr_y);
    }
    grid[curr_x as usize][curr_y as usize] = '.';
    grid[(curr_x + dx) as usize][(curr_y + dy) as usize] = '@';
    return (curr_x + dx, curr_y + dy);
}

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn cal_gps_coordinate(grid: &Vec<Vec<char>>) -> i64 {
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                res += (i as i64) * 100 + j as i64;
            }
        }
    }
    return res;
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day15.txt").unwrap();
    let blank_line_idx = find_blank_line_idx(&lines);
    let mut grid = parse_grid(&lines, blank_line_idx);
    let moves = lines
        .iter()
        .skip(blank_line_idx + 1)
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .chars()
        .collect::<Vec<char>>();
    let (mut x, mut y) = find_robot(&grid);

    for m in &moves {
        if m == &'<' {
            (x, y) = move_robot(&mut grid, x, y, 0, -1);
        } else if m == &'>' {
            (x, y) = move_robot(&mut grid, x, y, 0, 1);
        } else if m == &'v' {
            (x, y) = move_robot(&mut grid, x, y, 1, 0);
        } else if m == &'^' {
            (x, y) = move_robot(&mut grid, x, y, -1, 0);
        }
    }

    let res = cal_gps_coordinate(&grid);
    println!("res={}", res);
    return res;
}
