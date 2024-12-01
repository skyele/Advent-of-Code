use std::collections::HashMap;

use crate::common::file_helper::read_lines;

pub fn get_cnt(map: &HashMap<i32, i32>, num: i32) -> i32 {
    if map.contains_key(&num) {
        return *map.get(&num).unwrap();
    }
    return 0;
}

pub fn solve() {
    let lines = read_lines("inputs/day1.txt").unwrap();
    let mut v1: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let nums: Vec<&str> = line.split_whitespace().collect();
        v1.push(nums[0].parse::<i32>().unwrap());
        let num2 = nums[1].parse::<i32>().unwrap();
        if map.contains_key(&num2) {
            let count = map.get(&num2).unwrap();
            map.insert(num2, count + 1);
        } else {
            map.insert(num2, 1);
        }
    }

    let mut scores = 0;

    for ele in v1 {
        let cnt = get_cnt(&map, ele);
        scores += ele * cnt;
    }

    println!("scores: {}", scores);
}
