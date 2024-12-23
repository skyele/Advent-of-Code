use crate::common::file_helper::read_lines;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Special,
}

struct Graph {
    nodes: Vec<(i64, i64)>,
    start: (i64, i64),
    end: (i64, i64),
    visited: HashSet<(i64, i64)>,
    non_visited: HashSet<(i64, i64)>,
    scores_ranking: BinaryHeap<Reverse<(i64, (i64, i64))>>,
    scores: HashMap<(i64, i64), i64>,
    directions: HashMap<(i64, i64), Direction>,
}

impl Graph {
    fn update_score(&mut self, node: (i64, i64), score: i64) {
        self.scores.insert(node, score);
        self.scores_ranking.push(Reverse((score, node)));
    }

    fn mark_visit(&mut self, node: (i64, i64)) {
        self.visited.insert(node);
        self.non_visited.remove(&node);
    }
}

pub fn parse_grid(lines: &Vec<String>, blank_line_idx: usize) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in lines.iter().take(blank_line_idx) {
        let row = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }
    return grid;
}

pub fn parse_graph(grid: &Vec<Vec<char>>) -> Graph {
    let mut graph = Graph {
        nodes: Vec::new(),
        start: (0, 0),
        end: (0, 0),
        visited: HashSet::new(),
        non_visited: HashSet::new(),
        scores_ranking: BinaryHeap::new(),
        scores: HashMap::new(),
        directions: HashMap::new(),
    };

    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let (i_i64, j_i64) = (i as i64, j as i64);
            if c != &'#' {
                graph.nodes.push((i_i64, j_i64));
                graph.non_visited.insert((i_i64, j_i64));
            }
            match c {
                'S' => graph.start = (i_i64, j_i64),
                'E' => graph.end = (i_i64, j_i64),
                _ => (),
            }
        }
    }

    return graph;
}

pub fn find_min_score(graph: &mut Graph) -> (i64, (i64, i64)) {
    let Reverse((min_score, node)) = graph.scores_ranking.pop().unwrap();
    return (min_score, node);
}

pub fn is_valid(grid: &Vec<Vec<char>>, node: (i64, i64)) -> bool {
    let (i, j) = node;
    if i < 0
        || i >= grid.len() as i64
        || j < 0
        || j >= grid[0].len() as i64
        || grid[i as usize][j as usize] == '#'
    {
        return false;
    }
    return true;
}

pub fn get_direction(from: (i64, i64), to: (i64, i64)) -> Direction {
    let (from_x, from_y) = from;
    let (to_x, to_y) = to;
    match (to_x - from_x, to_y - from_y) {
        (0, 1) => return Direction::Right,
        (0, -1) => return Direction::Left,
        (1, 0) => return Direction::Down,
        (-1, 0) => return Direction::Up,
        _ => return Direction::Special,
    }
}

pub fn update_score(graph: &mut Graph, node: (i64, i64), score: i64) {
    graph.scores.insert(node, score);
    graph.scores_ranking.push(Reverse((score, node)));
}

pub fn cal_score(
    graph: &Graph,
    score: i64,
    curr: (i64, i64),
    next: (i64, i64),
    exist_direction: &Direction,
) -> i64 {
    let next_direction = get_direction(curr, next);
    let new_score = score
        + 1
        + if exist_direction != &next_direction || exist_direction == &Direction::Special {
            1000
        } else {
            0
        };
    return new_score;
}

pub fn dijkstra(grid: &Vec<Vec<char>>, graph: &mut Graph) {
    let mut curr = graph.start;
    graph.mark_visit(curr);
    graph.update_score(curr, 0);
    graph.directions.insert(curr, Direction::Special);

    while !graph.non_visited.is_empty() {
        let (score, curr) = find_min_score(graph);
        graph.mark_visit(curr);

        for d in DIRECTIONS {
            let next = (curr.0 + d.0, curr.1 + d.1);
            if !is_valid(grid, next) || !graph.non_visited.contains(&next) {
                continue;
            }

            let exist_score = *graph.scores.get(&next).unwrap_or(&std::i64::MAX);
            let new_score = cal_score(
                graph,
                score,
                curr,
                next,
                graph.directions.get(&curr).unwrap(),
            );
            let next_direction = get_direction(curr, next);

            if new_score < exist_score {
                graph.update_score(next, new_score);
                graph.directions.insert(next, next_direction);
            }
        }
    }
}

pub fn dfs(
    x: i64,
    y: i64,
    grid: &Vec<Vec<char>>,
    graph: &mut Graph,
    direction: Direction,
    score: i64,
    target: i64,
    cache: &mut HashMap<(i64, i64, i64), bool>,
    node_set: &mut HashSet<(i64, i64)>,
) -> bool {
    let curr = (x, y);
    if curr == graph.end && score == target {
        return true;
    }

    let cache_key = (x, y, score);
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    if !is_valid(grid, curr) || graph.visited.contains(&curr) || score > target || curr == graph.end
    {
        return false;
    }

    graph.visited.insert(curr);

    let mut res = false;
    for d in DIRECTIONS {
        let next = (curr.0 + d.0, curr.1 + d.1);
        let new_score = cal_score(graph, score, curr, next, &direction);
        let next_direction = get_direction(curr, next);
        res = dfs(
            next.0,
            next.1,
            grid,
            graph,
            next_direction,
            new_score,
            target,
            cache,
            node_set,
        ) || res; // order matters, || short cut
    }

    if res {
        node_set.insert(curr);
    }

    cache.insert(cache_key, res);
    graph.visited.remove(&curr);
    return res;
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day16.txt").unwrap();
    let mut grid = parse_grid(&lines, lines.len());
    let mut graph = parse_graph(&grid);

    dijkstra(&grid, &mut graph);

    let res = *graph.scores.get(&graph.end).unwrap();
    println!("result: {}", res);
    return res;
}

pub fn solve_2() -> i64 {
    let lines = read_lines("inputs/day16.txt").unwrap();
    let mut grid = parse_grid(&lines, lines.len());
    let mut graph = parse_graph(&grid);
    let mut cache = HashMap::new();
    let mut node_set = HashSet::new();
    node_set.insert(graph.end);

    dijkstra(&grid, &mut graph);
    let target = *graph.scores.get(&graph.end).unwrap();
    graph.visited.clear();
    dfs(
        graph.start.0,
        graph.start.1,
        &grid,
        &mut graph,
        Direction::Special,
        0,
        target,
        &mut cache,
        &mut node_set,
    );

    let res = node_set.len() as i64;
    println!("result: {}", res);
    return res;
}
