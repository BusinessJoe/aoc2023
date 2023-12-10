use day9::{solution1, solution2};

use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution1(input.lines());
    let p2 = solution2(input.lines());
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
