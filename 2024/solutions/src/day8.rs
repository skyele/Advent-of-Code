use crate::common::file_helper::read_lines;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Mode {
    Mode1,
    Mode2,
}

pub fn parse_map(lines: &Vec<String>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                let mut vec = map.entry(c).or_insert(Vec::<(i32, i32)>::new());
                vec.push((i as i32, j as i32));
            }
        }
    }
    return map;
}

pub fn set_grid(grid: &mut Vec<Vec<i32>>, ele: (i32, i32)) -> bool {
    let xsize = grid.len() as i32;
    let ysize = grid[0].len() as i32;

    if ele.0 < 0 || ele.0 >= xsize || ele.1 < 0 || ele.1 >= ysize {
        return false;
    }

    grid[ele.0 as usize][ele.1 as usize] = 1;
    return true;
}

pub fn count_grid(grid: &Vec<Vec<i32>>) -> i32 {
    return grid.iter().map(|x| x.iter().sum::<i32>()).sum::<i32>();
}

pub fn set_antenna(
    xsize: usize,
    ysize: usize,
    map: &HashMap<char, Vec<(i32, i32)>>,
    grid: &mut Vec<Vec<i32>>,
    mode: Mode,
) {
    for (k, vec) in map.iter() {
        let vsize = vec.len();
        for i in 0..vsize {
            for j in i + 1..vsize {
                let (x1, y1) = vec[i];
                let (x2, y2) = vec[j];
                let (x_diff, y_diff) = (x1 - x2, y1 - y2);

                if mode == Mode::Mode1 {
                    set_grid(grid, (x1 + x_diff, y1 + y_diff));
                    set_grid(grid, (x2 - x_diff, y2 - y_diff));
                }
            }
        }
    }
}

pub fn solve_1() -> i32 {
    let lines = read_lines("inputs/day8.txt").unwrap();
    let map = parse_map(&lines);
    let xsize = lines.len() as i32;
    let ysize = lines[0].len() as i32;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; ysize as usize]; xsize as usize];
    set_antenna(xsize as usize, ysize as usize, &map, &mut grid);
    let res = count_grid(&mut grid);
    println!("res={}", res);
    return res;
}
