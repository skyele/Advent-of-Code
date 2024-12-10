use crate::common::file_helper::read_lines;

pub fn translate(s: &str) -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    let mut id = 0;
    for (idx, c) in s.chars().enumerate() {
        let cnt = c.to_digit(10).unwrap();
        if idx % 2 == 0 {
            for _ in 0..cnt {
                res.push(id);
            }
            id += 1;
        } else {
            for _ in 0..cnt {
                res.push(-1);
            }
        }
    }
    return res;
}

pub fn amphipod_move(vec: &mut Vec<i32>) {
    let (mut l, mut r) = (0, vec.len() - 1);
    while l < r {
        if vec[r] == -1 {
            r -= 1;
        } else if vec[l] != -1 {
            l += 1;
        } else {
            vec.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}

pub fn checksum(vec: &Vec<i32>) -> i64 {
    return vec.iter().enumerate().fold(0, |acc, (idx, ele)| {
        let val = (*ele == -1).then(|| 0).unwrap_or(*ele);
        acc + (idx as i64) * (val as i64)
    });
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day9.txt").unwrap();
    let mut vec = translate(&lines[0]);
    amphipod_move(&mut vec);
    let res = checksum(&vec);
    println!("res={}", res);
    return res;
}
