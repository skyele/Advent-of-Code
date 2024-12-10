use crate::common::file_helper::read_lines;

pub fn check(v: &Vec<i64>, idx: usize, target: i64, curr: i64) -> bool {
    if curr > target {
        return false;
    }

    if idx == v.len() {
        return curr == target;
    }

    let mul = (idx == 0).then(|| 1 * v[idx]).unwrap_or(curr * v[idx]);
    return check(v, idx + 1, target, mul) || check(v, idx + 1, target, curr + v[idx]);
}

pub fn process_line(line: &str) -> i64 {
    let target: i64 = line.split(":").collect::<Vec<&str>>()[0].parse().unwrap();
    let nums: Vec<i64> = line.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
    return check(&nums, 0, target, 0).then(|| target).unwrap_or(0);
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day7.txt").unwrap();
    let mut res = 0;
    for line in lines {
        res += process_line(&line);
    }
    println!("res={}", res);
    return res;
}
