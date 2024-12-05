use crate::common::file_helper::read_lines;

pub fn search(grids: &Vec<String>, x: i32, y: i32, dx: &i32, dy: &i32, expect: char) -> bool {
    let xsize = grids.len();
    let ysize = grids[0].len();

    if x < 0 || x >= xsize as i32 || y < 0 || y >= ysize as i32 {
        return false;
    }

    if grids[x as usize].chars().nth(y as usize).unwrap() == expect {
        if expect == 'S' {
            return true;
        }

        let mut next_expect: char = ' ';
        if expect == 'X' {
            next_expect = 'M';
        } else if expect == 'M' {
            next_expect = 'A';
        } else if expect == 'A' {
            next_expect = 'S';
        }

        if search(grids, x + dx, y + dy, dx, dy, next_expect) {
            return true;
        }
    }
    return false;
}

pub fn solve_1() {
    let lines = read_lines("inputs/day4.txt").unwrap();
    let xsize: usize = lines.len();
    let ysize: usize = lines[0].len();
    let dxys = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    let mut cnt = 0;
    let expects = vec!['X', 'M', 'A', 'S'];
    for x in 0..xsize {
        for y in 0..ysize {
            for (dx, dy) in &dxys {
                    cnt += search(&lines, x as i32, y as i32, dx, dy, 'X') as i32;
            }
        }
    }
    println!("res={}", cnt);
}
