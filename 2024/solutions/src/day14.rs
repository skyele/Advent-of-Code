use crate::common::file_helper::read_lines;

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Robot {
    fn move_robot(&mut self, times: i64, grid_x: i64, grid_y: i64) {
        self.x += self.dx * times;
        self.y += self.dy * times;

        self.x = (self.x % grid_x + grid_x) % grid_x;
        self.y = (self.y % grid_y + grid_y) % grid_y;
    }
}

pub fn get_quadrant(x: i64, y: i64, grid_x: i64, grid_y: i64) -> i64 {
    let (mid_x, mid_y) = (grid_x / 2, grid_y / 2);
    if x < mid_x && y < mid_y {
        return 1;
    } else if x > mid_x && y < mid_y {
        return 2;
    } else if x < mid_x && y > mid_y {
        return 3;
    } else if x > mid_x && y > mid_y {
        return 4;
    } else {
        return -1;
    }
}

pub fn parse_pair(line: &str) -> (i64, i64) {
    let nums = line
        .split("=")
        .nth(1)
        .unwrap()
        .chars()
        .map(|c| if c.is_digit(10) || c == '-' { c } else { ' ' })
        .collect::<String>()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    return (nums[0], nums[1]);
}

pub fn parse_robot(line: &str) -> Robot {
    let p = line.split(" ").nth(0).unwrap();
    let v = line.split(" ").nth(1).unwrap();
    let (y, x) = parse_pair(&p);
    let (dy, dx) = parse_pair(&v);

    return Robot {
        x: x,
        y: y,
        dx: dx,
        dy: dy,
    };
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day14.txt").unwrap();
    let (grid_x, grid_y) = (103, 101);
    let (mut quad_1, mut quad_2, mut quad_3, mut quad_4) = (0, 0, 0, 0);
    for line in lines {
        let mut robot = parse_robot(&line);
        robot.move_robot(100, grid_x, grid_y);
        match get_quadrant(robot.x, robot.y, grid_x, grid_y) {
            1 => quad_1 += 1,
            2 => quad_2 += 1,
            3 => quad_3 += 1,
            4 => quad_4 += 1,
            _ => (),
        }
    }

    let res = quad_1 * quad_2 * quad_3 * quad_4;
    println!("res={}", res);
    return res;
}
