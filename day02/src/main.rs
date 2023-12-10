use std::{
    cmp,
    io::{self, BufRead},
};

/// Returns the line's ID and an iterator over (count, color) pairs.
fn parse_line(line: &str) -> (u32, impl Iterator<Item = (u32, &str)>) {
    let (id_part, cube_part): (&str, &str) = line.split_once(": ").unwrap();
    let id: u32 = id_part.split_whitespace().last().unwrap().parse().unwrap();

    let count_col_iter = cube_part
        .split("; ")
        .flat_map(|round| round.split(", "))
        .map(|cube_count| {
            let (count_str, color) = cube_count.split_once(' ').unwrap();
            let count: u32 = count_str.parse().unwrap();
            (count, color)
        });

    (id, count_col_iter)
}

/// Return solutions to part 1 and part 2.
fn solution(lines: impl IntoIterator<Item = impl AsRef<str>>) -> (u32, u32) {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in lines {
        let (id, count_col_iter) = parse_line(line.as_ref());

        let (mut r, mut g, mut b) = (0, 0, 0);
        for (count, color) in count_col_iter {
            match color {
                "red" => r = cmp::max(r, count),
                "green" => g = cmp::max(g, count),
                "blue" => b = cmp::max(b, count),
                _ => panic!(),
            }
        }

        if r <= 12 && g <= 13 && b <= 14 {
            sum1 += id;
        }
        sum2 += r * g * b;
    }

    (sum1, sum2)
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let (sum1, sum2) = solution(lines);
    println!("Part 1: {sum1}");
    println!("Part 2: {sum2}");
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn test_2() {
        let lines = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let (_, sum) = solution(&lines);
        assert_eq!(2286, sum);
    }

    #[test]
    fn test_2_single() {
        let lines = vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"];

        let (_, sum) = solution(&lines);
        assert_eq!(48, sum);
    }
}
