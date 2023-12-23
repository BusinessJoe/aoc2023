use std::{
    cmp,
    collections::{HashMap, HashSet},
    io::{self, Read},
};

struct Grid {
    tiles: Vec<Vec<u8>>,
}

type Coord = (usize, usize);
type SignedCoord = (i32, i32);
type DistCache = HashMap<Coord, usize>;

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        Self { tiles }
    }

    fn rows(&self) -> usize {
        self.tiles.len()
    }

    fn cols(&self) -> usize {
        self.tiles[0].len()
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

    /// Returns distance of path and coordinate of intersection/endpoint
    fn follow_path(&self, start: SignedCoord, mut prev: SignedCoord) -> (usize, SignedCoord) {
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
                    } else if tile == b'>' && dc == 1
                        || tile == b'<' && dc == -1
                        || tile == b'v' && dr == 1
                        || tile == b'^' && dr == -1
                    {
                        (row, col) = (row + 2 * dr, col + 2 * dc);
                        dist += 2;
                        return (dist, (row, col));
                    }
                }
            }

            if end {
                return (dist, (row, col));
            }
        }
    }

    fn longest_distance_helper(&self, start: SignedCoord, prev: SignedCoord) -> usize {
        let (dist, end) = self.follow_path(start, prev);

        // We're at an intersection or done?
        let mut dists: Vec<usize> = Vec::new();
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (end.0 + dr, end.1 + dc);
            if let Some(tile) = self.get_signed(next) {
                if tile == b'>' && dc == 1
                    || tile == b'<' && dc == -1
                    || tile == b'v' && dr == 1
                    || tile == b'^' && dr == -1
                {
                    dists.push(self.longest_distance_helper(next, end));
                }
            }
        }

        dist + dists.into_iter().max().unwrap_or_default()
    }
}

pub fn solution_1(input: &str) -> usize {
    let grid = Grid::parse(input);
    grid.longest_distance_helper((0, 1), (0, 0)) - 1
}

pub fn solution_2(input: &str) -> usize {
    0
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
