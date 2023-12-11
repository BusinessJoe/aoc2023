use std::io::{self, Read};

fn parse_universe<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<Vec<u8>> {
    lines
        .into_iter()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

pub fn solution<'a>(lines: impl IntoIterator<Item = &'a str>, scale: usize) -> usize {
    let universe = parse_universe(lines);
    let width = universe[0].len();

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (row_idx, row) in universe.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if *tile != b'.' {
                galaxies.push((row_idx, col_idx));
            }
        }
    }

    let mut empty_rows: Vec<usize> = Vec::new();
    for (row_idx, row) in universe.iter().enumerate() {
        if row.iter().all(|&tile| tile == b'.') {
            empty_rows.push(row_idx);
        }
    }

    let mut empty_cols: Vec<usize> = Vec::new();
    for col_idx in 0..width {
        if universe.iter().all(|row| row[col_idx] == b'.') {
            empty_cols.push(col_idx);
        }
    }

    for galaxy in &mut galaxies {
        for row in empty_rows.iter().rev() {
            if galaxy.0 > *row {
                galaxy.0 += scale;
            }
        }
        for col in empty_cols.iter().rev() {
            if galaxy.1 > *col {
                galaxy.1 += scale;
            }
        }
    }

    let mut total = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[i + 1..] {
            total += g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
        }
    }

    total
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution(input.lines(), 1);
    let p2 = solution(input.lines(), 999_999);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
