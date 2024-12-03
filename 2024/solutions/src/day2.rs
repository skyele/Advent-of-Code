
use crate::common::file_helper::read_lines;

pub fn is_safe_metric1(vec: &Vec<i32>) -> bool {
  let equal_list = vec.iter().zip(vec.iter().skip(1)).filter(|(a, b)| a == b).count() as i32;
  if equal_list > 0 {
    return false;
  }

  let greater_list: Vec<_> = vec.iter().zip(vec.iter().skip(1)).map(|(a, b)| a > b).collect();
  return greater_list.iter().all(|&x| x == greater_list[0]);
}

pub fn is_safe_metric2(vec: &Vec<i32>) -> bool {
  let distance_list = vec.iter().zip(vec.iter().skip(1)).map(|(a, b)| (a - b).abs()).collect::<Vec<i32>>();
  return distance_list.iter().all(|&x| x <= 3);
}

pub fn is_safe(vec: &Vec<i32>) -> bool {
  return is_safe_metric1(vec) && is_safe_metric2(vec);
}

pub fn is_same_with_dampener(vec: &Vec<i32>) -> bool {
  if is_safe(vec) {
    return true;
  }

  for i in 0..vec.len() {
    let mut new_vec = vec.clone();
    new_vec.remove(i);
    if is_safe(&new_vec) {
      return true;
    }
  }
  return false;
}

pub fn parse_input(fname: &str) -> Vec<Vec<i32>> {
  let lines = read_lines(fname).unwrap();
  return Vec<Vec<i32>> = lines.into_iter().map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()).collect();
}

pub fn solve_1() {
  let vec_2d = parse_input("inputs/day2.txt");
  let safe_cnt = vec_2d.iter().filter(|x| is_safe(x)).count() as i32;
  println!("Safe count: {}", safe_cnt);
}

pub fn solve_2() {
  let vec_2d = parse_input("inputs/day2.txt");
  let safe_cnt = vec_2d.iter().filter(|x| is_same_with_dampener(x)).count() as i32;
  println!("Safe count with dampener: {}", safe_cnt);
}
