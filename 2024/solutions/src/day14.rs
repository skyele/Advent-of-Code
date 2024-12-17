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

pub fn print_grid(robots: &Vec<Robot>, grid_x: i64, grid_y: i64) {
    let mut grid = vec![vec!['.'; grid_x as usize]; grid_y as usize];

    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = '#';
    }

    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn get_cnt(row: &Vec<i32>) -> i32 {
    return row.iter().filter(|&x| *x == 1).count() as i32;
}

pub fn check_tree(robots: &Vec<Robot>, grid_x: i64, grid_y: i64) -> bool {
    let mut grid = vec![vec![0; grid_y as usize]; grid_x as usize];
    robots.iter().for_each(|robot| {
        grid[robot.x as usize][robot.y as usize] = 1;
    });

    for row in grid {
        if get_cnt(&row) < 30 {
            continue;
        }

        let mut consecutive = 0;
        for i in 1..row.len() {
            consecutive += row[i - 1] * row[i];
            if consecutive > 25 {
                return true;
            }
        }
    }

    return false;
}

pub fn cal_safety_factor(robots: &mut Vec<Robot>, move_time: i64, grid_x: i64, grid_y: i64) -> i64 {
    let (mut quad_1, mut quad_2, mut quad_3, mut quad_4) = (0, 0, 0, 0);

    for robot in robots {
        robot.move_robot(move_time, grid_x, grid_y);
        match get_quadrant(robot.x, robot.y, grid_x, grid_y) {
            1 => quad_1 += 1,
            2 => quad_2 += 1,
            3 => quad_3 += 1,
            4 => quad_4 += 1,
            _ => (),
        }
    }

    return quad_1 * quad_2 * quad_3 * quad_4;
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day14.txt").unwrap();
    let mut robots = lines
        .iter()
        .map(|line| parse_robot(line))
        .collect::<Vec<Robot>>();

    let (grid_x, grid_y) = (103, 101);
    let res = cal_safety_factor(&mut robots, 100, grid_x, grid_y);
    println!("res={}", res);
    return res;
}

pub fn solve_2() -> i64 {
    let lines = read_lines("inputs/day14.txt").unwrap();
    let mut robots = lines
        .iter()
        .map(|line| parse_robot(line))
        .collect::<Vec<Robot>>();

    let threshold: i64 = 125 as i64 * 125 * 125 * 125 / 2;
    let (grid_x, grid_y) = (103, 101);
    let mut res = 0;

    for iter in 0..grid_x * grid_y {
        if cal_safety_factor(&mut robots, 1, grid_x, grid_y) > threshold {
            continue;
        }

        if check_tree(&robots, grid_x, grid_y) {
            res = iter + 1;
            break;
        }
    }

    println!("res={}", res);
    return res;
}
