use day5::{solution1, solution2};
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let p1 = solution1(&lines);
    let p2 = solution2(&lines);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

