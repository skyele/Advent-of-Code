use crate::common::file_helper::read_lines;

#[derive(Debug, PartialEq)]
enum Mode {
    Mode1,
    Mode2,
}

#[derive(Debug)]
struct EquationParam {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    target_x: i64,
    target_y: i64,
}

pub fn parse_button(line: &str) -> (i64, i64) {
    let substr = line
        .split(":")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|&c| c.is_digit(10) || c == ' ')
        .collect::<String>()
        .trim()
        .to_string();
    let nums = substr
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    return (nums[0], nums[1]);
}

pub fn parse_equations(lines: Vec<String>, mode: Mode) -> Vec<EquationParam> {
    let mut equations = Vec::new();
    let (mut a, mut b, mut c, mut d, mut target_x, mut target_y) = (0, 0, 0, 0, 0, 0);
    for line in lines {
        if line == "" {
            equations.push(EquationParam {
                a: a,
                b: b,
                c: c,
                d: d,
                target_x: target_x,
                target_y: target_y,
            });
            (a, b, c, d, target_x, target_y) = (0, 0, 0, 0, 0, 0);
            continue;
        }
        if line.contains("Button A") {
            (a, b) = parse_button(&line);
        } else if line.contains("Button B") {
            (c, d) = parse_button(&line);
        } else if line.contains("Prize") {
            (target_x, target_y) = parse_button(&line);
            if mode == Mode::Mode2 {
                target_x += 10000000000000;
                target_y += 10000000000000;
            }
        }
    }

    equations.push(EquationParam {
        a: a,
        b: b,
        c: c,
        d: d,
        target_x: target_x,
        target_y: target_y,
    });
    return equations;
}

pub fn solve_equation(equation: &EquationParam) -> Option<i64> {
    let (a, b, c, d) = (equation.a, equation.b, equation.c, equation.d);
    let (target_x, target_y) = (equation.target_x, equation.target_y);
    let x = (target_x * d - target_y * c) / (a * d - b * c);
    let y = (target_x - a * x) / c;

    (a * x + c * y == target_x && b * x + d * y == target_y).then(|| 3 * x + y)
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day13.txt").unwrap();
    let equations = parse_equations(lines, Mode::Mode1);
    let res = equations
        .iter()
        .map(|equation| solve_equation(&equation).unwrap_or(0) as i64)
        .sum();

    println!("res={}", res);
    return res;
}

pub fn solve_2() -> i64 {
    let lines = read_lines("inputs/day13.txt").unwrap();
    let equations = parse_equations(lines, Mode::Mode2);
    let res = equations
        .iter()
        .map(|equation| solve_equation(&equation).unwrap_or(0) as i64)
        .sum();

    println!("res={}", res);
    return res;
}
