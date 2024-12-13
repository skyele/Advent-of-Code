use crate::common::file_helper::read_lines;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn is_valid(x: i32, y: i32, graph: &Vec<Vec<char>>) -> bool {
    return x >= 0 && x < graph.len() as i32 && y >= 0 && y < graph[0].len() as i32;
}

pub fn is_same(x: i32, y: i32, graph: &Vec<Vec<char>>, target: char) -> bool {
    return is_valid(x, y, graph) && graph[x as usize][y as usize] == target;
}

pub fn cal_perimeter(x: usize, y: usize, graph: &mut Vec<Vec<char>>, target: char) -> i64 {
    return 4 - DIRECTIONS.iter().fold(0, |acc, (dx, dy)| {
        is_same(x as i32 + dx, y as i32 + dy, graph, target) as i64 + acc
    });
}

pub fn dfs(
    x: usize,
    y: usize,
    graph: &mut Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    area: &mut i64,
    perimeter: &mut i64,
    target: char,
) {
    if !is_valid(x as i32, y as i32, graph) || visited[x][y] || graph[x][y] != target {
        return;
    }

    visited[x][y] = true;

    *area += 1;
    *perimeter += cal_perimeter(x, y, graph, target);

    for (dx, dy) in DIRECTIONS.iter() {
        dfs(
            (x as i32 + dx) as usize,
            (y as i32 + dy) as usize,
            graph,
            visited,
            area,
            perimeter,
            target,
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
                dfs(
                    i,
                    j,
                    &mut graph.clone(),
                    &mut visited,
                    &mut area,
                    &mut perimeter,
                    graph[i][j],
                );
                res += area * perimeter;
            }
        }
    }
    println!("res={}", res);
    return res;
}
