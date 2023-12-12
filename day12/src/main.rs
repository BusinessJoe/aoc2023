use std::io::{self, Read};

#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    Ok,
    Damaged,
    Unknown,
}

pub fn parse_line(line: &str) -> (Vec<Symbol>, Vec<usize>) {
    let (symbols, nums) = line.split_once(' ').unwrap();
    let symbols: Vec<Symbol> = symbols
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'.' => Symbol::Ok,
            b'#' => Symbol::Damaged,
            b'?' => Symbol::Unknown,
            _ => panic!(),
        })
        .collect();
    let nums: Vec<usize> = nums
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    (symbols, nums)
}

pub fn next_group(symbols: &[Symbol]) -> (usize, usize) {
    let mut i = 0;
    let mut num_ok = 0;
    while i < symbols.len() && symbols[i] == Symbol::Ok {
        i += 1;
        num_ok += 1;
    }

    let mut num_not_ok = 0;
    // We're either at unknown or damaged now
    while i < symbols.len() && symbols[i] != Symbol::Ok {
        i += 1;
        num_not_ok += 1;
    }

    (num_ok, num_not_ok)
}

pub fn count_possibilities(symbols: &[Symbol], nums: &[usize]) -> usize {
    //dbg!(symbols, nums);
    if nums.len() == 0 {
        if symbols.contains(&Symbol::Damaged) {
            return 0;
        } else {
            return 1;
        }
    } else if symbols.len() == 0 {
        return 0;
    }

    let (num_ok, num_not_ok) = next_group(symbols);
    //dbg!(num_ok, num_not_ok);
    if num_not_ok == 0 {
        return 0;
    }
    let target = nums[0];

    if num_not_ok < target {
        if symbols[num_ok] == Symbol::Damaged {
            // gg
            0
        } else {
            count_possibilities(&symbols[num_ok+1..], nums)
        }
    } else {
        if symbols[num_ok] == Symbol::Damaged {
            // forced to take next `target` symbols as damaged
            match symbols.get(num_ok+target) {
                Some(Symbol::Damaged) => 0,
                None | Some(Symbol::Ok) => count_possibilities(&symbols[num_ok + target..], &nums[1..]),
                Some(Symbol::Unknown) => count_possibilities(&symbols[num_ok + target + 1..], &nums[1..]),
            }
        } else {
            //println!("take ok path from {:?} {:?}", symbols, nums);
            // we have options bc this symbol is unknown
            let if_ok = count_possibilities(&symbols[num_ok+1..], nums);
            //println!("ok path from {:?} {:?} has {}", symbols, nums, if_ok);

            //println!("take dmg path from {:?} {:?}", symbols, nums);
            // take next `target` symbols as damaged
            let if_dmged = match symbols.get(num_ok+target) {
                Some(Symbol::Damaged) => 0,
                None | Some(Symbol::Ok) => count_possibilities(&symbols[num_ok + target..], &nums[1..]),
                Some(Symbol::Unknown) => count_possibilities(&symbols[num_ok + target + 1..], &nums[1..]),
            };
            //println!("dmg path from {:?} {:?} has {}", symbols, nums, if_dmged);

            if_ok + if_dmged
        }
    }
}

pub fn solution<'a>(lines: impl IntoIterator<Item = &'a str>) -> usize {
    let mut total = 0;
    for line in lines {
        let (symbols, nums) = parse_line(line);
        let x = count_possibilities(&symbols, &nums);
        dbg!(line);
        dbg!(x);
        total += x;
    }
    total
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution(input.lines());
    println!("Part 1: {p1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_unknown() {
        let x = count_possibilities(&[Symbol::Unknown], &[1]);
        assert_eq!(1, x);
    }

    #[test]
    fn test_double_unknown() {
        let x = count_possibilities(&[Symbol::Unknown, Symbol::Unknown], &[1]);
        assert_eq!(2, x);
    }

    #[test]
    fn test_triple_unknown() {
        let x = count_possibilities(&[Symbol::Unknown, Symbol::Unknown, Symbol::Unknown], &[1, 1]);
        assert_eq!(1, x);
    }

    #[test]
    fn test_a() {
        let (symbols, nums) = parse_line("???##???..?. 5,1");
        let x = count_possibilities(&symbols, &nums);
        assert_eq!(7, x);
    }

    #[test]
    fn test_b() {
        let (symbols, nums) = parse_line("###???.????????.???? 5,2,1,1,1,1");
        let x = count_possibilities(&symbols, &nums);
        assert_eq!(34, x);
    }

    #[test]
    fn test_c() {
        let (symbols, nums) = parse_line("..#... 1");
        let x = count_possibilities(&symbols, &nums);
        assert_eq!(1, x);
    }
}
