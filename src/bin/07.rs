use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(7);

#[derive(Debug)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn to_int(&self) -> u8 {
        match &self {
            HandType::HighCard => 1,
            HandType::OnePair => 2,
            HandType::TwoPair => 3,
            HandType::ThreeKind => 4,
            HandType::FullHouse => 5,
            HandType::FourKind => 6,
            HandType::FiveKind => 7,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    hand_type: HandType,
    bet: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type.to_int() != other.hand_type.to_int() {
            return self.hand_type.to_int().cmp(&other.hand_type.to_int());
        }
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].cmp(&other.cards[i]);
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type.to_int() != other.hand_type.to_int() {
            return self
                .hand_type
                .to_int()
                .partial_cmp(&other.hand_type.to_int());
        }
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].partial_cmp(&other.cards[i]);
            }
        }
        None
    }
}

impl FromStr for Hand {
    type Err = &'static str; // Define the associated error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Attempt to parse the string and create an instance of MyStruct
        let (card_str, bet_str) = s.split_once(' ').unwrap();
        let bet = bet_str.parse::<u32>().unwrap();

        let cards: [u8; 5] = card_str
            .chars()
            .map(|c| match c {
                c if c.is_numeric() => c.to_digit(10).unwrap() as u8,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!(),
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        let hand_type = classify_hand(&cards);
        Ok(Hand {
            cards,
            hand_type,
            bet,
        })
    }
}

impl Hand {
    fn wildcard(mut self) -> Self {
        // first, replace in each array the 11s with 0s and count jokers

        let mut n_jokers = 0;
        for card in self.cards.iter_mut() {
            if *card == 11 {
                *card = 0;
                n_jokers += 1;
            }
        }

        // Figure out the most common card that isn't a wildcard

        let counter: HashMap<u8, usize> = create_counter(&self.cards);

        let (max_card, _) = counter
            .iter()
            .filter(|&(k, _)| *k != 0)
            .max_by_key(|&(_, v)| v)
            .unwrap_or((&50, &50));

        // everything was a wildcard so return
        if max_card == &50 {
            return self;
        }

        let mut new_hand = self.cards;
        for _ in 0..n_jokers {
            for card in new_hand.iter_mut() {
                if *card == 0 {
                    *card = *max_card;
                }
            }
        }

        self.hand_type = classify_hand(&new_hand);

        self
    }
}

fn create_counter(cards: &[u8]) -> HashMap<u8, usize> {
    let mut counter: HashMap<u8, usize> = HashMap::new();
    for &card in cards {
        let count = counter.entry(card).or_insert(0);
        *count += 1;
    }
    counter
}

fn classify_hand(cards: &[u8]) -> HandType {
    let counter = create_counter(cards);
    let mut values = counter.values().cloned().collect::<Vec<usize>>();
    values.sort_by(|a, b| b.cmp(a));

    match values[0] {
        5 => HandType::FiveKind,
        4 => HandType::FourKind,
        3 => {
            if values[1] == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeKind
            }
        }
        2 => {
            if values[1] == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        1 => HandType::HighCard,
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bets: Vec<Hand> = input.lines().map(|s| s.parse::<Hand>().unwrap()).collect();
    bets.sort();
    Some(
        bets.iter()
            .enumerate()
            .map(|(i, hand)| (i as u32 + 1) * hand.bet)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bets: Vec<Hand> = input
        .lines()
        .map(|s| s.parse::<Hand>().unwrap().wildcard())
        .collect();

    bets.sort();
    Some(
        bets.iter()
            .enumerate()
            .map(|(i, hand)| (i as u32 + 1) * hand.bet)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
