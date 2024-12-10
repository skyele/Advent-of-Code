use std::collections::HashSet;

use crate::common::file_helper::read_lines;

const VISITED: i32 = 9999;
const THRESHOLD: i32 = 9900;
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn print_grid(grid: &Vec<Vec<i32>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{} ", grid[i][j]);
        }
        println!();
    }
}

pub fn parse_grid(lines: &Vec<String>) -> Vec<Vec<i32>> {
    return lines
        .iter()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect())
        .collect();
}

pub fn parse_result_grid(lines: &Vec<String>) -> Vec<Vec<HashSet<(usize, usize)>>> {
    return lines
        .iter()
        .map(|x| x.chars().map(|y| HashSet::new()).collect())
        .collect();
}

pub fn is_valid_pos(x: usize, y: usize, grid: &Vec<Vec<i32>>) -> bool {
    return x >= 0 && x < grid.len() && y >= 0 && y < grid[0].len();
}

pub fn restore_grid(grid: &mut Vec<Vec<i32>>) {
    grid.iter_mut().for_each(|x| {
        x.iter_mut().for_each(|y| {
            (*y > THRESHOLD).then(|| {
                *y = VISITED - *y;
            });
        })
    });
}

pub fn search(
    x: usize,
    y: usize,
    expect: i32,
    start_pos: (usize, usize),
    grid: &mut Vec<Vec<i32>>,
    result_grid: &mut Vec<Vec<HashSet<(usize, usize)>>>,
) {
    if !is_valid_pos(x, y, grid) || grid[x][y] > THRESHOLD || grid[x][y] != expect {
        return;
    }

    if expect == 9 && grid[x][y] == expect {
        result_grid[x][y].insert(start_pos);
        return;
    }

    grid[x][y] = VISITED - grid[x][y];

    for (dx, dy) in DIRECTIONS.iter() {
        let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);
        search(
            new_x as usize,
            new_y as usize,
            expect + 1,
            start_pos,
            grid,
            result_grid,
        );
    }
}

pub fn score_grid(result_grid: &mut Vec<Vec<HashSet<(usize, usize)>>>) -> i64 {
    return result_grid
        .iter()
        .map(|x| x.iter().map(|y| y.len() as i64).sum::<i64>())
        .sum::<i64>();
}

pub fn score_trailheads(
    grid: &mut Vec<Vec<i32>>,
    result_grid: &mut Vec<Vec<HashSet<(usize, usize)>>>,
) -> i64 {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                search(i, j, 0, (i, j), grid, result_grid);
                restore_grid(grid);
            }
        }
    }

    return score_grid(result_grid);
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day10.txt").unwrap();
    let mut grid = parse_grid(&lines);
    let mut result_grid = parse_result_grid(&lines);
    let res = score_trailheads(&mut grid, &mut result_grid) as i64;
    println!("res={}", res);
    return res;
}
