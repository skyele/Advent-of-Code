use crate::common::file_helper::read_lines;

pub fn search<'a>(
    grids: &Vec<String>,
    x: i32,
    y: i32,
    dx: &i32,
    dy: &i32,
    mut expect: impl Iterator<Item = &'a char>,
) -> bool {
    let next_ele = expect.next();
    if next_ele.is_none() {
        return true;
    }

    return check(grids, x, y, *next_ele.unwrap()) && search(grids, x + dx, y + dy, dx, dy, expect);
}

pub fn check(grids: &Vec<String>, x: i32, y: i32, expect: char) -> bool {
    let xsize = grids.len();
    let ysize = grids[0].len();
    if x < 0 || x >= xsize as i32 || y < 0 || y >= ysize as i32 {
        return false;
    }
    return grids[x as usize].chars().nth(y as usize).unwrap() == expect;
}

pub fn search_1<'a>(grids: &Vec<String>, x: i32, y: i32) -> bool {
    let xsize = grids.len();
    let ysize = grids[0].len();

    if !check(grids, x, y, 'A') {
        return false;
    }

    let diagonal = check(grids, x + 1, y + 1, 'M') && check(grids, x - 1, y - 1, 'S')
        || check(grids, x + 1, y + 1, 'S') && check(grids, x - 1, y - 1, 'M');
    let anti_diagonal = check(grids, x + 1, y - 1, 'M') && check(grids, x - 1, y + 1, 'S')
        || check(grids, x + 1, y - 1, 'S') && check(grids, x - 1, y + 1, 'M');

    return diagonal && anti_diagonal;
}

pub fn solve_1() {
    let lines = read_lines("inputs/day4.txt").unwrap();
    let xsize: usize = lines.len();
    let ysize: usize = lines[0].len();
    let dxys = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut cnt = 0;
    let expects = vec!['X', 'M', 'A', 'S'];
    for x in 0..xsize {
        for y in 0..ysize {
            for (dx, dy) in &dxys {
                cnt += search(&lines, x as i32, y as i32, dx, dy, expects.iter()) as i32;
            }
        }
    }
    println!("res={}", cnt);
}

pub fn solve_2() {
    let lines = read_lines("inputs/day4.txt").unwrap();
    let xsize: usize = lines.len();
    let ysize: usize = lines[0].len();

    let mut cnt = 0;
    for x in 0..xsize {
        for y in 0..ysize {
            cnt += search_1(&lines, x as i32, y as i32) as i32;
        }
    }
    println!("res={}", cnt);
}
