use std::{
    cmp,
    collections::{HashMap, HashSet},
    io::{self, Read},
};

/// (X, Y, Z) triple
type Coord = (usize, usize, usize);
type Id = usize;

#[derive(Debug)]
struct Brick {
    id: Id,
    lower: Coord,
    upper: Coord,
}

impl Brick {
    /// Returns z coordinate of lowest cube
    fn bottom(&self) -> usize {
        cmp::min(self.lower.2, self.upper.2)
    }

    fn coords(&self) -> Box<dyn Iterator<Item = Coord> + '_> {
        let dx = self.upper.0 - self.lower.0;
        let dy = self.upper.1 - self.lower.1;
        let dz = self.upper.2 - self.lower.2;

        match (dx, dy, dz) {
            (_, 0, 0) => {
                Box::new((self.lower.0..=self.upper.0).map(|x| (x, self.lower.1, self.lower.2)))
            }
            (0, _, 0) => {
                Box::new((self.lower.1..=self.upper.1).map(|y| (self.lower.0, y, self.lower.2)))
            }
            (0, 0, _) => {
                Box::new((self.lower.2..=self.upper.2).map(|z| (self.lower.0, self.lower.1, z)))
            }
            _ => panic!("{} {} {}", dx, dy, dz),
        }
    }

    fn parse_coord(s: &str) -> Coord {
        let coord: Vec<usize> = s.split(',').map(|s| s.parse().unwrap()).collect();
        (coord[0], coord[1], coord[2])
    }

    fn parse(id: Id, s: &str) -> Self {
        let (left, right) = s.split_once('~').unwrap();

        let a = Self::parse_coord(left);
        let b = Self::parse_coord(right);

        if a < b {
            Self {
                id,
                lower: a,
                upper: b,
            }
        } else {
            Self {
                id,
                lower: b,
                upper: a,
            }
        }
    }
}

pub fn solution(input: &str) -> (usize, usize) {
    let mut bricks: Vec<Brick> = input
        .lines()
        .enumerate()
        .map(|(id, s)| Brick::parse(id, s))
        .collect();
    bricks.sort_by_key(|b| b.bottom());

    // Maps an (x, y) pair to its height
    let mut heights: HashMap<(usize, usize), (Option<Id>, usize)> = HashMap::new();
    let mut below: HashMap<Id, HashSet<Id>> = HashMap::new();
    let mut above: HashMap<Id, HashSet<Id>> = HashMap::new();

    for b in &mut bricks {
        let max_height = b
            .coords()
            .map(|(x, y, _)| heights.entry((x, y)).or_insert((None, 0)).1)
            .max()
            .unwrap();

        for (x, y, _) in b.coords() {
            if let (Some(id), height) = heights.entry((x, y)).or_insert((None, 0)) {
                if *height == max_height {
                    below
                        .entry(b.id)
                        .and_modify(|v| {
                            v.insert(*id);
                        })
                        .or_insert(HashSet::from([*id]));
                    above
                        .entry(*id)
                        .and_modify(|v| {
                            v.insert(b.id);
                        })
                        .or_insert(HashSet::from([b.id]));
                }
            }
        }

        let old_lower_z = b.lower.2;
        b.lower.2 = max_height + 1;
        let diff = old_lower_z - b.lower.2;
        b.upper.2 -= diff;

        for (x, y, z) in b.coords() {
            heights.insert((x, y), (Some(b.id), z));
        }

        for (x, y, z) in b.coords() {
            heights.insert((x, y), (Some(b.id), z));
        }
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for b in &bricks {
        let mut removed = HashSet::from([b.id]);

        loop {
            let mut changed = false;

            for a in &bricks {
                if removed.contains(&a.id) {
                    continue;
                }
                let can_remove = below.get(&a.id).map_or(false, |brick_below| {
                    !brick_below.is_empty() && brick_below.is_subset(&removed)
                });

                if can_remove {
                    changed = true;
                    removed.insert(a.id);
                }
            }

            if !changed {
                break;
            }
        }

        // brick can be disintigrated if all bricks above it has at least a second brick below it
        let can_be_disintigrated = removed.len() == 1;

        if can_be_disintigrated {
            p1 += 1;
        }
        p2 += removed.len() - 1;
    }

    (p1, p2)
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let (p1, p2) = solution(&input);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brick_iter() {
        let b = Brick {
            id: 0,
            lower: (1, 0, 1),
            upper: (1, 2, 1),
        };
        let mut iter = b.coords();
        assert_eq!(Some((1, 0, 1)), iter.next());
        assert_eq!(Some((1, 1, 1)), iter.next());
        assert_eq!(Some((1, 2, 1)), iter.next());
    }
}
