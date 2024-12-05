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

pub fn get_mid_ele(nums: &Vec<i32>) -> i32 {
    assert!(nums.len() % 2 == 1);
    return nums[nums.len() / 2];
}

pub fn find_blank_line_idx(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .position(|x| x.is_empty())
        .unwrap_or(lines.len())
}

pub fn parse_rule(line: &str) -> (i32, i32) {
    let parts: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
    return (parts[0], parts[1]);
}

pub fn parse_rules(
    lines: &Vec<String>,
    blank_line_idx: usize,
    is_reverse: bool,
) -> HashMap<i32, HashSet<i32>> {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in lines.iter().take(blank_line_idx) {
        let (k, v) = parse_rule(line);
        let (k, v) = if is_reverse { (v, k) } else { (k, v) };
        rules.entry(k).or_insert(HashSet::new()).insert(v);
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

pub fn correctify_updates(nums: &mut Vec<i32>, reverse_rules: &HashMap<i32, HashSet<i32>>) {
    let size = nums.len();
    for i in 0..size {
        let mut degree = 0;
        for j in 0..size {
            if i == j {
                continue;
            }
            degree += reverse_rules.get(&nums[j]).unwrap().contains(&nums[i]) as i32 * 2 - 1;
        }

        if degree == 0 {
            nums.swap(i, size / 2);
            return;
        }
    }
}

pub fn process_updates(
    line: &String,
    reverse_rules: &HashMap<i32, HashSet<i32>>,
    expect: bool,
) -> i32 {
    let mut nums: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    (check_updates(&nums, reverse_rules) == expect)
        .then(|| {
            (expect == false).then(|| correctify_updates(&mut nums, reverse_rules));
            get_mid_ele(&nums)
        })
        .unwrap_or(0)
}

pub fn solve_1() -> i32 {
    let lines = read_lines("inputs/day5.txt").unwrap();
    let blank_line_idx = find_blank_line_idx(&lines);
    let reverse_rules = parse_rules(&lines, blank_line_idx, true);

    let res = lines.iter().skip(blank_line_idx + 1).fold(0, |acc, line| {
        acc + process_updates(line, &reverse_rules, true)
    });
    println!("res={}", res);
    return res;
}

pub fn solve_2() -> i32 {
    let lines = read_lines("inputs/day5.txt").unwrap();
    let blank_line_idx = find_blank_line_idx(&lines);
    let reverse_rules = parse_rules(&lines, blank_line_idx, true);

    let res = lines.iter().skip(blank_line_idx + 1).fold(0, |acc, line| {
        acc + process_updates(line, &reverse_rules, false)
    });
    println!("res={}", res);
    return res;
}
