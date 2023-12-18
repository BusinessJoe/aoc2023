use std::{
    collections::HashSet,
    io::{self, Read},
};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

trait Movable {
    fn mv(&self, dir: Dir) -> Self;
}

impl Movable for (i32, i32) {
    fn mv(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => (self.0 - 1, self.1),
            Dir::Down => (self.0 + 1, self.1),
            Dir::Left => (self.0, self.1 - 1),
            Dir::Right => (self.0, self.1 + 1),
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    dir: Dir,
    len: usize,
}

fn parse_line_1(line: &str) -> Line {
    let tokens: Vec<&str> = line.split(' ').collect();

    let dir = match tokens[0] {
        "U" => Dir::Up,
        "D" => Dir::Down,
        "L" => Dir::Left,
        "R" => Dir::Right,
        _ => unreachable!(),
    };

    let len: usize = tokens[1].parse().unwrap();

    let color = {
        let r = u8::from_str_radix(&tokens[2][2..4], 16).unwrap();
        let g = u8::from_str_radix(&tokens[2][4..6], 16).unwrap();
        let b = u8::from_str_radix(&tokens[2][6..8], 16).unwrap();
        (r, g, b)
    };

    Line { dir, len }
}

fn parse_line_2(line: &str) -> Line {
    let tokens: Vec<&str> = line.split(' ').collect();

    let len: usize = usize::from_str_radix(&tokens[2][2..7], 16).unwrap();
    let dir = match &tokens[2].chars().nth(7).unwrap() {
        '0' => Dir::Right,
        '1' => Dir::Down,
        '2' => Dir::Left,
        '3' => Dir::Up,
        _ => unreachable!()
    };

    Line { dir, len }
}

fn solution(lines: &[Line]) -> usize {
    let mut pos: (i32, i32) = (0, 0);
    let mut border: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        for _ in 0..line.len {
            pos = pos.mv(line.dir);
            border.insert(pos);
        }
    }

    let min_row = *border.iter().map(|(r, _)| r).min().unwrap();
    let min_col = *border.iter().map(|(_, c)| c).min().unwrap();
    let max_row = *border.iter().map(|(r, _)| r).max().unwrap();
    let max_col = *border.iter().map(|(_, c)| c).max().unwrap();

    let rows = usize::try_from(max_row - min_row + 1).unwrap();
    let cols = usize::try_from(max_col - min_col + 1).unwrap();

    // Add 1 for padding.
    border = border
        .into_iter()
        .map(|(r, c)| (r - min_row + 1, c - min_col + 1))
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut to_visit = vec![(0, 0)];

    while let Some(pos) = to_visit.pop() {
        if !(0 <= pos.0 && pos.0 < rows as i32 + 2 && 0 <= pos.1 && pos.1 < cols as i32 + 2) {
            continue;
        }

        if border.contains(&pos) {
            continue;
        }

        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        to_visit.extend([
            pos.mv(Dir::Up),
            pos.mv(Dir::Down),
            pos.mv(Dir::Left),
            pos.mv(Dir::Right),
        ])
    }

    (rows + 2) * (cols + 2) - visited.len()
}

pub fn solution_1(input: &str) -> usize {
    let lines: Vec<Line> = input.lines().map(parse_line_1).collect();
    solution(&lines)
}

pub fn solution_2(input: &str) -> usize {
    let lines: Vec<Line> = input.lines().map(parse_line_2).collect();
    solution(&lines)
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution_1(&input);
    println!("Part 1: {p1}");
    let p2 = solution_2(&input);
    println!("Part 2: {p2}");
}
