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

pub fn is_valid(x: i32, y: i32, lines: &Vec<String>) -> bool {
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

    fn is_guarded(&self, lines: &Vec<String>) -> bool {
        if !is_valid(self.x, self.y, lines) {
            return false;
        }
        return lines[self.x as usize].chars().nth(self.y as usize).unwrap() == '#';
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

    fn move_next(&mut self, lines: &Vec<String>, extra_x: i32, extra_y: i32) {
        self.move_forward();
        if !self.is_guarded(lines) && !(self.x == extra_x && self.y == extra_y) {
            return;
        }

        self.move_backward();
        self.turn_right();
        self.move_forward();

        if !self.is_guarded(lines) && !(self.x == extra_x && self.y == extra_y) {
            return;
        }
        self.move_backward();
    }

    fn is_valid(&self, lines: &Vec<String>) -> bool {
        return is_valid(self.x, self.y, lines);
    }
}

pub fn find_start_place(lines: &Vec<String>) -> State {
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                return State {
                    x: i as i32,
                    y: j as i32,
                    direction: Direction::from_char(c),
                };
            }
        }
    }

    panic!("No start place found");
}

pub fn process_grid_1(lines: &Vec<String>) -> usize {
    let mut start_state: State = find_start_place(&lines);
    let mut places = HashSet::<(i32, i32)>::new();
    let curr_state = &mut start_state;
    while curr_state.is_valid(&lines) {
        places.insert(curr_state.get_position());
        curr_state.move_next(lines, -1, -1);
    }
    return places.len();
}

pub fn process_grid_with_new_obstacle(
    start_state: &State,
    lines: &Vec<String>,
    x: i32,
    y: i32,
) -> bool {
    let mut states = HashSet::<State>::new();
    let mut curr_state = start_state.clone();
    let xsize = lines.len() as i32;
    let ysize = lines[0].len() as i32;

    while curr_state.is_valid(&lines) {
        if states.contains(&curr_state) {
            return true;
        }
        states.insert(curr_state.clone());
        curr_state.move_next(lines, x, y);
    }
    return false;
}

pub fn process_grid_2(lines: &Vec<String>) -> i32 {
    let mut start_state: State = find_start_place(&lines);
    let xsize = lines.len() as i32;
    let ysize = lines[0].len() as i32;
    let mut res: i32 = 0;

    for i in 0..xsize {
        for j in 0..ysize {
            if i == start_state.x && j == start_state.y {
                continue;
            }

            let is_loop = process_grid_with_new_obstacle(&start_state, &lines, i, j);
            res += is_loop as i32;
        }
    }

    return res;
}

pub fn solve_1() -> i32 {
    let lines = read_lines("inputs/day6.txt").unwrap();
    let res = process_grid_1(&lines);
    println!("res={}", res);
    return res as i32;
}

pub fn solve_2() -> i32 {
    let lines = read_lines("inputs/day6.txt").unwrap();
    let res = process_grid_2(&lines);
    println!("res={}", res);
    return res as i32;
}
