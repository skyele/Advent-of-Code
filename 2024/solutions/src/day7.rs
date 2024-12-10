use crate::common::file_helper::read_lines;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Mode {
    Op2,
    Op3,
}

pub fn check(v: &Vec<i64>, idx: usize, target: i64, curr: i64, mode: &Mode) -> bool {
    if curr > target {
        return false;
    }

    if idx == v.len() {
        return curr == target;
    }

    let ele = v[idx];
    let mul = (idx == 0).then(|| 1 * ele).unwrap_or(curr * ele);
    let mut res =
        check(v, idx + 1, target, mul, mode) || check(v, idx + 1, target, curr + v[idx], mode);
    if *mode == Mode::Op3 {
        let num_digits = (ele as f64).log(10.0).floor() as i64 + 1;
        res = res
            || check(
                v,
                idx + 1,
                target,
                curr * 10i64.pow(num_digits.try_into().unwrap()) + ele,
                mode,
            );
    }
    return res;
}

pub fn process_line(line: &str, mode: &Mode) -> i64 {
    let target: i64 = line.split(":").collect::<Vec<&str>>()[0].parse().unwrap();
    let nums: Vec<i64> = line.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
    return check(&nums, 0, target, 0, mode)
        .then(|| target)
        .unwrap_or(0);
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day7.txt").unwrap();
    let mut res = 0;
    for line in lines {
        res += process_line(&line, &Mode::Op2);
    }
    println!("res={}", res);
    return res;
}

pub fn solve_2() -> i64 {
    let lines = read_lines("inputs/day7.txt").unwrap();
    let mut res = 0;
    for line in lines {
        res += process_line(&line, &Mode::Op3);
    }
    println!("res={}", res);
    return res;
}
