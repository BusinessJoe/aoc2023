use regex::Regex;
use std::{
    cmp,
    io::{self, BufRead},
};

fn part1(lines: &[String]) -> u32 {
    let id_regex = Regex::new(r"Game (\d+):").unwrap();
    let cube_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut sum = 0;
    for line in lines {
        let id_str = id_regex.captures(line).unwrap().get(1).unwrap().as_str();
        let id: u32 = id_str.parse().unwrap();

        let valid_game = cube_regex.captures_iter(line).all(|cap| {
            let count_str = cap.get(1).unwrap().as_str();
            let count: u32 = count_str.parse().unwrap();
            let color = cap.get(2).unwrap().as_str();

            let limit = match color {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => panic!(),
            };

            count <= limit
        });

        if valid_game {
            sum += id;
        }
    }

    sum
}

fn part2(lines: &[String]) -> u32 {
    let cube_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut sum = 0;
    for line in lines {
        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;

        for cap in cube_regex.captures_iter(line) {
            let count_str = cap.get(1).unwrap().as_str();
            let count: u32 = count_str.parse().unwrap();
            let color = cap.get(2).unwrap().as_str();

            match color {
                "red" => max_r = cmp::max(max_r, count),
                "green" => max_g = cmp::max(max_g, count),
                "blue" => max_b = cmp::max(max_b, count),
                _ => panic!(),
            };
        }

        sum += max_r * max_g * max_b;
    }

    sum
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let sum1 = part1(&lines);
    let sum2 = part2(&lines);
    println!("Part 1: {sum1}");
    println!("Part 2: {sum2}");
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test_2() {
        let lines = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];

        let sum = part2(&lines);
        assert_eq!(2286, sum);
    }

    #[test]
    fn test_2_single() {
        let lines = vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()];

        let sum = part2(&lines);
        assert_eq!(48, sum);
    }
}
