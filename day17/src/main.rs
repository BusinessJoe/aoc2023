use std::{
    cmp,
    collections::VecDeque,
    io::{self, Read},
};

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect())
        .collect()
}

#[derive(Debug, Clone, Copy)]
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

    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn turn_back(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn idx(self) -> usize {
        match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 3,
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

#[derive(Debug)]
struct Head {
    pos: (i32, i32),
    dir: Dir,
    hist: u8,
    prev_min: u64,
}

// Lower min is best, followed by lower hist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CacheEntry {
    min: u64,
}

impl CacheEntry {
    fn max() -> Self {
        Self {
            min: u64::MAX,
        }
    }
}

fn search(grid: &[Vec<u8>], cache: &mut [[[CacheEntry; 3]; 4]]) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut heads = VecDeque::new();
    heads.push_front(Head {
        pos: (0, 1),
        dir: Dir::Right,
        hist: 0,
        prev_min: 0,
    });
    heads.push_front(Head {
        pos: (1, 0),
        dir: Dir::Down,
        hist: 0,
        prev_min: 0,
    });

    while let Some(head) = heads.pop_back() {
        if !(0 <= head.pos.0
            && head.pos.0 < rows as i32
            && 0 <= head.pos.1
            && head.pos.1 < cols as i32)
        {
            continue;
        }
        let row = head.pos.0 as usize;
        let col = head.pos.1 as usize;

        let entries = &mut cache[row *cols + col][head.dir.idx()][head.hist as usize..];
        let new_entry = CacheEntry {
            min: head.prev_min + grid[row][col] as u64,
        };

        let cache_hit = entries[0] <= new_entry;

        if cache_hit {
            continue;
        }

        for e in entries.iter_mut() {
            if new_entry < *e {
               *e = new_entry; 
            }
        }

        if head.hist < 2 {
            heads.push_front(Head {
                dir: head.dir,
                hist: head.hist + 1,
                pos: head.pos.mv(head.dir),
                prev_min: new_entry.min,
            });
        }

        heads.push_front(Head {
            dir: head.dir.turn_left(),
            hist: 0,
            pos: head.pos.mv(head.dir.turn_left()),
            prev_min: new_entry.min,
        });

        heads.push_front(Head {
            dir: head.dir.turn_right(),
            hist: 0,
            pos: head.pos.mv(head.dir.turn_right()),
            prev_min: new_entry.min,
        });
    }
}

/*
fn print_solution(grid: &[Vec<u8>], cache: &[[CacheEntry; 4]]) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut chars = vec![vec!['0'; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            chars[r][c] = (b'0' + grid[r][c]).into()
        }
    }

    let mut pos = (rows as i32 - 1, cols as i32 - 1);
    loop {
        let row = pos.0 as usize;
        let col = pos.1 as usize;
        let dir = cache[row * cols + col]
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(idx, _)| match idx {
                0 => Dir::Up,
                1 => Dir::Down,
                2 => Dir::Left,
                3 => Dir::Right,
                _ => unreachable!(),
            })
            .unwrap();

        chars[row][col] = match dir {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
        };

        pos = pos.mv(dir.turn_back());

        if pos == (0, 0) {
            break;
        }
    }

    let lines: Vec<String> = chars
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect();
    println!("{}", lines.join("\n"));
}
*/

pub fn solution_1(input: &str) -> u64 {
    let grid = parse_grid(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut cache: Vec<[[CacheEntry; 3]; 4]> = vec![[[CacheEntry::max(); 3]; 4]; rows * cols];

    search(&grid, &mut cache);
    //print_solution(&grid, &cache);

    let min = cache[cache.len() - 1]
        .iter()
        .map(|arr| arr.iter().min().unwrap())
        .min()
        .unwrap();
    min.min
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
