use std::collections::HashSet;
use std::fmt;

use crate::common::file_helper::read_lines;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    x: i32,
    y: i32,
    direction: Direction,
}

pub fn is_valid(x: i32, y: i32, lines: &Vec<Vec<i32>>) -> bool {
    return x >= 0 && x < lines.len() as i32 && y >= 0 && y < lines[0].len() as i32;
}

impl Direction {
    pub fn from_char(c: char) -> Direction {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid character: {}", c),
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 4,
            Direction::Right => 8,
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "State: x: {}, y: {}, direction: {:?}",
            self.x, self.y, self.direction
        )
    }
}

impl State {
    fn get_direction(&self) -> &Direction {
        return &self.direction;
    }

    fn get_position(&self) -> (i32, i32) {
        return (self.x, self.y);
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => {
                self.x -= 1;
            }
            Direction::Down => {
                self.x += 1;
            }
            Direction::Left => {
                self.y -= 1;
            }
            Direction::Right => {
                self.y += 1;
            }
        }
    }

    fn move_backward(&mut self) {
        match self.direction {
            Direction::Up => {
                self.x += 1;
            }
            Direction::Down => {
                self.x -= 1;
            }
            Direction::Left => {
                self.y += 1;
            }
            Direction::Right => {
                self.y -= 1;
            }
        }
    }

    fn is_guarded(&self, lines: &Vec<Vec<i32>>) -> bool {
        if !is_valid(self.x, self.y, lines) {
            return false;
        }
        return lines[self.x as usize][self.y as usize] == 16;
    }

    fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => {
                self.direction = Direction::Right;
            }
            Direction::Down => {
                self.direction = Direction::Left;
            }
            Direction::Left => {
                self.direction = Direction::Up;
            }
            Direction::Right => {
                self.direction = Direction::Down;
            }
        }
    }

    fn move_next(&mut self, lines: &Vec<Vec<i32>>) {
        if !self.is_guarded(lines) {
            self.move_forward();
        } else {
            self.move_backward();
            self.turn_right();
        }
    }

    fn is_valid(&self, lines: &Vec<Vec<i32>>) -> bool {
        return is_valid(self.x, self.y, lines);
    }
}

pub fn find_start_place(lines: &mut Vec<Vec<i32>>) -> State {
    let xsize = lines.len();
    let ysize = lines[0].len();

    for i in 0..xsize {
        for j in 0..ysize {
            if lines[i][j] == 1 {
                lines[i][j] = 0;
                return State {
                    x: i as i32,
                    y: j as i32,
                    direction: Direction::Up,
                };
            }
        }
    }

    panic!("No start place found");
}

pub fn get(x: i32, y: i32, lines: &Vec<Vec<i32>>) -> i32 {
    if !is_valid(x, y, lines) {
        return 0;
    }
    return lines[x as usize][y as usize];
}

pub fn process_grid_1(grid: &mut Vec<Vec<i32>>) -> usize {
    let mut start_state: State = find_start_place(grid);
    let curr_state = &mut start_state;
    let mut res = 0;
    while curr_state.is_valid(&grid) {
        if !curr_state.is_guarded(&grid) {
            if get(curr_state.x, curr_state.y, grid) != 2 {
                res += 1;
            }
            grid[curr_state.x as usize][curr_state.y as usize] = 2;
        }
        curr_state.move_next(grid);
    }
    return res;
}

pub fn process_grid_with_new_obstacle(
    start_state: &State,
    grid: &Vec<Vec<i32>>,
    x: i32,
    y: i32,
) -> bool {
    let mut states = HashSet::<State>::new();
    let mut curr_state = start_state.clone();
    let mut local_grid = grid.clone();

    while curr_state.is_valid(&local_grid) {
        if !curr_state.is_guarded(&local_grid) {
            let val = get(curr_state.x, curr_state.y, &local_grid);
            let curr_val = curr_state.direction.to_i32();
            if val & curr_val != 0 {
                return true;
            }

            local_grid[curr_state.x as usize][curr_state.y as usize] |= curr_val;
        }
        curr_state.move_next(&local_grid);
    }
    return false;
}

pub fn process_grid_2(grid: &mut Vec<Vec<i32>>) -> i32 {
    let start_state: State = find_start_place(grid);
    let xsize = grid.len() as i32;
    let ysize = grid[0].len() as i32;
    let mut res: i32 = 0;

    for i in 0..xsize {
        for j in 0..ysize {
            if i == start_state.x && j == start_state.y {
                continue;
            }

            if grid[i as usize][j as usize] == 16 {
                continue;
            }

            grid[i as usize][j as usize] = 16;
            let is_loop = process_grid_with_new_obstacle(&start_state, &grid, i, j);
            res += is_loop as i32;
            grid[i as usize][j as usize] = 0;
        }
    }

    return res;
}

pub fn parse_grid(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut grid = Vec::<Vec<i32>>::new();
    for line in lines {
        let mut row = Vec::<i32>::new();
        for c in line.chars() {
            match c {
                '.' => row.push(0),
                '#' => row.push(16),
                '^' => row.push(1),
                _ => panic!("Invalid character: {}", c),
            }
        }
        grid.push(row);
    }
    return grid;
}

pub fn solve_1() -> i32 {
    let lines = read_lines("inputs/day6.txt").unwrap();
    let mut grid = parse_grid(&lines);
    let res = process_grid_1(&mut grid);
    println!("res={}", res);
    return res as i32;
}

pub fn solve_2() -> i32 {
    let lines = read_lines("inputs/day6.txt").unwrap();
    // let lines = read_lines("inputs/day6-1.txt").unwrap();
    let mut grid = parse_grid(&lines);
    let res = process_grid_2(&mut grid);
    println!("res={}", res);
    return res as i32;
}
