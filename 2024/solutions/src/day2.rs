
use crate::common::file_helper::read_lines;

pub fn is_safe_metric1(vec: &Vec<i32>) -> bool {
  if vec.len() == 1 {
    return true;
  }

  let mut is_descending = false;
  let mut is_ascending = false;

  if vec[0] > vec[1] {
    is_descending = true;
  } else if vec[0] < vec[1] {
    is_ascending = true;
  } else {
    return false;
  }

  for i in 1..vec.len() {
    if is_descending && vec[i] < vec[i - 1] {
      continue;
    } else if is_ascending && vec[i] > vec[i - 1] {
      continue;
    } else {
      return false;
    }
  }
  return true;
}

pub fn is_safe_metric2(vec: &Vec<i32>) -> bool {
  if vec.len() == 1 {
    return true;
  }

  for i in 1..vec.len() {
    if (vec[i] - vec[i - 1]).abs() > 3 {
      return false;
    }
  }
  return true;
}

pub fn is_safe(vec: &Vec<i32>) -> bool {
  return is_safe_metric1(vec) && is_safe_metric2(vec);
}

pub fn is_same_with_dampener(vec: &Vec<i32>) -> bool {
  if is_safe_metric1(vec) && is_safe_metric2(vec) {
    return true;
  }

  for i in 0..vec.len() {
    let mut new_vec = vec.clone();
    new_vec.remove(i);
    if is_safe_metric1(&new_vec) && is_safe_metric2(&new_vec) {
      return true;
    }
  }
  return false;
}

pub fn parse_input(fname: &str) -> Vec<Vec<i32>> {
  let lines = read_lines(fname).unwrap();
  let mut vec_2d: Vec<Vec<i32>> = Vec::new();
  for line in lines{
    let local_vec: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    vec_2d.push(local_vec);
  }
  return vec_2d;
}

pub fn solve_1() {
  let vec_2d = parse_input("inputs/day2.txt");

  let mut safe_cnt = 0;
  for vec in vec_2d {
    safe_cnt += is_safe(&vec) as i32;
  }
  println!("Safe count: {}", safe_cnt);
}

pub fn solve_2() {
  let vec_2d = parse_input("inputs/day2.txt");

  let mut safe_cnt = 0;
  for vec in vec_2d {
    safe_cnt += is_same_with_dampener(&vec) as i32;
  }
  println!("Safe count with dampener: {}", safe_cnt);
}
