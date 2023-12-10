use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::{cmp, io};

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

enum Direction {
    Left,
    Right,
}

fn parse_line(line: &str) -> (&str, &str, &str) {
    (
        &line[0..3],
        &line[7..10],
        &line[12..15],
    )
}

fn parse_dirs(line: &str) -> Vec<Direction> {
    line.as_bytes()
        .iter()
        .map(|&c| {
            if c == b'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect()
}

fn parse_map<'a>(lines: impl IntoIterator<Item = &'a str>) -> Map<'a> {
    lines
        .into_iter()
        .map(parse_line)
        .map(|(name, left, right)| (name, (left, right)))
        .collect()
}

fn solution1<'a>(lines: impl IntoIterator<Item = &'a str>) -> usize {
    let mut lines = lines.into_iter();
    let dirs = parse_dirs(lines.next().unwrap());
    let map = parse_map(lines.skip(1));

    let mut name = "AAA";
    let mut i = 0;
    while name != "ZZZ" {
        let dir = &dirs[i % dirs.len()];
        let (left, right) = map.get(name).unwrap();
        name = match dir {
            Direction::Left => left,
            Direction::Right => right,
        };
        i += 1;
    }

    i
}

#[derive(Debug)]
struct Cycle {
    /// Initial pattern of Zs before the cycle starts looping
    head_zees: Vec<bool>,
    /// Length of this cycle's loop
    period: usize,
    /// Positions of Zs inside the cycle's loop
    periodic_zees: HashSet<usize>,
}

impl Cycle {
    fn zees(&self, i: usize) -> bool {
        if i < self.head_zees.len() {
            self.head_zees[i]
        } else {
            let idx = (i - self.head_zees.len()) % self.period;
            self.periodic_zees.contains(&idx)
        }
    }
}

fn build_cycle(start_name: &str, map: &Map, dirs: &[Direction]) -> Cycle {
    let mut name = start_name;
    let mut zees: Vec<bool> = Vec::new();

    let mut idx = 0;
    let mut seen: HashMap<&str, usize> = HashMap::new();

    while seen.get(name).is_none() {
        seen.insert(name, idx);
        for dir in dirs {
            zees.push(name.ends_with('Z'));
            let (left, right) = map.get(name).unwrap();
            name = match dir {
                Direction::Left => left,
                Direction::Right => right,
            };
            idx += 1;
        }
    }

    let split_idx = seen.get(name).unwrap();
    let period = idx - split_idx;
    let periodic_zees_vec = zees.split_off(*split_idx);
    debug_assert_eq!(period, periodic_zees_vec.len());

    let periodic_zees: HashSet<usize> = periodic_zees_vec
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .collect();

    debug_assert_ne!(0, periodic_zees.len());
    Cycle {
        period,
        periodic_zees,
        head_zees: zees,
    }
}

fn merge_cycles(c1: &Cycle, c2: &Cycle) -> Cycle {
    let head_len = cmp::max(c1.head_zees.len(), c2.head_zees.len());
    let mut head_zees: Vec<bool> = Vec::new();
    let mut periodic_zees: HashSet<usize> = HashSet::new();

    for i in 0..head_len {
        head_zees.push(c1.zees(i) & c2.zees(i));
    }

    let period = num::integer::lcm(c1.period, c2.period);
    let reps = period / c1.period;
    for r in 0..reps {
        for idx in &c1.periodic_zees {
            if c2.zees(head_len + idx + r * c1.period) {
                periodic_zees.insert(idx + r * c1.period);
            }
        }
    }

    Cycle { head_zees, period, periodic_zees }
}

fn solution2<'a>(lines: impl IntoIterator<Item = &'a str>) -> usize {
    let mut lines = lines.into_iter();
    let dirs = parse_dirs(lines.next().unwrap());
    let map = parse_map(lines.skip(1));
    let names = map.keys().filter(|name| name.ends_with('A'));

    // Build cycle for each starting name and merge them together
    let final_cycle = names
        .map(|name| build_cycle(name, &map, &dirs))
        .reduce(|c1, c2| merge_cycles(&c1, &c2))
        .unwrap();

    // Search cycle for answer
    if let Some(idx) = final_cycle.head_zees.iter().position(|&x| x) {
        idx
    } else {
        final_cycle.head_zees.len() + final_cycle.periodic_zees.iter().min().unwrap()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution1(input.lines());
    println!("Part 1: {p1}");
    let p2 = solution2(input.lines());
    println!("Part 2: {p2}");
}
