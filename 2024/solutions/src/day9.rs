use crate::common::file_helper::read_lines;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Type {
    File,
    FreeSpace,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Block {
    id: i32,
    len: i32,
    block_type: Type,
}

impl Block {
    fn new(id: i32, len: i32, block_type: Type) -> Block {
        return Block {
            id,
            len,
            block_type,
        };
    }
}

pub fn translate_1(s: &str) -> Vec<i32> {
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

pub fn translate_2(s: &str) -> Vec<Block> {
    let mut res = Vec::<Block>::new();
    let mut id = 0;
    for (idx, c) in s.chars().enumerate() {
        let cnt = c.to_digit(10).unwrap() as i32;
        if idx % 2 == 0 {
            res.push(Block::new(id, cnt, Type::File));
            id += 1;
        } else {
            res.push(Block::new(-1, cnt, Type::FreeSpace));
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

pub fn merge_freespace(idx: usize, vec: &mut Vec<Block>) {
    if idx + 1 < vec.len() && vec[idx + 1].block_type == Type::FreeSpace {
        vec[idx].len += vec[idx + 1].len;
        vec.remove(idx + 1);
    }

    if idx > 0 && vec[idx - 1].block_type == Type::FreeSpace {
        vec[idx - 1].len += vec[idx].len;
        vec.remove(idx);
    }
}

pub fn amphipod_eager_move(vec: &mut Vec<Block>) {
    let r = vec
        .iter()
        .rposition(|x| x.block_type == Type::File)
        .unwrap();
    let mut to_process_id = vec[r].id;
    while to_process_id > 0 {
        let (mut l, r) = (0, vec.iter().rposition(|x| x.id == to_process_id).unwrap());
        let r_block = &vec[r];
        let r_b_len = r_block.len;

        while l < r {
            if vec[l].block_type == Type::FreeSpace && vec[l].len >= r_b_len {
                let left = vec[l].len - r_b_len;
                vec[l] = Block::new(r_block.id, r_b_len, Type::File);
                vec[r] = Block::new(-1, r_b_len, Type::FreeSpace);
                merge_freespace(r, vec);

                if left > 0 {
                    vec.insert(l + 1, Block::new(-1, left, Type::FreeSpace));
                    merge_freespace(l + 1, vec);
                }

                break;
            }
            l += 1;
        }
        to_process_id -= 1;
    }
}

pub fn checksum(vec: &Vec<i32>) -> i64 {
    return vec.iter().enumerate().fold(0, |acc, (idx, ele)| {
        let val = (*ele == -1).then(|| 0).unwrap_or(*ele);
        acc + (idx as i64) * (val as i64)
    });
}

pub fn checksum_block_mode(vec: &Vec<Block>) -> i64 {
    let mut idx: i64 = 0;
    let mut res: i64 = 0;
    for block in vec {
        if block.block_type == Type::File {
            for i in 0..block.len {
                res += (idx + i as i64) * block.id as i64;
            }
        }
        idx += block.len as i64;
    }
    return res;
}

pub fn solve_1() -> i64 {
    let lines = read_lines("inputs/day9.txt").unwrap();
    let mut vec = translate_1(&lines[0]);
    amphipod_move(&mut vec);
    let res = checksum(&vec);
    println!("res={}", res);
    return res;
}

pub fn solve_2() -> i64 {
    let lines = read_lines("inputs/day9.txt").unwrap();
    let mut vec = translate_2(&lines[0]);
    amphipod_eager_move(&mut vec);
    let res = checksum_block_mode(&vec);
    println!("res={}", res);
    return res;
}
