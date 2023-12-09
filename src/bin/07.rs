use std::{cmp::Ordering, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum PlayingCard {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum PlayingCardJoker {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for PlayingCard {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            _ => Self::Two,
        }
    }
}

impl From<char> for PlayingCardJoker {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            _ => Self::Two,
        }
    }
}

struct PlayingCards([PlayingCard; 5]);
struct PlayingCardsJoker([PlayingCardJoker; 5]);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum PlayingCardsType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PlayingCards {
    fn is_five_of_a_kind(&self) -> bool {
        self.0
            .iter()
            .counts()
            .values()
            .any(|&count| count == 5_usize)
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.0
            .iter()
            .counts()
            .values()
            .any(|&count| count == 4_usize)
    }

    fn is_full_house(&self) -> bool {
        let counts = self.0.iter().counts();
        counts.values().any(|&count| count == 3_usize)
            && counts.values().any(|&count| count == 2_usize)
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.0
            .iter()
            .counts()
            .values()
            .any(|&count| count == 3_usize)
    }

    fn is_two_pair(&self) -> bool {
        self.0
            .iter()
            .counts()
            .values()
            .filter(|&&count| count == 2_usize)
            .count()
            == 2
    }

    fn is_one_pair(&self) -> bool {
        self.0
            .iter()
            .counts()
            .values()
            .any(|&count| count == 2_usize)
    }
}

impl PlayingCardsJoker {
    fn jokers(&self) -> usize {
        self.0
            .iter()
            .filter(|&&card| card == PlayingCardJoker::Joker)
            .count()
    }

    fn is_five_of_a_kind(&self) -> bool {
        let jokers = self.jokers();
        self.0
            .iter()
            .counts()
            .values()
            .any(|&count| count == 5_usize)
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 4_usize)
                && jokers == 1
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 3_usize)
                && jokers == 2
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 2_usize)
                && jokers == 3
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 2_usize)
                && jokers == 3
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 1_usize)
                && jokers == 4
    }

    fn is_four_of_a_kind(&self) -> bool {
        let jokers = self.jokers();
        self.0
            .iter()
            .counts()
            .values()
            .any(|&count| count == 4_usize)
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 3_usize)
                && jokers == 1
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 2_usize)
                && jokers == 2
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 1_usize)
                && jokers == 3
    }

    fn is_full_house(&self) -> bool {
        let counts = self.0.iter().counts();
        let jokers = self.jokers();
        counts.values().any(|&count| count == 3_usize)
            && counts.values().any(|&count| count == 2_usize)
            || self
                .0
                .iter()
                .counts()
                .values()
                .filter(|&&count| count == 2_usize)
                .count()
                == 2
                && jokers > 0
    }

    fn is_three_of_a_kind(&self) -> bool {
        let jokers = self.jokers();
        self.0
            .iter()
            .counts()
            .values()
            .any(|&count| count == 3_usize)
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 2_usize)
                && jokers == 1
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 1_usize)
                && jokers == 2
    }

    fn is_two_pair(&self) -> bool {
        self.0
            .iter()
            .counts()
            .values()
            .filter(|&&count| count == 2_usize)
            .count()
            == 2
    }

    fn is_one_pair(&self) -> bool {
        let jokers = self.jokers();
        self.0
            .iter()
            .counts()
            .values()
            .any(|&count| count == 2_usize)
            || self
                .0
                .iter()
                .filter(|&&card| card != PlayingCardJoker::Joker)
                .counts()
                .values()
                .any(|&count| count == 1_usize)
                && jokers == 1
    }
}

impl From<&PlayingCards> for PlayingCardsType {
    fn from(value: &PlayingCards) -> Self {
        if value.is_five_of_a_kind() {
            PlayingCardsType::FiveOfAKind
        } else if value.is_four_of_a_kind() {
            PlayingCardsType::FourOfAKind
        } else if value.is_full_house() {
            PlayingCardsType::FullHouse
        } else if value.is_three_of_a_kind() {
            PlayingCardsType::ThreeOfAKind
        } else if value.is_two_pair() {
            PlayingCardsType::TwoPair
        } else if value.is_one_pair() {
            PlayingCardsType::OnePair
        } else {
            PlayingCardsType::HighCard
        }
    }
}

impl From<&PlayingCardsJoker> for PlayingCardsType {
    fn from(value: &PlayingCardsJoker) -> Self {
        if value.is_five_of_a_kind() {
            PlayingCardsType::FiveOfAKind
        } else if value.is_four_of_a_kind() {
            PlayingCardsType::FourOfAKind
        } else if value.is_full_house() {
            PlayingCardsType::FullHouse
        } else if value.is_three_of_a_kind() {
            PlayingCardsType::ThreeOfAKind
        } else if value.is_two_pair() {
            PlayingCardsType::TwoPair
        } else if value.is_one_pair() {
            PlayingCardsType::OnePair
        } else {
            PlayingCardsType::HighCard
        }
    }
}

impl PartialEq for PlayingCards {
    fn eq(&self, other: &Self) -> bool {
        let matching = self.0.iter().zip(&other.0).filter(|&(a, b)| a == b).count();
        matching == self.0.len() && matching == other.0.len()
    }
}

impl Eq for PlayingCards {}

impl PartialOrd for PlayingCards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayingCards {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = Into::<PlayingCardsType>::into(self).cmp(&other.into());
        if ord == Ordering::Equal {
            for (&a, b) in self.0.iter().zip(other.0) {
                let ord = a.cmp(&b);
                if ord != Ordering::Equal {
                    return ord;
                }
            }

            ord
        } else {
            ord
        }
    }
}

impl PartialEq for PlayingCardsJoker {
    fn eq(&self, other: &Self) -> bool {
        let matching = self.0.iter().zip(&other.0).filter(|&(a, b)| a == b).count();
        matching == self.0.len() && matching == other.0.len()
    }
}

impl Eq for PlayingCardsJoker {}

impl PartialOrd for PlayingCardsJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayingCardsJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = Into::<PlayingCardsType>::into(self).cmp(&other.into());
        if ord == Ordering::Equal {
            for (&a, b) in self.0.iter().zip(other.0) {
                let ord = a.cmp(&b);
                if ord != Ordering::Equal {
                    return ord;
                }
            }

            ord
        } else {
            ord
        }
    }
}

impl FromStr for PlayingCards {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 5 {
            let cards: Vec<PlayingCard> = s.chars().map(|c| c.into()).collect();
            Ok(PlayingCards([
                cards[0], cards[1], cards[2], cards[3], cards[4],
            ]))
        } else {
            Err("parse error")
        }
    }
}

impl FromStr for PlayingCardsJoker {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 5 {
            let cards: Vec<PlayingCardJoker> = s.chars().map(|c| c.into()).collect();
            Ok(PlayingCardsJoker([
                cards[0], cards[1], cards[2], cards[3], cards[4],
            ]))
        } else {
            Err("parse error")
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (cards, bid) = l.split_once(' ').unwrap();
                let cards: PlayingCards = cards.parse().unwrap();
                let bid: u32 = bid.parse().unwrap();
                (cards, bid)
            })
            .sorted_by(|(a, _), (b, _)| a.cmp(b))
            .enumerate()
            .map(|(i, (_, bid))| bid * (i as u32 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (cards, bid) = l.split_once(' ').unwrap();
                let cards: PlayingCardsJoker = cards.parse().unwrap();
                let bid: u32 = bid.parse().unwrap();
                (cards, bid)
            })
            .sorted_by(|(a, _), (b, _)| a.cmp(b))
            .enumerate()
            .map(|(i, (_, bid))| bid * (i as u32 + 1))
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
