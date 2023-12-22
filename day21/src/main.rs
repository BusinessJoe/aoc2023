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
    fn parse(input: &str) -> (Self, (usize, usize)) {
        let tiles: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

        let mut grid = Self {
            rows: tiles.len(),
            cols: tiles[0].len(),
            tiles,
        };
        // find and replace start
        let (row, col) = grid.find_start();
        grid.tiles[row][col] = b'.';
        (grid, (row, col))
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

struct MinSteps {
    max: usize,
    min_steps: Vec<Vec<Option<usize>>>,
}

impl MinSteps {
    fn get_min_steps(grid: &Grid, (s_row, s_col): (usize, usize)) -> Vec<Vec<Option<usize>>> {
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
                if grid.signed_get((row, col)) == Some(b'.') {
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

        min_steps
    }

    fn get_max_steps(min_steps: &Vec<Vec<Option<usize>>>) -> usize {
        min_steps.iter().flatten().flatten().copied().max().unwrap()
    }

    pub fn new(grid: &Grid, (s_row, s_col): (usize, usize)) -> Self {
        let min_steps = Self::get_min_steps(grid, (s_row, s_col));
        let max = Self::get_max_steps(&min_steps);
        Self { min_steps, max }
    }

    pub fn count<F>(&self, predicate: F) -> usize
    where
        F: Fn(usize) -> bool,
    {
        self.min_steps
            .iter()
            .flatten()
            .flatten()
            .copied()
            .filter(|&x| predicate(x))
            .count()
    }
}

pub fn solution_1(input: &str) -> usize {
    let (grid, (s_row, s_col)) = Grid::parse(input);

    let min_steps = MinSteps::new(&grid, (s_row, s_col));

    min_steps.count(|steps| steps <= 64 && steps % 2 == 0)
}

pub fn solution_2(input: &str, max_steps: usize) -> usize {
    let (grid, (s_row, s_col)) = Grid::parse(input);
    assert_eq!(grid.rows, grid.cols);

    let steps_center = MinSteps::new(&grid, (s_row, s_col));
    let corner_steps = [
        MinSteps::new(&grid, (0, 0)),
        MinSteps::new(&grid, (0, grid.cols - 1)),
        MinSteps::new(&grid, (grid.rows - 1, 0)),
        MinSteps::new(&grid, (grid.rows - 1, grid.cols - 1)),
    ];

    dbg!(corner_steps[0].max);
    assert!(corner_steps.iter().all(|cs| cs.max == corner_steps[0].max));

    let edge_steps = [
        MinSteps::new(&grid, (grid.rows / 2, 0)),
        MinSteps::new(&grid, (grid.rows / 2, grid.cols - 1)),
        MinSteps::new(&grid, (0, grid.cols / 2)),
        MinSteps::new(&grid, (grid.rows - 1, grid.cols / 2)),
    ];

    dbg!(edge_steps[0].max);
    assert!(edge_steps.iter().all(|es| es.max == edge_steps[0].max));

    // Corner steps have same count, edge steps have opposite count.
    let center_count_all_even = steps_center.count(|steps| steps % 2 == 0);
    let center_count_all_odd = steps_center.count(|steps| steps % 2 != 0);

    let edge_count_all_even = edge_steps[0].count(|steps| steps % 2 == 0);
    let edge_count_all_odd = edge_steps[0].count(|steps| steps % 2 != 0);

    // Center
    let center = if max_steps % 2 == 0 {
        center_count_all_even
    } else {
        center_count_all_odd
    };

    // Orthogonal
    let mut ortho = 0;
    {
        let center_to_orthogonal_grid = grid.rows / 2 + 1;
        let mut remaining = max_steps - center_to_orthogonal_grid;
        loop {
            // All edge steps have same max count, just check against any one
            if remaining >= edge_steps[0].max {
                // We can reach all the cells

                // Edge steps are opposite parity from center
                if remaining % 2 == 0 {
                    ortho += 4 * edge_count_all_even;
                } else {
                    ortho += 4 * edge_count_all_odd;
                }
            } else {
                for es in &edge_steps {
                    ortho += es.count(|steps| steps <= remaining && steps % 2 == remaining % 2);
                }
            }

            if remaining < grid.rows {
                break;
            } else {
                remaining -= grid.rows;
            }
        }
    }

    // Quadrant
    let mut quad = 0;
    {
        let center_to_quadrant_grid = 2 * (grid.rows / 2 + 1);
        for col in 0.. {
            if max_steps < center_to_quadrant_grid + col * grid.cols {
                break;
            }
            let mut remaining = max_steps - center_to_quadrant_grid - col * grid.cols;
            loop {
                // All corner steps have same max count, just check against any one
                if remaining >= corner_steps[0].max {
                    // We can reach all the cells

                    if remaining % 2 == 0 {
                        quad += 4 * center_count_all_even;
                    } else {
                        quad += 4 * center_count_all_odd;
                    }
                } else {
                    for cs in &corner_steps {
                        quad += cs.count(|steps| steps <= remaining && steps % 2 == remaining % 2);
                    }
                }

                if remaining < grid.rows {
                    break;
                } else {
                    remaining -= grid.rows;
                }
            }
        }
    }

    dbg!(center, ortho, quad);
    center + ortho + quad
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution_1(&input);
    println!("Part 1: {p1}");
    let p2 = solution_2(&input, 26501365);
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_2_blank_5x5() {
        for i in 5..100 {
            let p2 = solution_2(include_str!("../blank5x5.txt"), i);
            assert_eq!((i + 1) * (i + 1), p2, "i={}", i);
        }
    }

    #[test]
    fn test_solution_2_blank() {
        let i = 381;
        let p2 = solution_2(include_str!("../blank131.txt"), i);
        assert_eq!((i + 1) * (i + 1), p2, "i={}", i);
    }
}
