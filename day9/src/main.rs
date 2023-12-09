use std::io::{self, Read};

fn extrapolate_history(history: &[i32]) -> i32 {
    let diffs: Vec<i32> = history.windows(2).map(|win| win[1] - win[0]).collect();
    if diffs.iter().all(|&d| d == 0) {
        history[0]
    } else {
        history[history.len() - 1] + extrapolate_history(&diffs)
    }
}

fn solution1<'a>(lines: impl IntoIterator<Item = &'a str>) -> i32 {
    lines
        .into_iter()
        .map(|line| {
            let history: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            extrapolate_history(&history)
        })
        .sum()
}

/// Literally just part 1 but I reverse the order of numbers in each line
fn solution2<'a>(lines: impl IntoIterator<Item = &'a str>) -> i32 {
    lines
        .into_iter()
        .map(|line| {
            let history: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .rev()
                .collect();
            extrapolate_history(&history)
        })
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution1(input.lines());
    let p2 = solution2(input.lines());
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
