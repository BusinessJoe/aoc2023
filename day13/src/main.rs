use std::io::{self, Read};

type Terrain = Vec<Vec<u8>>;
fn parse_terrain<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<Terrain> {
    let mut all_terrain = Vec::new();
    let mut terrain: Terrain = Vec::new();
    for line in lines {
        if line.is_empty() {
            all_terrain.push(terrain);
            terrain = Vec::new();
        } else {
            terrain.push(line.as_bytes().to_vec());
        }
    }
    all_terrain.push(terrain);

    all_terrain
}

fn transpose(terrain: &Terrain) -> Terrain {
    let rows = terrain.len();
    let cols = terrain[0].len();

    let mut transposed = vec![vec![0; rows]; cols];

    for r in 0..rows {
        for c in 0..cols {
            transposed[c][r] = terrain[r][c];
        }
    }

    transposed
}

/// Returns number of mis-matches.
fn try_relect_row(row: &[u8], col: usize) -> usize {
    row[0..col]
        .iter()
        .rev()
        .zip(row[col..].iter())
        .filter(|(a, b)| a != b)
        .count()
}

/// Returns number of mis-matches.
fn try_reflect(terrain: &Terrain, col: usize) -> usize {
    terrain.iter().map(|line| try_relect_row(line, col)).sum()
}

fn find_mirror(terrain: &Terrain, defects: usize) -> usize {
    let cols = terrain[0].len();
    for col in 1..cols {
        if try_reflect(terrain, col) == defects {
            return col;
        }
    }
    0
}

pub fn solution<'a>(lines: impl IntoIterator<Item = &'a str>, defects: usize) -> usize {
    parse_terrain(lines)
        .into_iter()
        .map(|t| find_mirror(&t, defects) + 100 * find_mirror(&transpose(&t), defects))
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution(input.lines(), 0);
    println!("Part 1: {p1}");
    let p2 = solution(input.lines(), 1);
    println!("Part 2: {p2}");
}
