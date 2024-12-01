use std::collections::HashMap;

use crate::common::file_helper::read_lines;

pub fn solve() {
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
}
