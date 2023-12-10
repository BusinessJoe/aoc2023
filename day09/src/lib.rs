#[must_use]
pub fn gen_coeffs(n: usize) -> Vec<i64> {
    let mut coeffs = vec![0; n];
    coeffs[n - 1] = i64::try_from(n).unwrap();
    for i in (0..n - 1).rev() {
        let k: i64 = i64::try_from(n).unwrap() - 1 - i64::try_from(i).unwrap();
        // k goes from 1 ..= n-1
        coeffs[i] = -coeffs[i + 1] * (i64::try_from(n).unwrap() - k) / (k + 1);
    }
    coeffs
}

#[must_use]
pub fn parse_matrix<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<Vec<i64>> {
    lines
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn solution1<'a>(lines: impl IntoIterator<Item = &'a str>) -> i64 {
    let matrix = parse_matrix(lines);

    // Assume that each line has the same amount of numbers
    let coeffs = gen_coeffs(matrix[0].len());

    matrix
        .into_iter()
        .map(|nums| {
            nums.into_iter()
                .zip(&coeffs)
                .map(|(a, b)| a * b)
                .sum::<i64>()
        })
        .sum()
}

fn extrapolate_history(history: &[i32]) -> i32 {
    let diffs: Vec<i32> = history.windows(2).map(|win| win[1] - win[0]).collect();
    if diffs.iter().all(|&d| d == 0) {
        history[0]
    } else {
        history[history.len() - 1] + extrapolate_history(&diffs)
    }
}

pub fn solution1_fd<'a>(lines: impl IntoIterator<Item = &'a str>) -> i32 {
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
pub fn solution2<'a>(lines: impl IntoIterator<Item = &'a str>) -> i64 {
    let matrix = parse_matrix(lines);

    // Assume that each line has the same amount of numbers
    let coeffs = gen_coeffs(matrix[0].len());

    matrix
        .into_iter()
        .map(|nums| {
            nums.into_iter()
                .rev() // Reversed!
                .zip(&coeffs)
                .map(|(a, b)| a * b)
                .sum::<i64>()
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use crate::{solution1, solution2, solution1_fd};

    #[test]
    fn test_example_1() {
        let input = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
        let p1 = solution1(input);
        assert_eq!(114, p1);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../input.txt");
        let p1 = solution1(input.lines());
        assert_eq!(1637452029, p1);
    }

    #[test]
    fn test_part_1_fd() {
        let input = include_str!("../input.txt");
        let p1 = solution1_fd(input.lines());
        assert_eq!(1637452029, p1);
    }

    #[test]
    fn test_example_2() {
        let input = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
        let p2 = solution2(input);
        assert_eq!(2, p2);
    }
}
