use crate::common::file_helper::read_lines;
use crate::common::math_helper;
use std::collections::HashMap;

pub fn process_new_num(
    num: i64,
    cnt: i64,
    new_num_cnt: &mut HashMap<i64, i64>,
    num_cache: &mut HashMap<i64, Vec<i64>>,
) {
    let (mut num1, mut num2) = (Option::None, Option::None);
    if num == 0 {
        num1 = Some(1);
    } else {
        let num_digits = math_helper::get_num_digits(num);
        if num_digits % 2 == 0 {
            let mask = 10i64.pow((num_digits / 2) as u32);
            (num1, num2) = (Some(num / mask), Some(num % mask));
        } else {
            num1 = Some(num.checked_mul(2024).unwrap());
        }
    }

    for &ele in [&num1, &num2] {
        ele.map(|ele| {
            new_num_cnt
                .entry(ele)
                .and_modify(|v| *v = v.checked_add(cnt).unwrap())
                .or_insert(cnt);
            num_cache.entry(num).or_insert(Vec::new()).push(ele);
        });
    }
}

pub fn process_exist_num(
    num: i64,
    cnt: i64,
    new_num_cnt: &mut HashMap<i64, i64>,
    num_cache: &mut HashMap<i64, Vec<i64>>,
) {
    num_cache.get(&num).unwrap().iter().for_each(|&ele| {
        new_num_cnt
            .entry(ele)
            .and_modify(|v| *v = v.checked_add(cnt).unwrap())
            .or_insert(cnt);
    });
}

pub fn blink(num_cnt: &mut HashMap<i64, i64>, num_cache: &mut HashMap<i64, Vec<i64>>) {
    let mut new_num_cnt: HashMap<i64, i64> = HashMap::new();

    for (&num, &cnt) in num_cnt.iter() {
        if !num_cache.contains_key(&num) {
            process_new_num(num, cnt, &mut new_num_cnt, num_cache);
        } else {
            process_exist_num(num, cnt, &mut new_num_cnt, num_cache);
        }
    }

    *num_cnt = new_num_cnt;
}

pub fn parse_stones(line: &str) -> HashMap<i64, i64> {
    return line
        .split_whitespace()
        .map(|x| x.parse::<i64>().expect("Failed to parse number"))
        .fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
}

pub fn score_num_cnt(num_cnt: &HashMap<i64, i64>) -> i64 {
    return num_cnt.into_iter().fold(0, |acc, (_, cnt)| acc + cnt);
}

pub fn blinks(num_cnt: &mut HashMap<i64, i64>, num_cache: &mut HashMap<i64, Vec<i64>>, n: i32) {
    for _ in 0..n {
        blink(num_cnt, num_cache);
    }
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day11.txt").unwrap();
    let mut num_cnt: HashMap<i64, i64> = parse_stones(&lines[0]);
    let mut num_cache: HashMap<i64, Vec<i64>> = HashMap::new();

    blinks(&mut num_cnt, &mut num_cache, 25);

    let res = score_num_cnt(&num_cnt);
    println!("res={}", res);
    return res;
}

pub fn solve_2() -> i64 {
    let lines = read_lines("inputs/day11.txt").unwrap();
    let mut num_cnt: HashMap<i64, i64> = parse_stones(&lines[0]);
    let mut num_cache: HashMap<i64, Vec<i64>> = HashMap::new();

    blinks(&mut num_cnt, &mut num_cache, 75);

    let res = score_num_cnt(&num_cnt);
    println!("res={}", res);
    return res;
}
