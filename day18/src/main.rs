use std::io::{self, Read};

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

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

fn solution(lines: &[Line]) -> i64 {
    let mut y: i64 = 0;
    let mut sub_area: i64 = 0;
    let mut loop_length: i64 = 0;

    for line in lines {
        loop_length += line.len as i64;
        match line.dir {
            Dir::Down => y += line.len as i64,
            Dir::Up => y -= line.len as i64,
            Dir::Right => sub_area += y * line.len as i64,
            Dir::Left => sub_area -= y * line.len as i64,
        }
    }

    sub_area.abs() + loop_length / 2 + 1
}

pub fn solution_1(input: &str) -> i64 {
    let lines: Vec<Line> = input.lines().map(parse_line_1).collect();
    solution(&lines)
}

pub fn solution_2(input: &str) -> i64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(36807, solution_1(include_str!("../input.txt")))
    }

    #[test]
    fn part_2() {
        assert_eq!(48797603984357, solution_2(include_str!("../input.txt")))
    }
}
