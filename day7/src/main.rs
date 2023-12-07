use std::cmp::Ordering;
use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    OnlyJokers,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn from_cards(cards: &[u8]) -> Self {
        // The number of occurences of each card number, ignoring jokers.
        let mut counts: Vec<_> = (0..5)
            .map(|i| cards.iter().filter(|&&x| x == cards[i] && x != 0).count())
            .collect();
        counts.sort();

        let mut hand_type = match counts[..] {
            [5, 5, 5, 5, 5] => Self::FiveOfAKind,
            [_, 4, 4, 4, 4] => Self::FourOfAKind,
            [2, 2, 3, 3, 3] => Self::FullHouse,
            [_, _, 3, 3, 3] => Self::ThreeOfAKind,
            [_, 2, 2, 2, 2] => Self::TwoPair,
            [_, _, _, 2, 2] => Self::OnePair,
            [_, _, _, _, 1] => Self::HighCard,
            [0, 0, 0, 0, 0] => Self::OnlyJokers,
            _ => panic!(),
        };

        let num_jokers = cards.iter().filter(|&&x| x == 0).count();
        for _ in 0..num_jokers {
            hand_type = hand_type.upgrade_with_joker();
        }

        hand_type
    }

    fn upgrade_with_joker(&self) -> Self {
        match self {
            Self::OnlyJokers => Self::HighCard,
            Self::HighCard => Self::OnePair,
            Self::OnePair => Self::ThreeOfAKind,
            Self::TwoPair => Self::FullHouse,
            Self::ThreeOfAKind => Self::FourOfAKind,
            Self::FourOfAKind => Self::FiveOfAKind,
            // Can't upgrade full house or five of a kind with a joker
            Self::FullHouse | Self::FiveOfAKind => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    hand_type: HandType,
    cards: [u8; 5],
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }
}

fn map_char(c: char, use_joker: bool) -> u8 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap().try_into().unwrap(),
        'T' => 10,
        'J' => {
            if use_joker {
                0
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!(),
    }
}

impl Hand {
    pub fn parse(line: &str, use_joker: bool) -> Self {
        let (cards_str, bid_str) = line.split_once(' ').unwrap();
        let cards: [u8; 5] = cards_str
            .chars()
            .map(|c| map_char(c, use_joker))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        let bid: u32 = bid_str.parse().unwrap();

        let hand_type = HandType::from_cards(&cards);

        Self {
            hand_type,
            cards,
            bid,
        }
    }
}

fn solution(lines: impl IntoIterator<Item = impl AsRef<str>>, use_joker: bool) -> u32 {
    let mut hands: Vec<Hand> = lines
        .into_iter()
        .map(|line| Hand::parse(line.as_ref(), use_joker))
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

    let p1 = solution(&lines, false);
    let p2 = solution(&lines, true);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
