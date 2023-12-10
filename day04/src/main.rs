use std::io::{self, BufRead};
use day4::solutions;

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let (p1, p2) = solutions(&lines);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

