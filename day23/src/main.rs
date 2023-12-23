use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

struct Grid {
    tiles: Vec<Vec<u8>>,
    path_cache: HashMap<SignedCoord, (usize, SignedCoord, SignedCoord)>,
}

type SignedCoord = (i32, i32);

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        Self {
            tiles,
            path_cache: HashMap::new(),
        }
    }

    fn rows(&self) -> usize {
        self.tiles.len()
    }

    fn cols(&self) -> usize {
        self.tiles[0].len()
    }

    fn end(&self) -> SignedCoord {
        (self.rows() as i32 - 1, self.cols() as i32 - 2)
    }

    fn get_signed(&self, coord: SignedCoord) -> Option<u8> {
        if 0 <= coord.0
            && coord.0 < self.rows() as i32
            && 0 <= coord.1
            && coord.1 < self.cols() as i32
        {
            Some(self.tiles[coord.0 as usize][coord.1 as usize])
        } else {
            None
        }
    }

    /// Returns distance of path and coordinate of intersection/endpoint as well as prev coord
    fn follow_path(
        &mut self,
        start: SignedCoord,
        mut prev: SignedCoord,
    ) -> (usize, SignedCoord, SignedCoord) {
        if let Some(output) = self.path_cache.get(&start) {
            return *output;
        }

        let mut dist = 1;

        let mut row = start.0 as i32;
        let mut col = start.1 as i32;

        loop {
            let mut end = true;

            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next = (row + dr, col + dc);
                if next == prev {
                    continue;
                }

                if let Some(tile) = self.get_signed(next) {
                    if tile == b'.' {
                        prev = (row, col);
                        (row, col) = next;
                        dist += 1;
                        end = false;
                    } else if [b'>', b'<', b'v', b'^'].contains(&tile) {
                        prev = (row + dr, col + dc);
                        (row, col) = (row + 2 * dr, col + 2 * dc);
                        dist += 2;
                        let output = (dist, (row, col), prev);
                        self.path_cache.insert(start, output);
                        return output;
                    }
                }
            }

            if end {
                let output = (dist, (row, col), prev);
                self.path_cache.insert(start, output);
                return output;
            }
        }
    }

    fn longest_distance_helper_1(
        &mut self,
        start: SignedCoord,
        prev: SignedCoord,
        visited: &mut HashSet<SignedCoord>,
    ) -> Option<usize> {
        let (dist, end, prev) = self.follow_path(start, prev);

        if visited.contains(&end) {
            return None;
        }

        visited.insert(end);

        if end == self.end() {
            visited.remove(&end);
            return Some(dist);
        }

        // We're at an intersection or done?
        let mut dists: Vec<usize> = Vec::new();
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (end.0 + dr, end.1 + dc);
            if next == prev {
                continue;
            }
            if let Some(tile) = self.get_signed(next) {
                if tile == b'>' && dc == 1
                    || tile == b'<' && dc == -1
                    || tile == b'^' && dr == -1
                    || tile == b'v' && dr == 1
                {
                    if let Some(d) = self.longest_distance_helper_1(next, end, visited) {
                        dists.push(d);
                    }
                }
            }
        }
        visited.remove(&end);

        if dists.is_empty() {
            return None;
        }

        let ret = dist + dists.into_iter().max().unwrap();
        Some(ret)
    }

    fn longest_distance_helper_2(
        &mut self,
        start: SignedCoord,
        prev: SignedCoord,
        visited: &mut HashSet<SignedCoord>,
    ) -> Option<usize> {
        let (dist, end, prev) = self.follow_path(start, prev);

        if visited.contains(&end) {
            return None;
        }

        visited.insert(end);

        if end == self.end() {
            visited.remove(&end);
            return Some(dist);
        }

        // We're at an intersection or done?
        let mut dists: Vec<usize> = Vec::new();
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (end.0 + dr, end.1 + dc);
            if next == prev {
                continue;
            }
            if let Some(tile) = self.get_signed(next) {
                if [b'>', b'<', b'v', b'^'].contains(&tile) {
                    if let Some(d) = self.longest_distance_helper_2(next, end, visited) {
                        dists.push(d);
                    }
                }
            }
        }
        visited.remove(&end);

        if dists.is_empty() {
            return None;
        }

        let ret = dist + dists.into_iter().max().unwrap();
        Some(ret)
    }
}

pub fn solution_1(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    let mut visited = HashSet::new();
    grid.longest_distance_helper_1((0, 1), (0, 0), &mut visited)
        .unwrap()
        - 1
}

pub fn solution_2(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    let mut visited = HashSet::new();
    grid.longest_distance_helper_2((0, 1), (0, 0), &mut visited)
        .unwrap()
        - 1
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
