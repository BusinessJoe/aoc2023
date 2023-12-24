use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

#[derive(Debug)]
struct Hailstone {
    pos: (f64, f64, f64),
    vel: (f64, f64, f64),
}

const EPSILON: f64 = 0.000000001;

impl Hailstone {
    fn parse(s: &str) -> Self {
        let (pos_str, vel_str) = s.split_once(" @ ").unwrap();
        let pos: Vec<_> = pos_str.split(',').map(|s| s.trim().parse().unwrap()).collect();
        let vel: Vec<_> = vel_str.split(',').map(|s| s.trim().parse().unwrap()).collect();
        Self {
            pos: (pos[0], pos[1], pos[2]),
            vel: (vel[0], vel[1], vel[2]),
        }
    }

    fn xy_intersection(&self, o: &Self) -> Option<(f64, f64)> {
        // (vel.y / vel.x) * x - (vel.y / vel.x) * pos.x + pos.y = (o.vel.y / o.vel.x) * x - (o.vel.y / o.vel.x) * o.pos.x + o.pos.y
        // (vel.y / vel.x) * x - (o.vel.y / o.vel.x) * x = (vel.y / vel.x) * pos.x - (o.vel.y / o.vel.x) * o.pos.x + o.pos.y - pos.y
        // [(vel.y / vel.x) - (o.vel.y / o.vel.x)] * x = (vel.y / vel.x) * pos.x - (o.vel.y / o.vel.x) * o.pos.x + o.pos.y - pos.y
        // [(vel.y / vel.x) - (o.vel.y / o.vel.x)] * x = (vel.y * pos.x * o.vel.x / [vel.x * o.vel.x]) - (o.vel.y * o.pos.x * vel.x / [vel.x * o.vel.x]) + o.pos.y - pos.y
        // [(vel.y / vel.x) - (o.vel.y / o.vel.x)] * x = (vel.y * pos.x * o.vel.x - o.vel.y * o.pos.x * vel.x / [vel.x * o.vel.x]) + o.pos.y - pos.y
        // [(vel.y * o.vel.x - o.vel.y * vel.x / [vel.x * o.vel.x]) * x = (vel.y * pos.x * o.vel.x - o.vel.y * o.pos.x * vel.x / [vel.x * o.vel.x]) + o.pos.y - pos.y
        //

        assert_ne!(0., self.vel.0);
        assert_ne!(0., o.vel.0);

        if (self.vel.1 / self.vel.0 - o.vel.1 / o.vel.0).abs() < EPSILON {
            return None
        }

        assert_ne!(0., self.vel.1 / self.vel.0 - o.vel.1 / o.vel.0, "{} {}, {} {}", self.vel.0, self.vel.1, o.vel.0, o.vel.1);
        let x = ((self.vel.1 / self.vel.0) * self.pos.0 - (o.vel.1 / o.vel.0) * o.pos.0 + o.pos.1 - self.pos.1) / (self.vel.1 / self.vel.0 - o.vel.1 / o.vel.0);

        // y = (vel.y / vel.x) * (x - pos.x) + pos.y 

        let y = (self.vel.1 / self.vel.0) * (x - self.pos.0) + self.pos.1;

        // find if aligned with future by taking dot product of velocity and diff between current
        // pos and (x, y).
        // dot product will be positive iff (x, y) is in future.
        let is_self_future = self.vel.0 * (x - self.pos.0) + self.vel.1 * (y - self.pos.1) > 0.;
        let is_other_future = o.vel.0 * (x - o.pos.0) + o.vel.1 * (y - o.pos.1) > 0.;

        if is_self_future && is_other_future {
            Some((x, y))
        } else {
            None
        }
    }
}

const LOWER: f64 = 200000000000000.;
const UPPER: f64 = 400000000000000.;
//const LOWER: f64 = 7.;
//const UPPER: f64 = 27.;

pub fn solution_1(input: &str) -> usize {
    let stones: Vec<Hailstone> = input.lines().map(Hailstone::parse).collect();

    let mut p1 = 0;

    for (i, stone1) in stones.iter().enumerate().take(stones.len() - 1) {
        for stone2 in &stones[i + 1..] {
            if let Some((x, y)) = stone1.xy_intersection(&stone2) {
                if LOWER <= x && x <= UPPER && LOWER <= y && y <= UPPER {
                    //println!("{:?} -|- {:?} @ ({}, {})", stone1, stone2, x, y);
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
