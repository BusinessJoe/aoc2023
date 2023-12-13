use std::{
    collections::HashMap,
    io::{self, Read},
};

#[derive(Debug, PartialEq, Eq)]
enum Symbol {
    Ok,
    Damaged,
    Unknown,
}

fn parse_line(line: &str) -> (Vec<Symbol>, Vec<usize>) {
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
    let nums: Vec<usize> = nums.split(',').map(|s| s.parse().unwrap()).collect();

    (symbols, nums)
}

fn parse_line_2(line: &str) -> (Vec<Symbol>, Vec<usize>) {
    let (symbols, nums) = line.split_once(' ').unwrap();
    let symbols = [(); 5].map(|()| symbols).join("?");
    let nums = [(); 5].map(|()| nums).join(",");
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
    let nums: Vec<usize> = nums.split(',').map(|s| s.parse().unwrap()).collect();

    (symbols, nums)
}

fn next_group(symbols: &[Symbol]) -> (usize, usize) {
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

type Cache = HashMap<(usize, usize), usize>;
fn count_possibilities_cached(
    symbols: &[Symbol],
    s_idx: usize,
    nums: &[usize],
    n_idx: usize,
    cache: &mut Cache,
) -> usize {
    if n_idx == nums.len() && s_idx == symbols.len() {
        return 1;
    } else if s_idx == symbols.len() {
        return 0;
    } else if n_idx == nums.len() {
        // We've exhausted nums, so we fail to solve the puzzle if there are any
        // damaged symbols left.
        if symbols[s_idx..].contains(&Symbol::Damaged) {
            return 0;
        }
        return 1;
    }

    if let Some(ans) = cache.get(&(s_idx, n_idx)) {
        return *ans;
    }

    let target = nums[n_idx];
    let (num_ok, num_not_ok) = next_group(&symbols[s_idx..]);

    let ans = if num_not_ok < target {
        // The current group cannot satisfy `target`. We therefore require that the "not ok" part
        // is all unknowns so that they can all be set to Ok. Otherwise the puzzle is insoluble.
        if symbols[s_idx + num_ok..s_idx + num_ok + num_not_ok].contains(&Symbol::Damaged) {
            0
        } else {
            count_possibilities_cached(symbols, s_idx + num_ok + num_not_ok, nums, n_idx, cache)
        }
    } else {
        // Both Damaged and Unknown count the damaged case.
        // Take next `target` symbols as damaged.
        let if_dmged = match symbols.get(s_idx + num_ok + target) {
            // If the symbol right after the group is damaged, we're forced to have `target+1`
            // damaged symbols in a row.
            Some(Symbol::Damaged) => 0,
            // If the symbol right after the group is unknown, we're forced to make it Ok due to
            // the above comment. We consume `target+1` non-ok symbols.
            Some(Symbol::Unknown) => count_possibilities_cached(
                symbols,
                s_idx + num_ok + target + 1,
                nums,
                n_idx + 1,
                cache,
            ),
            // If the symbol right after the group doesn't exist or is Ok, we consume `target`
            // non-ok symbols to satisfy target.
            None | Some(Symbol::Ok) => {
                count_possibilities_cached(symbols, s_idx + num_ok + target, nums, n_idx + 1, cache)
            }
        };

        if symbols[s_idx + num_ok] == Symbol::Unknown {
            // Unknown should consider the possiblity of being ok
            let if_ok = count_possibilities_cached(symbols, s_idx + num_ok + 1, nums, n_idx, cache);

            if_ok + if_dmged
        } else {
            if_dmged
        }
    };

    cache.insert((s_idx, n_idx), ans);
    ans
}

fn count_possibilities(symbols: &[Symbol], nums: &[usize]) -> usize {
    let mut cache: Cache = Cache::new();
    count_possibilities_cached(symbols, 0, nums, 0, &mut cache)
}

#[must_use]
pub fn solution_1<'a>(lines: impl IntoIterator<Item = &'a str>) -> usize {
    lines
        .into_iter()
        .map(parse_line)
        .map(|(symbols, nums)| count_possibilities(&symbols, &nums))
        .sum()
}

#[must_use]
pub fn solution_2<'a>(lines: impl IntoIterator<Item = &'a str>) -> usize {
    lines
        .into_iter()
        .map(parse_line_2)
        .map(|(symbols, nums)| count_possibilities(&symbols, &nums))
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution_1(input.lines());
    println!("Part 1: {p1}");
    let p2 = solution_2(input.lines());
    println!("Part 2: {p2}");
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
        let x = count_possibilities(
            &[Symbol::Unknown, Symbol::Unknown, Symbol::Unknown],
            &[1, 1],
        );
        assert_eq!(1, x);
    }

    #[test]
    fn test_p1() {
        let p1 = solution_1(include_str!("../input.txt").lines());
        assert_eq!(7694, p1);
    }

    #[test]
    fn test_p2() {
        let p2 = solution_2(include_str!("../input.txt").lines());
        assert_eq!(5071883216318, p2);
    }
}
