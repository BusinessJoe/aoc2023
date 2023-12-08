use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::{cmp, io};

type Map = HashMap<String, (String, String)>;

enum Direction {
    Left,
    Right,
}

fn parse_line(line: &str) -> (String, String, String) {
    (
        line[0..3].to_string(),
        line[7..10].to_string(),
        line[12..15].to_string(),
    )
}

fn solution1(lines: impl IntoIterator<Item = impl AsRef<str>>) -> u32 {
    let mut lines = lines.into_iter();
    let dirs: Vec<Direction> = lines
        .next()
        .unwrap()
        .as_ref()
        .as_bytes()
        .iter()
        .map(|&c| {
            if c == b'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect();

    let mut map: Map = HashMap::new();
    lines
        .skip(1)
        .map(|line| parse_line(line.as_ref()))
        .for_each(|(name, left, right)| {
            map.insert(name, (left, right));
        });

    let mut name = &"AAA".to_string();
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

    i.try_into().unwrap()
}

fn solved(names: &[String]) -> bool {
    names.into_iter().all(|n| n.ends_with('Z'))
}

#[derive(Debug)]
struct Cycle {
    period_len: usize,
    head_zees: Vec<bool>,
    periodic_zees: HashSet<usize>,
}

impl Cycle {
    fn zees(&self, i: usize) -> bool {
        if i < self.head_zees.len() {
            self.head_zees[i]
        } else {
            let idx = (i - self.head_zees.len()) % self.period_len;
            self.periodic_zees.contains(&idx)
        }
    }
}

fn build_cycle(mut name: String, map: &Map, dirs: &[Direction]) -> Cycle {
    let mut zees: Vec<bool> = Vec::new();

    let mut idx = 0;
    let mut seen: HashMap<String, usize> = HashMap::new();
    while seen.get(&name).is_none() {
        seen.insert(name.to_string(), idx);
        for dir in dirs {
            zees.push(name.ends_with('Z'));
            let (left, right) = map.get(&name).unwrap();
            name = match dir {
                Direction::Left => left.clone(),
                Direction::Right => right.clone(),
            };
            idx += 1;
        }
    }

    let split_idx = seen.get(&name).unwrap();
    let period_len = idx - split_idx;
    let periodic_zees_vec = zees.split_off(*split_idx);
    debug_assert_eq!(period_len, periodic_zees_vec.len());
    let mut periodic_zees = HashSet::new();
    for (i, b) in periodic_zees_vec.into_iter().enumerate() {
        if b {
            periodic_zees.insert(i);
        }
    }

    debug_assert_ne!(0, periodic_zees.len());
    Cycle {
        period_len,
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

    let period_len = num::integer::lcm(c1.period_len, c2.period_len);
    let reps = period_len / c1.period_len;
    for r in 0..reps {
        for idx in &c1.periodic_zees {
            if c2.zees(head_len + idx + r * c1.period_len) {
                periodic_zees.insert(idx + r * c1.period_len);
            }
        }
    }

    Cycle {
        period_len,
        head_zees,
        periodic_zees,
    }
}

fn solution2(lines: impl IntoIterator<Item = impl AsRef<str>>) -> u64 {
    let mut lines = lines.into_iter();
    let dirs: Vec<Direction> = lines
        .next()
        .unwrap()
        .as_ref()
        .as_bytes()
        .iter()
        .map(|&c| {
            if c == b'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut names: Vec<String> = Vec::new();

    lines
        .skip(1)
        .map(|line| parse_line(line.as_ref()))
        .for_each(|(name, left, right)| {
            if name.ends_with('A') {
                names.push(name.clone());
            }
            map.insert(name, (left, right));
        });

    // Build loop for each name
    let final_cycle = names
        .iter()
        .map(|name| build_cycle(name.clone(), &map, &dirs))
        .reduce(|c1, c2| merge_cycles(&c1, &c2))
        .unwrap();

    println!(
        "looking for answer in cycle with period length {}",
        final_cycle.period_len
    );
    (final_cycle.head_zees.len() + final_cycle.periodic_zees.iter().min().unwrap())
        .try_into()
        .unwrap()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let p1 = solution1(&lines);
    println!("Part 1: {p1}");
    let p2 = solution2(&lines);
    println!("Part 2: {p2}");
}
