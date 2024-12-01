use crate::common::file_helper::read_lines;

pub fn solve() {
    let lines = read_lines("inputs/day1.txt").unwrap();
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in lines {
        let nums: Vec<&str> = line.split_whitespace().collect();
        v1.push(nums[0].parse::<i32>().unwrap());
        v2.push(nums[1].parse::<i32>().unwrap());
    }

    v1.sort();
    v2.sort();

    let size = v1.len();
    let mut total_distance = 0;

    for i in 0..size {
        let num1 = v1[i];
        let num2 = v2[i];

        total_distance += (num1 - num2).abs();
    }

    println!("total distance: {}", total_distance);
}
