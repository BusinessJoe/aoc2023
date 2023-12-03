use std::io::{self, BufRead};

fn solution1(lines: impl IntoIterator<Item = impl AsRef<str>>) -> u32 {
    let mut sum = 0;

    let mut allowed_coords: Vec<(i32, i32)> = Vec::new();

    let lines: Vec<String> = lines
        .into_iter()
        .map(|line| line.as_ref().to_string())
        .collect();

    for (r, line) in lines.iter().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char != '.' && !char.is_ascii_digit() {
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        let r: i32 = r.try_into().unwrap();
                        let c: i32 = c.try_into().unwrap();
                        allowed_coords.push((r + dr, c + dc));
                    }
                }
            }
        }
    }

    for (r, line) in lines.iter().enumerate() {
        let mut num_buffer = String::new();
        let mut valid = false;

        for (c, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                num_buffer.push(char);
                if allowed_coords.contains(&(r.try_into().unwrap(), c.try_into().unwrap())) {
                    valid = true;
                }
            } else {
                if !num_buffer.is_empty() && valid {
                    let num: u32 = num_buffer.parse().unwrap();
                    sum += num;
                }
                num_buffer.clear();
                valid = false;
            }
        }
        if !num_buffer.is_empty() && valid {
            let num: u32 = num_buffer.parse().unwrap();
            sum += num;
        }
    }

    sum
}

fn adjacencies(r: usize, c: usize, numbers: &[(usize, usize, usize, u32)]) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();

    for (row, start, end, num) in numbers {
        if row.abs_diff(r) <= 1 && start.saturating_sub(1) <= c && c <= *end {
            vec.push(*num);
        }
    }

    vec
}

fn solution2(lines: impl IntoIterator<Item = impl AsRef<str>>) -> u32 {
    let mut sum = 0;

    // row, start, end, value
    let mut numbers: Vec<(usize, usize, usize, u32)> = Vec::new();

    let lines: Vec<String> = lines
        .into_iter()
        .map(|line| line.as_ref().to_string())
        .collect();

    for (r, line) in lines.iter().enumerate() {
        let mut num_buffer = String::new();
        let mut start: usize = 0;

        for (c, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                if num_buffer.is_empty() {
                    start = c;
                }
                num_buffer.push(char);
            } else {
                if !num_buffer.is_empty() {
                    let num: u32 = num_buffer.parse().unwrap();
                    numbers.push((r, start, c, num));
                }
                num_buffer.clear();
            }
        }
        if !num_buffer.is_empty() {
            let num: u32 = num_buffer.parse().unwrap();
            numbers.push((r, start, line.len(), num));
        }
    }

    for (r, line) in lines.iter().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == '*' {
                let adj = adjacencies(r, c, &numbers);
                if adj.len() == 2 {
                    sum += adj[0] * adj[1];
                }
            }
        }
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let p1 = solution1(&lines);
    let p2 = solution2(&lines);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {}
