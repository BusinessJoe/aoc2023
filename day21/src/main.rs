use std::{
    collections::VecDeque,
    io::{self, Read},
};

struct Grid {
    tiles: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

        Self {
            rows: tiles.len(),
            cols: tiles[0].len(),
            tiles,
        }
    }

    fn find_start(&self) -> (usize, usize) {
        for (row_idx, row) in self.tiles.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                if *tile == b'S' {
                    return (row_idx, col_idx);
                }
            }
        }
        panic!()
    }

    fn contains_signed(&self, pos: (i64, i64)) -> bool {
        let (row, col) = pos;
        0 <= row && row < self.rows as i64 && 0 <= col && col < self.cols as i64
    }

    fn signed_get(&self, (row, col): (i64, i64)) -> Option<u8> {
        if self.contains_signed((row, col)) {
            Some(self.tiles[row as usize][col as usize])
        } else {
            None
        }
    }
}

pub fn solution_1(input: &str) -> usize {
    let grid = Grid::parse(input);
    let (s_row, s_col) = grid.find_start();

    let mut min_steps: Vec<Vec<Option<usize>>> = vec![vec![None; grid.cols]; grid.rows];
    min_steps[s_row][s_col] = Some(0);
    let mut pos_queue: VecDeque<(i64, i64)> = VecDeque::from([(s_row as i64, s_col as i64)]);

    while let Some((row, col)) = pos_queue.pop_front() {
        let steps = min_steps[row as usize][col as usize].unwrap();
        match min_steps[row as usize][col as usize] {
            Some(n) if n < steps => {
                continue;
            }
            _ => {
                min_steps[row as usize][col as usize] = Some(steps);
            }
        }

        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let row = row + dr;
            let col = col + dc;
            if steps < 64 && grid.signed_get((row, col)) == Some(b'.') {
                if let Some(n) = min_steps[row as usize][col as usize] {
                    if steps + 1 < n {
                        min_steps[row as usize][col as usize] = Some(steps + 1);
                        pos_queue.push_back((row, col));
                    }
                } else {
                    min_steps[row as usize][col as usize] = Some(steps + 1);
                    pos_queue.push_back((row, col));
                }
            }
        }
    }

    let mut p1 = 0;
    for row in min_steps {
        for optional_steps in row {
            if let Some(steps) = optional_steps {
                if steps % 2 == 0 {
                    p1 += 1;
                }
            }
        }
    }

    p1
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
