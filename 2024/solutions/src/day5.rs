use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::file_helper::read_lines;

pub fn print_rules(rules: &HashMap<i32, HashSet<i32>>) {
    for (num1, sets) in rules.iter() {
        print!("{} -> ", num1);
        for num2 in sets {
            print!("{}, ", num2);
        }
        println!();
    }
}

pub fn find_blank_line_idx(lines: &Vec<String>) -> usize {
    for (idx, line) in lines.iter().enumerate() {
        if line.is_empty() {
            return idx;
        }
    }
    return lines.len();
}

pub fn parse_rule(line: &str) -> (i32, i32) {
    let parts: Vec<&str> = line.split("|").collect();
    return (
        parts[0].parse::<i32>().unwrap(),
        parts[1].parse::<i32>().unwrap(),
    );
}

pub fn parse_rules(
    lines: &Vec<String>,
    idx: usize,
    is_reverse: bool,
) -> HashMap<i32, HashSet<i32>> {
    let blank_line_idx = find_blank_line_idx(lines);
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in lines.iter().take(blank_line_idx) {
        let (num1, num2) = parse_rule(line);
        if is_reverse {
            rules.entry(num2).or_insert(HashSet::new()).insert(num1);
        } else {
            rules.entry(num1).or_insert(HashSet::new()).insert(num2);
        }
    }
    return rules;
}

pub fn check_updates(nums: &Vec<i32>, reverse_rules: &HashMap<i32, HashSet<i32>>) -> bool {
    for i in 0..nums.len() {
        if let Some(set) = reverse_rules.get(&nums[i]) {
            for j in i + 1..nums.len() {
                if set.contains(&nums[j]) {
                    return false;
                }
            }
        }
    }

    return true;
}

pub fn process_updates(line: &String, reverse_rules: &HashMap<i32, HashSet<i32>>) -> i32 {
    let nums: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    assert!(nums.len() % 2 == 1);

    check_updates(&nums, reverse_rules)
        .then(|| nums[nums.len() / 2])
        .unwrap_or(0)
}

pub fn solve_1() -> i32 {
    let lines = read_lines("inputs/day5.txt").unwrap();
    let blank_line_idx = find_blank_line_idx(&lines);
    let reverse_rules = parse_rules(&lines, blank_line_idx, true);

    let mut res = 0;
    for line in lines.iter().skip(blank_line_idx + 1) {
        res += process_updates(line, &reverse_rules);
    }
    println!("res: {}", res);
    return res;
}
