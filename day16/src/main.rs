use std::{io::{self, Read}, cmp};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Dir {
    fn step(self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Self::Up => (pos.0 - 1, pos.1),
            Self::Down => (pos.0 + 1, pos.1),
            Self::Left => (pos.0, pos.1 - 1),
            Self::Right => (pos.0, pos.1 + 1),
        }
    }
}

#[derive(Clone)]
struct Tile {
    b: u8,
    dirs: Vec<Dir>,
}

impl Tile {
    fn new(b: u8) -> Self {
        Self { b, dirs: vec![] }
    }
}

#[derive(Debug)]
struct Beam {
    pos: (i32, i32),
    dir: Dir,
}

impl Beam {
    fn step(mut self, grid: &mut Grid) -> Vec<Beam> {
        if let Some(tile) = grid.get_mut(self.pos) {
            if tile.dirs.contains(&self.dir) {
                return vec![];
            }

            tile.dirs.push(self.dir);

            match tile.b {
                b'.' => {
                    self.pos = self.dir.step(self.pos);
                    vec![self]
                }
                b'\\' => {
                    self.dir = match self.dir {
                        Dir::Up => Dir::Left,
                        Dir::Down => Dir::Right,
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                    };
                    self.pos = self.dir.step(self.pos);
                    vec![self]
                }
                b'/' => {
                    self.dir = match self.dir {
                        Dir::Up => Dir::Right,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Down,
                        Dir::Right => Dir::Up,
                    };
                    self.pos = self.dir.step(self.pos);
                    vec![self]
                }
                b'-' if self.dir == Dir::Left || self.dir == Dir::Right => {
                    self.pos = self.dir.step(self.pos);
                    vec![self]
                }
                b'|' if self.dir == Dir::Up || self.dir == Dir::Down => {
                    self.pos = self.dir.step(self.pos);
                    vec![self]
                }
                b'-' => {
                    let other = Self {
                        pos: (self.pos.0, self.pos.1 + 1),
                        dir: Dir::Right,
                    };

                    self.pos = (self.pos.0, self.pos.1 - 1);
                    self.dir = Dir::Left;

                    vec![self, other]
                }
                b'|' => {
                    let other = Self {
                        pos: (self.pos.0 + 1, self.pos.1),
                        dir: Dir::Down,
                    };

                    self.pos = (self.pos.0 - 1, self.pos.1);
                    self.dir = Dir::Up;

                    vec![self, other]
                }
                _ => unreachable!(),
            }
        } else {
            vec![]
        }
    }
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    dims: (usize, usize),
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let tiles: Vec<Vec<Tile>> = input
            .split('\n')
            .map(|line| line.as_bytes().iter().map(|b| Tile::new(*b)).collect())
            .collect();

        let dims = (tiles.len(), tiles[0].len());

        Grid { tiles, dims }
    }

    fn get_mut(&mut self, pos: (i32, i32)) -> Option<&mut Tile> {
        self.tiles
            .get_mut(pos.0 as usize)
            .map(|row| row.get_mut(pos.1 as usize))
            .flatten()
    }

    fn count_energized(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|tile| !tile.dirs.is_empty()).count())
            .sum()
    }
}

fn calculate_energized(mut grid: Grid, initial_beam: Beam) -> usize {
    let mut beams = vec![initial_beam];

    while let Some(beam) = beams.pop() {
        beams.extend(beam.step(&mut grid));
    }

    grid.count_energized()
}

pub fn solution_1(input: &str) -> usize {
    let grid = Grid::parse(input);
    calculate_energized(
        grid,
        Beam {
            pos: (0, 0),
            dir: Dir::Right,
        },
    )
}

pub fn solution_2(input: &str) -> usize {
    let grid = Grid::parse(input);
    let dims = &grid.dims;

    let mut max = 0;
    for row in 0..dims.0 {
        max = cmp::max(max, calculate_energized(grid.clone(), Beam { pos: (row as i32, 0), dir: Dir::Right }));
        max = cmp::max(max, calculate_energized(grid.clone(), Beam { pos: (row as i32, dims.1 as i32 - 1), dir: Dir::Left }));
    }
    for col in 0..dims.1 {
        max = cmp::max(max, calculate_energized(grid.clone(), Beam { pos: (0, col as i32), dir: Dir::Down }));
        max = cmp::max(max, calculate_energized(grid.clone(), Beam { pos: (dims.0 as i32 - 1, col as i32), dir: Dir::Up }));
    }

    max
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
