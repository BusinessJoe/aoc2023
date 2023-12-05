use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet}, array, io};
use std::io::BufRead;

#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct MapEntry {
    pub start: usize,
    pub end: usize,
    pub dest: usize,
}
impl MapEntry {
    pub fn parse(line: &str) -> Self {
        let mut nums = line.split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap());
        let dest = nums.next().unwrap();
        let start = nums.next().unwrap();
        let len = nums.next().unwrap();

        Self {
            start,
            dest,
            end: start + len,
        }
    }

    pub fn map(&self, input: usize) -> Option<usize> {
        if input < self.start || self.end <= input {
            return None;
        }

        Some(self.dest + input - self.start)
    }
}

impl Ord for MapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.start >= other.end {
            Ordering::Greater
        } else if self.end <= other.start {
            Ordering::Less
        } else {
            self.start.cmp(&other.start)
        }
    }
}

fn get_seeds(seeds_line: &str) -> Vec<usize> {
    let no_header = seeds_line.split_once(':').unwrap().1;
    no_header.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect()
}

fn generate_maps(lines: &[String]) -> Vec<Vec<MapEntry>> {
    let mut lines = lines.iter();

    // Pop empty line.
    lines.next();

    let mut maps: Vec<Vec<MapEntry>> = Vec::new();

    lines.next(); // Consume title
    maps.push(Vec::new());
    while let Some(line) = lines.next() {
        if line.is_empty() {
            lines.next(); // Consume title
            maps.push(Vec::new());
            continue;
        }
        let map = MapEntry::parse(line);
        let idx = maps.len() - 1;
        maps[idx].push(map); 
    }

    maps
}

fn solution1(lines: &[String]) -> usize {
    let seeds_line = &lines[0];
    let seeds = get_seeds(&seeds_line);

    let maps = generate_maps(&lines[1..]);

    seeds.into_iter().map(|s| {
        maps.iter().fold(s, |s, entries| {
            for e in entries {
                if let Some(s) = e.map(s) {
                    return s;
                }
            }
            return s;
        })
    }).min().unwrap()
}

fn solution2(lines: &[String]) -> usize {
    let seeds_line = &lines[0];
    let seeds = get_seeds(&seeds_line);

    let maps = generate_maps(&lines[1..]);

    seeds.chunks_exact(2).map(|chunk| {
        let range = chunk[0]..chunk[0] + chunk[1];
        range.into_iter().map(|s| {
            maps.iter().fold(s, |s, entries| {
                for e in entries {
                    if let Some(s) = e.map(s) {
                        return s;
                    }
                }
                return s;
            })
        }).min().unwrap()
    }).min().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let p1 = solution1(&lines);
    let p2 = solution2(&lines);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
