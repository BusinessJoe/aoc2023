fn parse_ints(line: &[u8]) -> u128 {
    line.chunks_exact(3).map(|digits| {
        10 * (digits[1] & 0xf) + (digits[2] & 0xf)
    }).fold(0, |bit_set, n| bit_set | (1 << n))
}

pub fn solutions(lines: &[impl AsRef<str>]) -> (u32, u32) {
    let mut sum = 0;
    let mut copies: Vec<u32> = vec![1; lines.len()];

    // These are the same across the whole file
    let (colon_idx, pipe_idx) = {
        let line = lines[0].as_ref().as_bytes();
        let colon_idx = line.iter().position(|&c| c == b':').unwrap();
        let pipe_idx = line.iter().position(|&c| c == b'|').unwrap();

        (colon_idx, pipe_idx)
    };

    lines.iter().map(|line| {
        let line = line.as_ref().as_bytes();
        let winning = &line[colon_idx + 1..pipe_idx];
        let my_nums = &line[pipe_idx + 1..];

        (parse_ints(winning) & parse_ints(my_nums)).count_ones()
    }).enumerate().for_each(|(idx, count)| {
        let reps = copies[idx];
        match count {
            0 => {}
            n => {
                sum += 1 << (n - 1);
                for copy in copies.iter_mut().skip(idx + 1).take(n.try_into().unwrap()) {
                    *copy += reps;
                }
            }
        };
    });

    (sum, copies.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

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
