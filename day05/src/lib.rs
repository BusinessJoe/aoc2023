use std::cmp;

#[derive(Debug, PartialEq, Eq)]
struct MapEntry {
    pub start: usize,
    pub end: usize,
    pub dest: usize,
}
impl MapEntry {
    pub fn parse(line: &str) -> Self {
        let mut nums = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap());
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

struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn add(&mut self, entry: MapEntry) {
        self.entries.push(entry);
    }

    pub fn sort(&mut self) {
        self.entries.sort_by_key(|e| e.start);
    }

    fn map_id(&self, id: usize) -> usize {
        for e in &self.entries {
            if let Some(id) = e.map(id) {
                return id;
            }
        }
        id
    }

    pub fn map_interval(&self, interval: (usize, usize)) -> Vec<(usize, usize)> {
        let mut out = vec![];
        let mut remaining = interval;
        let mut start = interval.0;
        let mut end = 0;

        self.entries
            .iter()
            .skip_while(|entry| entry.end < interval.0)
            .take_while(|entry| interval.1 >= entry.start)
            .for_each(|entry| {
                debug_assert_ne!(remaining.0, remaining.1);

                start = cmp::max(remaining.0, entry.start);
                if start > remaining.0 {
                    // handle remainder
                    out.push((remaining.0, start));
                }

                end = cmp::min(remaining.1, entry.end);
                remaining.0 = end;
                out.push((
                    start + entry.dest - entry.start,
                    end + entry.dest - entry.start,
                ));
            });
        if remaining.0 != remaining.1 {
            out.push(remaining);
        }

        if out.is_empty() {
            vec![interval]
        } else {
            out.sort_unstable();
            out
        }
    }
}

fn get_seeds(seeds_line: &str) -> Vec<usize> {
    let no_header = seeds_line.split_once(':').unwrap().1;
    no_header
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn generate_maps(lines: impl IntoIterator<Item = impl AsRef<str>>) -> Vec<Map> {
    let mut lines = lines.into_iter();

    // Pop empty line.
    lines.next();

    let mut maps: Vec<Map> = Vec::new();

    lines.next(); // Consume title
    maps.push(Map::new());
    while let Some(line) = lines.next() {
        if line.as_ref().is_empty() {
            lines.next(); // Consume title
            maps.push(Map::new());
            continue;
        }
        let map_entry = MapEntry::parse(line.as_ref());
        let idx = maps.len() - 1;
        maps[idx].add(map_entry);
    }

    maps.iter_mut().for_each(Map::sort);

    maps
}

pub fn solution1(lines: impl IntoIterator<Item = impl AsRef<str>>) -> usize {
    let mut lines = lines.into_iter();
    let seeds_line = lines.next().unwrap();
    let seeds = get_seeds(seeds_line.as_ref());

    let maps = generate_maps(lines);

    seeds
        .into_iter()
        .map(|s| maps.iter().fold(s, |s, map| map.map_id(s)))
        .min()
        .unwrap()
}

pub fn solution2(lines: impl IntoIterator<Item = impl AsRef<str>>) -> usize {
    let mut lines = lines.into_iter();
    let seeds_line = lines.next().unwrap();
    let seeds = get_seeds(seeds_line.as_ref());

    let maps = generate_maps(lines);

    seeds
        .chunks_exact(2)
        .map(|chunk| {
            let interval = (chunk[0], chunk[0] + chunk[1]);
            let intervals = maps.iter().fold(vec![interval], |intervals, map| {
                intervals
                    .into_iter()
                    .flat_map(|interval| map.map_interval(interval))
                    .collect()
            });
            intervals.into_iter().map(|i| i.0).min().unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_interval() {
        let mut map = Map::new();
        map.add(MapEntry {
            start: 10,
            end: 20,
            dest: 50,
        });

        let intervals = map.map_interval((0, 100));
        let expected: Vec<(usize, usize)> = vec![(0, 10), (20, 100), (50, 60)];

        assert_eq!(expected, intervals);
    }
}
