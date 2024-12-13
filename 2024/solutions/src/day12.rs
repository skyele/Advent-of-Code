use crate::common::file_helper::read_lines;
use std::collections::BTreeSet;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn is_valid(x: i32, y: i32, graph: &Vec<Vec<char>>) -> bool {
    return x >= 0 && x < graph.len() as i32 && y >= 0 && y < graph[0].len() as i32;
}

pub fn is_same(x: i32, y: i32, graph: &Vec<Vec<char>>, target: char) -> bool {
    return is_valid(x, y, graph) && graph[x as usize][y as usize] == target;
}

pub fn update_perimeter(
    x: usize,
    y: usize,
    perimeter: &mut i64,
    graph: &mut Vec<Vec<char>>,
    target: char,
) {
    *perimeter += 4 - DIRECTIONS.iter().fold(0, |acc, (dx, dy)| {
        is_same(x as i32 + dx, y as i32 + dy, graph, target) as i64 + acc
    });
}

pub fn update_sides(
    x: usize,
    y: usize,
    sides: &mut BTreeSet<(i32, i32, i32, i32, i32)>,
    graph: &mut Vec<Vec<char>>,
    target: char,
) {
    for (dx, dy) in DIRECTIONS.iter() {
        if !is_same(x as i32 + dx, y as i32 + dy, graph, target) {
            sides.insert((x as i32 + dx, y as i32 + dy, dx + dy, dy.abs(), dx.abs()));
        }
    }
}

pub fn dfs<T, F>(
    x: usize,
    y: usize,
    graph: &mut Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    area: &mut i64,
    state: &mut T,
    target: char,
    update_closure: &F,
) where
    F: Fn(usize, usize, &mut T, &mut Vec<Vec<char>>, char),
{
    if !is_valid(x as i32, y as i32, graph) || visited[x][y] || graph[x][y] != target {
        return;
    }

    visited[x][y] = true;

    *area += 1;
    update_closure(x, y, state, graph, target);

    for (dx, dy) in DIRECTIONS.iter() {
        dfs(
            (x as i32 + dx) as usize,
            (y as i32 + dy) as usize,
            graph,
            visited,
            area,
            state,
            target,
            update_closure,
        );
    }
}

pub fn parse_graph(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut graph: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        graph.push(row);
    }
    return graph;
}

pub fn cal_sides(sides: &mut BTreeSet<(i32, i32, i32, i32, i32)>) -> i64 {
    let mut res = 0;
    let mut next = (-1, -1, -1, -1, -1);
    while sides.len() > 0 {
        if !sides.contains(&next) {
            res += 1;
            next = *sides.iter().next().unwrap();
        }
        sides.remove(&next);
        let (x, y, flag, dx, dy) = next;
        next = (x + dx, y + dy, flag, dx, dy);
    }
    return res;
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day12.txt").unwrap();
    let graph = parse_graph(lines);
    let mut visited = vec![vec![false; graph[0].len()]; graph.len()];
    let mut res = 0;
    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            if !visited[i][j] {
                let mut area = 0;
                let mut perimeter = 0;
                dfs::<_, _>(
                    i,
                    j,
                    &mut graph.clone(),
                    &mut visited,
                    &mut area,
                    &mut perimeter,
                    graph[i][j],
                    &update_perimeter,
                );
                res += area * perimeter;
            }
        }
    }
    println!("res={}", res);
    return res;
}

pub fn solve_2() -> i64 {
    let lines = read_lines("inputs/day12.txt").unwrap();
    let graph = parse_graph(lines);
    let mut visited = vec![vec![false; graph[0].len()]; graph.len()];
    let mut res = 0;
    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            if !visited[i][j] {
                let mut area = 0;
                let mut sides = BTreeSet::<(i32, i32, i32, i32, i32)>::new();
                dfs::<_, _>(
                    i,
                    j,
                    &mut graph.clone(),
                    &mut visited,
                    &mut area,
                    &mut sides,
                    graph[i][j],
                    &update_sides,
                );

                res += area * cal_sides(&mut sides);
            }
        }
    }
    println!("res={}", res);
    return res;
}
