use std::io::{self, BufRead};
use std::result::Result;

fn parse_line(s: impl AsRef<str>) -> u32 {
    let first_char = s.as_ref().chars().find(char::is_ascii_digit).unwrap();
    let last_char = s.as_ref().chars().rev().find(char::is_ascii_digit).unwrap();

    first_char.to_digit(10).unwrap() * 10 + last_char.to_digit(10).unwrap()
}

fn parse_num(num: &str) -> u32 {
    match num {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!(),
    }
}

const TOKENS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn find_first_match(s: &str) -> &str {
    for i in 0..s.len() {
        for token in &TOKENS {
            if s[i..].starts_with(token) {
                return token;
            }
        }
    }
    panic!()
}

fn find_last_match(s: &str) -> &str {
    for i in (0..s.len()).rev() {
        for token in &TOKENS {
            if s[i..].starts_with(token) {
                return token;
            }
        }
    }
    panic!()
}

fn parse_wordy_line(s: impl AsRef<str>) -> u32 {
    let first_match = find_first_match(s.as_ref());
    let last_match = find_last_match(s.as_ref());
    parse_num(first_match) * 10 + parse_num(last_match)
}

fn find_calibration_sum(lines: &[String]) -> u32 {
    lines.iter().map(parse_line).sum()
}

fn find_wordy_calibration_sum(lines: &[String]) -> u32 {
    lines.iter().map(parse_wordy_line).sum()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let sum = find_calibration_sum(&lines);
    let wordy_sum = find_wordy_calibration_sum(&lines);
    println!("Sum: {sum}");
    println!("Wordy Sum: {wordy_sum}");
}

#[cfg(test)]
mod tests {
    use crate::{find_calibration_sum, find_wordy_calibration_sum, parse_wordy_line};

    #[test]
    fn test_example() {
        let lines: Vec<String> = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];

        let sum = find_calibration_sum(&lines);
        assert_eq!(142, sum);
    }

    #[test]
    fn test_wordy() {
        let lines: Vec<String> = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        let sum = find_wordy_calibration_sum(&lines);
        assert_eq!(281, sum);
    }

    #[test]
    fn test_simple() {
        let val = parse_wordy_line("two1");
        assert_eq!(21, val);
    }

    #[test]
    fn test_zero() {
        let val = parse_wordy_line("120");
        assert_eq!(12, val);
    }

    #[test]
    fn test_one_num() {
        let val = parse_wordy_line("seven");
        assert_eq!(77, val);
    }

    #[test]
    fn test_surrounding_garbage() {
        let val = parse_wordy_line("zzzsevenxx13xxnineyy");
        assert_eq!(79, val);
    }

    #[test]
    fn test_doubles() {
        let cases = [
            ("oneone", 11),
            ("twotwo", 22),
            ("threethree", 33),
            ("fourfour", 44),
            ("fivefive", 55),
            ("sixsix", 66),
            ("sevenseven", 77),
            ("eighteight", 88),
            ("ninenine", 99),
        ];
        for case in cases.iter() {
            assert_eq!(case.1, parse_wordy_line(case.0));
        }
    }

    #[test]
    fn test_overlap() {
        assert_eq!(18, parse_wordy_line("oneight"));
    }
}
