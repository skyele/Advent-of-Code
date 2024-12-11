use crate::common::file_helper::read_lines;
use crate::common::math_helper;

pub fn blink(nums: &mut Vec<i64>) {
    let mut to_insert: Vec<(usize, i64)> = Vec::new();
    for (idx, ele) in nums.into_iter().enumerate() {
        if ele == &0 {
            *ele = 1;
        } else {
            let num_digits = math_helper::get_num_digits(*ele);
            if num_digits % 2 == 0 {
                let mask = 10i64.pow((num_digits / 2) as u32);
                let right = *ele % mask;
                let left = *ele / mask;
                *ele = left;
                to_insert.push((idx + 1, right));
            } else {
                *ele = (*ele).checked_mul(2024).unwrap();
            }
        }
    }

    for (idx, ele) in to_insert.into_iter().rev() {
        nums.insert(idx, ele);
    }
}

pub fn parse_stones(line: &str) -> Vec<i64> {
    return line
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
}

pub fn print_stones(stones: &Vec<i64>) {
    for stone in stones {
        print!("{} ", stone);
    }
    println!();
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day11.txt").unwrap();
    let mut nums: Vec<i64> = parse_stones(&lines[0]);

    for i in 0..25 {
        blink(&mut nums);
    }

    let res = nums.len() as i64;
    println!("res={}", res);
    return res;
}
