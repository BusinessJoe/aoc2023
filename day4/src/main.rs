use std::io::{self, BufRead};

fn parse_ints(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solutions(lines: &[impl AsRef<str>]) -> (u32, u32) {
    let mut sum = 0;
    let mut copies: Vec<u32> = vec![1; lines.len()];

    lines.iter().enumerate().for_each(|(idx, line)| {
        let line: &str = line.as_ref();
        let no_header = line.split_once(": ").unwrap().1;
        let (winning, my_nums) = no_header.split_once(" | ").unwrap();
        let winning = parse_ints(winning);
        let my_nums = parse_ints(my_nums);

        let reps = copies[idx];
        match my_nums.into_iter().filter(|n| winning.contains(n)).count() {
            0 => {}
            n => {
                sum += 1 << (n - 1);
                for copy in copies.iter_mut().skip(idx + 1).take(n) {
                    *copy += reps;
                }
            }
        };
    });

    (sum, copies.into_iter().sum())
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let (p1, p2) = solutions(&lines);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use crate::solutions;

    #[test]
    fn test_ex_1() {
        let lines = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        assert_eq!(13, solutions(&lines).0);
    }

    #[test]
    fn test_ex_2() {
        let lines = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        assert_eq!(30, solutions(&lines).1);
    }
}
