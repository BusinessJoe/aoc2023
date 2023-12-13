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

fn find_mirror(line: &[u8]) -> u64 {
    let mut bitset = 0;
    for i in 1..line.len() {
        if line[0..i]
            .iter()
            .rev()
            .zip(line[i..].iter())
            .all(|(a, b)| a == b)
        {
            bitset |= 1 << i;
        }
    }
    bitset
}

fn find_column_mirror(terrain: &Terrain) -> usize {
    let bitset = terrain
        .iter()
        .map(|line| find_mirror(line))
        .reduce(|acc, x| acc & x)
        .unwrap();
    if bitset == 0 {
        0
    } else {
        let cols = terrain[0].len();
        for i in 0..=cols / 2 {
            if bitset & (1 << cols / 2 + i) != 0 {
                return cols / 2 + i;
            }
            if bitset & (1 << cols / 2 - 1 - i) != 0 {
                return cols / 2 - 1 - i;
            }
        }
        panic!();
    }
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

pub fn solution_1<'a>(lines: impl IntoIterator<Item = &'a str>) -> usize {
    let all_terrain = parse_terrain(lines);

    let mut total = 0;
    for terrain in all_terrain {
        let col = find_column_mirror(&terrain);
        println!("col {col}");
        total += col;

        let row = find_column_mirror(&transpose(&terrain));
        println!("row {row}");
        total += row * 100;

        println!("");
        if row != 0 && col != 0 {
            panic!();
        }
    }
    total
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution_1(input.lines());
    println!("Part 1: {p1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_mirror() {
        let bitset = find_mirror("######....#######".as_bytes());
        let bitset2 = find_mirror("..#...####...#..#".as_bytes());
        assert_ne!(0, bitset & bitset2);
    }

    #[test]
    fn test_column_mirror() {
        let terrain = &parse_terrain(["######....#######", "..#...####...#..#"])[0];
        let mirror = find_column_mirror(terrain);
        assert_ne!(0, mirror);
    }
}
