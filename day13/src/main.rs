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

/// Reflect a row across the given column and return the number of mismatches.
fn count_row_mismatches(row: &[u8], col: usize) -> usize {
    row[0..col]
        .iter()
        .rev()
        .zip(row[col..].iter())
        .filter(|(a, b)| a != b)
        .count()
}

/// Reflect terrain across the given column and return the number of mismatches.
fn count_terrain_mismatches(terrain: &Terrain, col: usize) -> usize {
    terrain
        .iter()
        .map(|line| count_row_mismatches(line, col))
        .sum()
}

fn find_mirror(terrain: &Terrain, defects: usize) -> usize {
    let cols = terrain[0].len();
    (1..cols)
        .find(|&col| count_terrain_mismatches(terrain, col) == defects)
        .unwrap_or(0)
}

pub fn solution<'a>(lines: impl IntoIterator<Item = &'a str>, defects: usize) -> usize {
    parse_terrain(lines)
        .iter()
        .map(|t| find_mirror(t, defects) + 100 * find_mirror(&transpose(t), defects))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let p1 = solution(include_str!("../input.txt").lines(), 0);
        assert_eq!(33047, p1);
    }

    #[test]
    fn test_part_2() {
        let p2 = solution(include_str!("../input.txt").lines(), 1);
        assert_eq!(28806, p2);
    }
}
