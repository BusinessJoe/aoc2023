use std::{
    collections::HashMap,
    io::{self, Read},
};

fn parse<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<Vec<u8>> {
    lines
        .into_iter()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn shift(grid: &mut [Vec<u8>], coords: &[(usize, usize)]) {
    let mut next_idx = 0;
    let mut open_len = 0;
    for (idx, &(row, col)) in coords.iter().enumerate() {
        match grid[row][col] {
            b'O' => {
                if open_len > 0 {
                    // place rock
                    grid[row][col] = b'.';
                    let (next_row, next_col) = coords[next_idx];
                    grid[next_row][next_col] = b'O';
                    next_idx += 1;
                } else {
                    // keep rock in place
                    next_idx = idx + 1;
                    open_len = 0;
                }
            }
            b'#' => {
                next_idx = idx + 1;
                open_len = 0;
            }
            b'.' => {
                open_len += 1;
            }
            _ => unreachable!(),
        }
    }
}

fn calculate_col_load(grid: &Vec<Vec<u8>>, col: usize) -> usize {
    let mut load = 0;
    let num_rows = grid.len();
    for (row_idx, row) in grid.iter().enumerate() {
        if row[col] == b'O' {
            load += num_rows - row_idx;
        }
    }
    load
}

fn cycle_grid(grid: &mut Vec<Vec<u8>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    for col in 0..cols {
        let mut coords: Vec<(usize, usize)> = Vec::new();
        for row in 0..rows {
            coords.push((row, col));
        }
        shift(grid, &coords);
    }

    for row in 0..rows {
        let mut coords: Vec<(usize, usize)> = Vec::new();
        for col in 0..cols {
            coords.push((row, col));
        }
        shift(grid, &coords);
    }

    for col in 0..cols {
        let mut coords: Vec<(usize, usize)> = Vec::new();
        for row in (0..rows).rev() {
            coords.push((row, col));
        }
        shift(grid, &coords);
    }

    for row in 0..rows {
        let mut coords: Vec<(usize, usize)> = Vec::new();
        for col in (0..cols).rev() {
            coords.push((row, col));
        }
        shift(grid, &coords);
    }
}

pub fn print_grid(grid: &[Vec<u8>]) {
    let grid_rows: Vec<String> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|n| match n {
                    b'O' => 'O',
                    b'#' => '#',
                    b'.' => '.',
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let grid_str = grid_rows.join("\n");

    println!("{grid_str}\n");
}

pub fn solution_1<'a>(lines: impl IntoIterator<Item = &'a str>) -> usize {
    let mut grid = parse(lines);
    let rows = grid.len();
    let cols = grid[0].len();

    for col in 0..cols {
        let mut coords: Vec<(usize, usize)> = Vec::new();
        for row in 0..rows {
            coords.push((row, col));
        }
        shift(&mut grid, &coords);
    }

    let mut p1 = 0;
    let cols = grid[0].len();
    for col in 0..cols {
        p1 += calculate_col_load(&grid, col);
    }
    p1
}

pub fn solution_2<'a>(lines: impl IntoIterator<Item = &'a str>) -> usize {
    let mut grid = parse(lines);

    let mut seen = HashMap::new();

    for i in 0..1_000_000_000 {
        if let Some(initial) = seen.get(&grid) {
            let cycle_len = i - initial;
            let remaining = 1_000_000_000 - i;

            for _ in 0..remaining % cycle_len {
                cycle_grid(&mut grid);
            }
            break;
        }
        seen.insert(grid.clone(), i);
        cycle_grid(&mut grid);
    }

    let mut p2 = 0;
    let cols = grid[0].len();
    for col in 0..cols {
        p2 += calculate_col_load(&grid, col);
    }
    p2
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution_1(input.lines());
    println!("Part 1: {p1}");
    let p2 = solution_2(input.lines());
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let p1 = solution_1(include_str!("../input.txt").lines());
        assert_eq!(105623, p1);
    }

    #[test]
    fn test_part_2() {
        let p2 = solution_2(include_str!("../input.txt").lines());
        assert_eq!(98029, p2);
    }
}
