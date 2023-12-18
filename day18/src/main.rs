use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
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
        _ => unreachable!(),
    };

    Line { dir, len }
}

fn get_turns(lines: &[Line]) -> Vec<Turn> {
    let mut lines = lines.to_vec();
    lines.push(lines[0].clone());

    lines
        .windows(2)
        .map(|win| {
            let curr = &win[0];
            let next = &win[1];

            if curr.dir.turn_left() == next.dir {
                Turn::Left
            } else {
                Turn::Right
            }
        })
        .collect()
}

fn solution(lines: &[Line]) -> usize {
    let mut y: i64 = 0;
    let mut sub_area: i64 = 0;
    for line in lines {
        match line.dir {
            Dir::Down => y += line.len as i64,
            Dir::Up => y -= line.len as i64,
            Dir::Right | Dir::Left => {
                let sub_y = y * 2 + 1;
                let sub_width = (line.len - 1) * 2 + 2;

                if line.dir == Dir::Right {
                    sub_area += sub_y * sub_width as i64;
                } else {
                    sub_area -= sub_y * sub_width as i64;
                }
            }
        }
    }

    sub_area = sub_area.abs();
    let mut border_len = 0;

    let turns = get_turns(&lines);
    let num_right_turns = turns.iter().filter(|t| **t == Turn::Right).count();
    let clockwise = num_right_turns > turns.len() / 2;

    for (line, turn) in lines.iter().zip(&turns) {
        if clockwise == (*turn == Turn::Right) {
            sub_area -= 1;
        } else {
            sub_area -= 3;
        }
        sub_area -= (line.len as i64 - 1) * 2;
        border_len += line.len;
    }
    sub_area /= 4;

    usize::try_from(sub_area).unwrap() + border_len
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
