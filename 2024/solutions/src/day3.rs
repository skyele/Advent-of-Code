use crate::common::file_helper::read_lines;

#[derive(PartialEq)]
enum ModeT {
    Solve1,
    Solve2,
}

#[derive(PartialEq)]
enum StateT {
    Init,
    Mul,
    Left,
    Num1,
    Comma,
    Num2,
    Right,
}

fn get_char_at_index(input: &str, index: usize) -> char {
    return input.chars().nth(index).unwrap();
}

struct ParserT<'a> {
    input: &'a str,
    size: usize,
    cursor: usize,
    is_end: bool,
    state: StateT,
    num1: i32,
    num2: i32,
    mode: ModeT,
    enable: bool,
}

impl<'a> ParserT<'a> {
    pub fn new(input: &'a str, mode: ModeT) -> Self {
        return ParserT {
            input: input,
            size: input.len(),
            cursor: 0,
            is_end: false,
            state: StateT::Init,
            num1: 0,
            num2: 0,
            mode: mode,
            enable: true,
        };
    }

    pub fn check_boundary(&mut self, to_consume: usize) -> bool {
        if self.cursor + to_consume > self.size {
            self.is_end = true;
            self.reset();
            return false;
        }
        return true;
    }

    pub fn consume_mul(&mut self) -> bool {
        if !self.check_boundary(3) {
            return false;
        }

        let substr = &self.input[self.cursor..self.cursor + 3];
        if substr == "mul" {
            self.state = StateT::Mul;
            self.cursor += 3;
            return true;
        } else {
            return false;
        }
    }

    pub fn consume_enable(&mut self, token: &str, expect_enable: bool) -> bool {
        let size = token.len();
        if !self.check_boundary(size) {
            return false;
        }

        let substr = &self.input[self.cursor..self.cursor + size];
        if substr == token {
            self.enable = expect_enable;
            self.cursor += size;
            return true;
        } else {
            return false;
        }
    }

    pub fn consume_char(&mut self, c: char, expect_state: StateT) {
        if get_char_at_index(self.input, self.cursor) == c {
            self.state = expect_state;
            self.cursor += 1;
        } else {
            self.reset();
        }
    }

    pub fn consume_num(&mut self, expect_state: StateT) -> i32 {
        if !get_char_at_index(self.input, self.cursor).is_digit(10) {
            self.reset();
            return 0;
        }

        let mut num_str = String::new();
        while self.cursor < self.size {
            if !get_char_at_index(self.input, self.cursor).is_digit(10) {
                break;
            }
            num_str.push(get_char_at_index(self.input, self.cursor));
            self.cursor += 1;
        }
        self.state = expect_state;
        return num_str.parse::<i32>().unwrap();
    }

    pub fn reset(&mut self) {
        self.state = StateT::Init;
        self.num1 = 0;
        self.num2 = 0;
    }

    pub fn consume(&mut self) {
        if self.cursor >= self.size {
            self.is_end = true;
            self.reset();
            return;
        }

        if self.state == StateT::Init {
            if self.mode == ModeT::Solve1 {
                if !self.consume_mul() {
                    self.cursor += 1;
                }
            } else {
                if self.consume_mul()
                    || self.consume_enable("do()", true)
                    || self.consume_enable("don't()", false)
                {
                } else {
                    self.cursor += 1;
                }
            }
            return;
        } else if self.state == StateT::Mul {
            self.consume_char('(', StateT::Left);
        } else if self.state == StateT::Left {
            self.num1 = self.consume_num(StateT::Num1);
        } else if self.state == StateT::Num1 {
            self.consume_char(',', StateT::Comma);
        } else if self.state == StateT::Comma {
            self.num2 = self.consume_num(StateT::Num2);
        } else if self.state == StateT::Num2 {
            self.consume_char(')', StateT::Right);
        }
    }

    pub fn set_input(&mut self, input: &'a str) {
        self.input = input;
        self.size = input.len();
        self.cursor = 0;
        self.is_end = false;
        self.reset();
    }
}

pub fn parse_line(parser: &mut ParserT) -> i32 {
    let mut res = 0;
    while !parser.is_end {
        parser.consume();
        if parser.state == StateT::Right {
            res += parser.num1 * parser.num2 * (parser.enable as i32);
            parser.reset();
        }
    }
    return res;
}

pub fn solve_1() {
    let lines = read_lines("inputs/day3.txt").unwrap();
    let mut res = 0;
    let mut parser = ParserT::new("", ModeT::Solve1);
    for line in &lines {
        parser.set_input(line);
        res += parse_line(&mut parser);
    }
    println!("res={}", res);
}

pub fn solve_2() {
    let lines = read_lines("inputs/day3.txt").unwrap();
    let mut res = 0;
    let mut parser = ParserT::new("", ModeT::Solve2);
    for line in &lines {
        parser.set_input(line);
        res += parse_line(&mut parser);
    }
    println!("res={}", res);
}
