use std::{
    collections::HashMap,
    io::{self, Read},
};

struct Lens {
    label: Vec<u8>,
    focus: u8,
}

fn hash(s: &[u8]) -> u8 {
    let mut h: u8 = 0;
    for c in s {
        h = h.wrapping_add(*c).wrapping_mul(17)
    }
    h
}

pub fn solution_1<'a>(input: &str) -> u32 {
    input
        .split(',')
        .map(|s| u32::from(hash(s.as_bytes())))
        .sum()
}

pub fn solution_2<'a>(input: &str) -> usize {
    let mut boxes: HashMap<u8, Vec<Lens>> = HashMap::new();

    for label in input.split(',') {
        match label.as_bytes() {
            [name @ .., b'-'] => {
                let h = hash(name);
                let vec = boxes.entry(h).or_default();
                if let Some(i) = vec.iter().position(|l| l.label == name) {
                    vec.remove(i);
                }
            }
            [name @ .., b'=', d] => {
                let h = hash(name);
                let digit = d - b'0';
                let vec = boxes.entry(h).or_default();
                let lens = Lens {
                    label: name.to_vec(),
                    focus: digit,
                };
                if let Some(i) = vec.iter().position(|l| l.label == name) {
                    vec[i] = lens;
                } else {
                    vec.push(lens);
                }
            }
            _ => unreachable!("{:?}", label),
        }
    }

    let mut p2 = 0;
    for (box_num, lenses) in boxes.iter() {
        for (lens_idx, lens) in lenses.iter().enumerate() {
            p2 += (usize::from(*box_num) + 1) * (lens_idx + 1) * usize::from(lens.focus);
        }
    }
    p2
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();
    input = input.replace('\n', "");

    let p1 = solution_1(&input);
    println!("Part 1: {p1}");
    let p2 = solution_2(&input);
    println!("Part 2: {p2}");
}
