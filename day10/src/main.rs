use std::{
    cmp,
    collections::{HashMap, HashSet},
    io::{self, Read},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn step(&self, (row, col): (usize, usize)) -> (i32, i32) {
        let row: i32 = row.try_into().unwrap();
        let col: i32 = col.try_into().unwrap();
        match self {
            Self::North => (row - 1, col),
            Self::East => (row, col + 1),
            Self::South => (row + 1, col),
            Self::West => (row, col - 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    fn outgoing(&self, incoming: Dir) -> Option<Dir> {
        Some(match (self, incoming) {
            (Self::Vertical, Dir::North) => Dir::South,
            (Self::Vertical, Dir::South) => Dir::North,
            (Self::Horizontal, Dir::West) => Dir::East,
            (Self::Horizontal, Dir::East) => Dir::West,
            (Self::NorthEast, Dir::North) => Dir::East,
            (Self::NorthEast, Dir::East) => Dir::North,
            (Self::NorthWest, Dir::North) => Dir::West,
            (Self::NorthWest, Dir::West) => Dir::North,
            (Self::SouthWest, Dir::South) => Dir::West,
            (Self::SouthWest, Dir::West) => Dir::South,
            (Self::SouthEast, Dir::South) => Dir::East,
            (Self::SouthEast, Dir::East) => Dir::South,
            _ => return None,
        })
    }

    fn connects(&self, incoming: Dir) -> bool {
        self.outgoing(incoming).is_some()
    }
}

struct LoopIterator<'a> {
    tiles: &'a [Vec<Tile>],
    start: (usize, usize),
    curr: (usize, usize),
    outgoing: Dir,
    first: bool,
}

impl<'a> LoopIterator<'a> {
    fn new(tiles: &'a [Vec<Tile>], start: (usize, usize), outgoing: Dir) -> Self {
        Self {
            tiles,
            start,
            curr: start,
            outgoing,
            first: true,
        }
    }
}

impl<'a> Iterator for LoopIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.start);
        }

        let (r, c) = self.outgoing.step(self.curr);
        self.curr = (r.try_into().unwrap(), c.try_into().unwrap());

        self.outgoing = self.tiles[self.curr.0][self.curr.1]
            .outgoing(self.outgoing.opposite())
            .unwrap();

        if self.curr == self.start {
            None
        } else {
            Some(self.curr)
        }
    }
}

fn parse_tiles<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<Vec<Tile>> {
    lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Tile::Vertical,
                    '-' => Tile::Horizontal,
                    'L' => Tile::NorthEast,
                    'J' => Tile::NorthWest,
                    '7' => Tile::SouthWest,
                    'F' => Tile::SouthEast,
                    '.' => Tile::Ground,
                    'S' => Tile::Start,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn find_start(tiles: &[Vec<Tile>]) -> (usize, usize) {
    for (row_idx, row) in tiles.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if *tile == Tile::Start {
                return (row_idx, col_idx);
            }
        }
    }
    panic!();
}

fn find_distances(
    tiles: &[Vec<Tile>],
    start: (usize, usize),
    dir: Dir,
) -> HashMap<(usize, usize), usize> {
    LoopIterator::new(tiles, start, dir)
        .enumerate()
        .map(|(dist, coord)| (coord, dist))
        .collect()
}

// Replace start with appropriate tile.
fn replace_start(tiles: &mut [Vec<Tile>], (srow, scol): (usize, usize)) {
    let num_rows = tiles.len();
    let num_cols = tiles[0].len();

    let mut connections: Vec<Dir> = Vec::new();
    for outgoing in [Dir::North, Dir::South, Dir::East, Dir::West] {
        let (next_row, next_col) = outgoing.step((srow, scol));
        if 0 <= next_row
            && next_row < i32::try_from(num_rows).unwrap()
            && 0 <= next_col
            && next_col < i32::try_from(num_cols).unwrap()
            && tiles[usize::try_from(next_row).unwrap()][usize::try_from(next_col).unwrap()]
                .connects(outgoing.opposite())
        {
            connections.push(outgoing);
        }
    }
    let connections: [Dir; 2] = connections.try_into().unwrap();

    tiles[srow][scol] = match connections {
        [Dir::North, Dir::South] => Tile::Vertical,
        [Dir::East, Dir::West] => Tile::Horizontal,
        [Dir::North, Dir::East] => Tile::NorthEast,
        [Dir::North, Dir::West] => Tile::NorthWest,
        [Dir::South, Dir::East] => Tile::SouthEast,
        [Dir::South, Dir::West] => Tile::SouthWest,
        _ => panic!("{:?}", connections),
    };
}

pub fn solution<'a>(lines: impl IntoIterator<Item = &'a str>) -> (usize, usize) {
    let mut tiles = parse_tiles(lines);
    let num_rows = tiles.len();
    let num_cols = tiles[0].len();
    let (srow, scol) = find_start(&tiles);
    // We have the coordinates of the start tile so we can replace the start tile
    // with a regular tile without worrying.
    replace_start(&mut tiles, (srow, scol));

    // Part 1
    let mut distances = [Dir::North, Dir::South, Dir::East, Dir::West]
        .into_iter()
        .filter(|&outgoing| tiles[srow][scol].connects(outgoing))
        .map(|outgoing| find_distances(&tiles, (srow, scol), outgoing));

    let first = distances.next().unwrap();
    let second = distances.next().unwrap();

    let p1 = first
        .iter()
        .map(|(coord, val)| cmp::min(*val, *second.get(coord).unwrap()))
        .max()
        .unwrap();

    // Part 2
    // First convert non-loop tiles to ground
    let outgoing = [Dir::North, Dir::East, Dir::South, Dir::West]
        .into_iter()
        .find(|&outgoing| tiles[srow][scol].connects(outgoing))
        .unwrap();
    let coords: HashSet<(usize, usize)> =
        LoopIterator::new(&tiles, (srow, scol), outgoing).collect();
    for row in 0..num_rows {
        for col in 0..num_cols {
            if !coords.contains(&(row, col)) {
                tiles[row][col] = Tile::Ground;
            }
        }
    }

    // To find interior points, we cast a ray from the left edge of the board to the right, keeping
    // track of the number of times we intersect with the loop. When we find a non-loop point
    // (which will be Ground since we've previously converted all non-loop points to Ground) we
    // know it's inside the loop if and only if the number of intersections is odd.
    //
    // There's a literal edge case when we encounter a horizontal edge. We bias our ray to the
    // "upper half" of a tile so that we only intersect with Vertical, NorthEast, and NorthWest
    // tiles.
    let mut p2 = 0;
    for row in tiles.iter().take(num_rows) {
        let mut intersections = 0;
        for tile in row.iter() {
            match tile {
                Tile::Vertical | Tile::NorthEast | Tile::NorthWest => {
                    intersections += 1;
                }
                Tile::Ground => {
                    if intersections % 2 == 1 {
                        p2 += 1;
                    }
                }
                _ => {}
            }
        }
    }

    (p1, p2)
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let (p1, p2) = solution(input.lines());
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt").lines();
        let (p1, _) = solution(input);
        assert_eq!(6867, p1);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt").lines();
        let (_, p2) = solution(input);
        assert_eq!(595, p2);
    }
}
