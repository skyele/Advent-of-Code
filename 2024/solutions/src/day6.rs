use std::collections::HashSet;
use std::fmt;

use crate::common::file_helper::read_lines;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

    fn move_next(&mut self, lines: &Vec<String>) {
        self.move_forward();
        if !self.is_guarded(lines) {
            return;
        }

        self.move_backward();
        self.turn_right();
        self.move_forward();
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

pub fn process_grid(lines: &Vec<String>) -> usize {
    let mut start_state: State = find_start_place(&lines);
    let mut places = HashSet::<(i32, i32)>::new();
    let curr_state = &mut start_state;
    while curr_state.is_valid(&lines) {
        places.insert(curr_state.get_position());
        curr_state.move_next(lines);
    }
    return places.len();
}

pub fn solve_1() -> i32 {
    let lines = read_lines("inputs/day6.txt").unwrap();
    let res = process_grid(&lines);
    println!("res={}", res);
    return res as i32;
}
