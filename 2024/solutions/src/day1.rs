use std::collections::HashMap;

use crate::common::file_helper::read_lines;

pub fn solve_1() -> i32 {
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

pub fn solve_2() -> i32 {
    let lines = read_lines("inputs/day1.txt").unwrap();
    let mut v1: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let nums: Vec<&str> = line.split_whitespace().collect();
        v1.push(nums[0].parse::<i32>().unwrap());
        let num2 = nums[1].parse::<i32>().unwrap();
        map.entry(num2).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut scores = 0;

    for ele in v1 {
        scores += ele * map.get(&ele).unwrap_or(&0);
    }

    println!("scores: {}", scores);
    return scores;
}
