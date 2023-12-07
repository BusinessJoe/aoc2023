use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    High,
    Wild,
}

impl HandType {
    pub fn new(vals: &[u8]) -> Self {
        let mut counts_with_vals: Vec<_> = (0..5).map(|i| 
            (
                vals.iter().filter(|&&x| x == vals[i] && x != 0).count(),
                vals[i],
            )
        ).collect();
        counts_with_vals.sort();

        let mut hand_type = match counts_with_vals[..] {
            [(5, _), ..] => Self::Five,
            [_, (4, _), ..] => Self::Four,
            [(2, _), _, (3, _), _, _] => Self::FullHouse,
            [_, _, (3, _), ..] => Self::Three,
            [_, (2, _), _, (2, _), _] => Self::TwoPair,
            [.., (2, _), _] => Self::OnePair,
            [.., (0, _)] => Self::Wild,
            [..] => Self::High,
        };

        let num_jokers = vals.iter().filter(|&&x| x == 0).count();
        for _ in 0..num_jokers {
            hand_type = hand_type.upgrade();
        }

        hand_type
    }

    fn upgrade(&self) -> Self {
        match self {
            Self::Wild => Self::High,
            Self::High => Self::OnePair,
            Self::OnePair => Self::Three,
            Self::TwoPair => Self::FullHouse,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::FullHouse | Self::Five => panic!(),
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Wild => panic!(),
            Self::High => 0,
            Self::OnePair => 1,
            Self::TwoPair => 2,
            Self::Three => 3,
            Self::FullHouse => 4,
            Self::Four => 5,
            Self::Five => 6,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    hand_type: HandType,
    vals: [u8; 5],
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.value().cmp(&other.hand_type.value()) {
            std::cmp::Ordering::Equal => self.vals.cmp(&other.vals),
            x => x,
        }
    }
}

fn map_char(c: char) -> u8 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap().try_into().unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("{}", c),
    }
}

fn map_char_joker(c: char) -> u8 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap().try_into().unwrap(),
        'T' => 10,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("{}", c),
    }
}

impl Hand {
    pub fn parse(line: &str, jokers: bool) -> Self {
        let (vals_str, bid_str) = line.split_once(' ').unwrap();
        let vals: [u8; 5] = vals_str
            .chars()
            .map(|c| {
                if jokers {
                    map_char_joker(c)
                } else {
                    map_char(c)
                }
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        let bid: u32 = bid_str.parse().unwrap();

        let hand_type = HandType::new(&vals);

        Self {
            hand_type,
            vals,
            bid,
        }
    }
}

fn solution1(lines: impl IntoIterator<Item = impl AsRef<str>>) -> u32 {
    let mut hands: Vec<Hand> = lines
        .into_iter()
        .map(|line| Hand::parse(line.as_ref(), false))
        .collect();
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * u32::try_from(i + 1).unwrap())
        .sum()
}

fn solution2(lines: impl IntoIterator<Item = impl AsRef<str>>) -> u32 {
    let mut hands: Vec<Hand> = lines
        .into_iter()
        .map(|line| Hand::parse(line.as_ref(), true))
        .collect();
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * u32::try_from(i + 1).unwrap())
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let p1 = solution1(&lines);
    let p2 = solution2(lines);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
