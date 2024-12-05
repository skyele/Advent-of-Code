use crate::common::file_helper::read_lines;

pub fn solve() -> i32 {
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

    let mut total_distance = 0;

    for (num1, num2) in v1.iter().zip(v2.iter()) {
        total_distance += (num1 - num2).abs();
    }

    println!("total distance: {}", total_distance);
    return total_distance;
}
