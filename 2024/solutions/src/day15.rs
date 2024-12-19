use crate::common::file_helper::read_lines;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Mode1,
    Mode2,
}

pub fn find_blank_line_idx(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .position(|x| x.is_empty())
        .unwrap_or(lines.len())
}

pub fn parse_grid(lines: &Vec<String>, blank_line_idx: usize, mode: Mode) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in lines.iter().take(blank_line_idx) {
        if mode == Mode::Mode1 {
            let row = line.chars().collect::<Vec<char>>();
            grid.push(row);
        } else {
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '#' => {
                        row.push('#');
                        row.push('#');
                    }
                    'O' => {
                        row.push('[');
                        row.push(']');
                    }
                    '@' => {
                        row.push('@');
                        row.push('.');
                    }
                    '.' => {
                        row.push('.');
                        row.push('.');
                    }
                    _ => (),
                }
            }
            grid.push(row);
        }
    }
    return grid;
}

pub fn parse_moves(lines: &Vec<String>, blank_line_idx: usize) -> Vec<char> {
    return lines
        .iter()
        .skip(blank_line_idx + 1)
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .chars()
        .collect::<Vec<char>>();
}

pub fn cal_gps_coordinate(grid: &Vec<Vec<char>>, mode: Mode) -> i64 {
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' && mode == Mode::Mode1 || grid[i][j] == '[' && mode == Mode::Mode2
            {
                res += (i as i64) * 100 + j as i64;
            }
        }
    }
    return res;
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
        (curr_x, curr_y) = (curr_x + dx, curr_y + dy);

        if grid[curr_x as usize][curr_y as usize] == '.' {
            find_dot = true;
            break;
        }
    }

    if !find_dot {
        return (x, y);
    }

    while curr_x != x || curr_y != y {
        grid[curr_x as usize][curr_y as usize] =
            grid[(curr_x - dx) as usize][(curr_y - dy) as usize];

        (curr_x, curr_y) = (curr_x - dx, curr_y - dy);
    }
    grid[curr_x as usize][curr_y as usize] = '.';
    grid[(curr_x + dx) as usize][(curr_y + dy) as usize] = '@';
    return (curr_x + dx, curr_y + dy);
}

pub fn check_or_move_bot(
    grid: &mut Vec<Vec<char>>,
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
    check_mode: bool,
) -> bool {
    if !is_valid(grid, x, y) {
        return false;
    }

    if grid[x as usize][y as usize] == '.' {
        return true;
    }

    let mut movable = match grid[(x + dx) as usize][(y + dy) as usize] {
        '[' => check_or_move_bot(grid, x + dx, y + 1, dx, dy, check_mode),
        ']' => check_or_move_bot(grid, x + dx, y - 1, dx, dy, check_mode),
        _ => true, // '.'
    } && check_or_move_bot(grid, x + dx, y + dy, dx, dy, check_mode); // order matters

    if !check_mode {
        grid[(x + dx) as usize][(y + dy) as usize] = grid[x as usize][y as usize];
        grid[x as usize][y as usize] = '.';
    }

    return movable;
}

pub fn move_robot_2(grid: &mut Vec<Vec<char>>, x: i64, y: i64, dx: i64, dy: i64) -> (i64, i64) {
    check_or_move_bot(grid, x, y, dx, dy, true)
        .then(|| {
            check_or_move_bot(grid, x, y, dx, dy, false);
            (x + dx, y + dy)
        })
        .unwrap_or((x, y))
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day15.txt").unwrap();
    let blank_line_idx = find_blank_line_idx(&lines);
    let mut grid = parse_grid(&lines, blank_line_idx, Mode::Mode1);
    let moves = parse_moves(&lines, blank_line_idx);
    let (mut x, mut y) = find_robot(&grid);

    for m in &moves {
        match m {
            '<' => (x, y) = move_robot(&mut grid, x, y, 0, -1),
            '>' => (x, y) = move_robot(&mut grid, x, y, 0, 1),
            'v' => (x, y) = move_robot(&mut grid, x, y, 1, 0),
            '^' => (x, y) = move_robot(&mut grid, x, y, -1, 0),
            _ => (),
        }
    }

    let res = cal_gps_coordinate(&grid, Mode::Mode1);
    println!("res={}", res);
    return res;
}

pub fn solve_2() -> i64 {
    let lines = read_lines("inputs/day15.txt").unwrap();
    let blank_line_idx = find_blank_line_idx(&lines);
    let mut grid = parse_grid(&lines, blank_line_idx, Mode::Mode2);
    let moves = parse_moves(&lines, blank_line_idx);
    let (mut x, mut y) = find_robot(&grid);

    for m in &moves {
        match m {
            '<' => (x, y) = move_robot(&mut grid, x, y, 0, -1),
            '>' => (x, y) = move_robot(&mut grid, x, y, 0, 1),
            'v' => (x, y) = move_robot_2(&mut grid, x, y, 1, 0),
            '^' => (x, y) = move_robot_2(&mut grid, x, y, -1, 0),
            _ => (),
        }
    }

    let res = cal_gps_coordinate(&grid, Mode::Mode2);
    println!("res={}", res);
    return res;
}
