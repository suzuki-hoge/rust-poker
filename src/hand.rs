use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::card::{Card, Cards, Joker, Rank};
use crate::card::Card::JokerCard;
use crate::card::Card::RankCard;
use crate::Cards::{JokerCards, NoJokerCards};
use crate::hand::HandName::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Hand {
    name: HandName,
    card: Card,
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.card {
            RankCard { value } => write!(f, "{:?} ( strongest: {:?} )", self.name, value),
            JokerCard { value } => write!(f, "{:?} ( strongest: {:?} )", self.name, value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HandName {
    HighCards,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalStraightFlush,
}

pub fn judge(cards: &Cards) -> Hand {
    vec![
        royal_straight_flush(cards),
        straight_flush(cards),
        four_of_a_kind(cards),
        full_house(cards),
        flush(cards),
        straight(cards),
        three_of_a_kind(cards),
        two_pair(cards),
        one_pair(cards),
        high_cards(cards),
    ].into_iter().flatten().collect_vec()[0]
}

fn high_cards(cards: &Cards) -> Option<Hand> {
    match cards {
        JokerCards { joker: _, rank_cards: _ } => {
            Some(Hand { name: HighCards, card: JokerCard { value: Joker {} } })
        }
        NoJokerCards { rank_cards } => {
            Some(Hand { name: HighCards, card: RankCard { value: rank_cards[rank_cards.len() - 1] } })
        }
    }
}

fn one_pair(cards: &Cards) -> Option<Hand> {
    match cards {
        JokerCards { joker: _, rank_cards } => {
            n_same_numbers_expect_m_count(1, 4, &rank_cards).map(|_| Hand { name: OnePair, card: JokerCard { value: Joker {} } })
        }
        NoJokerCards { rank_cards } => {
            n_same_numbers_expect_m_count(2, 1, &rank_cards).map(|&r| Hand { name: OnePair, card: RankCard { value: r } })
        }
    }
}

fn two_pair(cards: &Cards) -> Option<Hand> {
    match cards {
        JokerCards { joker: _, rank_cards } => {
            n_same_numbers_expect_m_count(2, 1, &rank_cards).map(|_| Hand { name: TwoPair, card: JokerCard { value: Joker {} } })
        }
        NoJokerCards { rank_cards } => {
            n_same_numbers_expect_m_count(2, 2, &rank_cards).map(|&r| Hand { name: TwoPair, card: RankCard { value: r } })
        }
    }
}

fn three_of_a_kind(cards: &Cards) -> Option<Hand> {
    match cards {
        JokerCards { joker: _, rank_cards } => {
            n_same_numbers_expect_m_count(2, 1, &rank_cards).map(|_| Hand { name: ThreeOfAKind, card: JokerCard { value: Joker {} } })
        }
        NoJokerCards { rank_cards } => {
            n_same_numbers_expect_m_count(3, 1, &rank_cards).map(|&r| Hand { name: ThreeOfAKind, card: RankCard { value: r } })
        }
    }
}

fn straight(cards: &Cards) -> Option<Hand> {
    match cards {
        JokerCards { joker: _, rank_cards } => {
            let patterns: Vec<Vec<u8>> = vec![
                vec![0, 1, 2, 3],
                vec![0, 2, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 4],
            ];
            patterns.into_iter().find_map(
                |pattern| sequential(&rank_cards, pattern).map(|_| Hand { name: Straight, card: JokerCard { value: Joker {} } })
            )
        }
        NoJokerCards { rank_cards } => {
            sequential(&rank_cards, vec![0, 1, 2, 3, 4]).map(|&r| Hand { name: Straight, card: RankCard { value: r } })
        }
    }
}

fn flush(cards: &Cards) -> Option<Hand> {
    match cards {
        JokerCards { joker: _, rank_cards } => {
            same_suits(&rank_cards).map(|_| Hand { name: Flush, card: JokerCard { value: Joker {} } })
        }
        NoJokerCards { rank_cards } => {
            same_suits(&rank_cards).map(|&r| Hand { name: Flush, card: RankCard { value: r } })
        }
    }
}

fn full_house(cards: &Cards) -> Option<Hand> {
    match cards {
        JokerCards { joker: _, rank_cards } => {
            n_same_numbers_expect_m_count(2, 2, &rank_cards).map(|_| Hand { name: FullHouse, card: JokerCard { value: Joker {} } })
        }
        NoJokerCards { rank_cards } => {
            let high = n_same_numbers_expect_m_count(3, 1, &rank_cards);
            let low = n_same_numbers_expect_m_count(2, 1, &rank_cards);
            if high.is_some() && low.is_some() {
                high.map(|&r| Hand { name: FullHouse, card: RankCard { value: r } })
            } else {
                None
            }
        }
    }
}

fn four_of_a_kind(cards: &Cards) -> Option<Hand> {
    match cards {
        JokerCards { joker: _, rank_cards } => {
            n_same_numbers_expect_m_count(3, 1, &rank_cards).map(|_| Hand { name: FourOfAKind, card: JokerCard { value: Joker {} } })
        }
        NoJokerCards { rank_cards } => {
            n_same_numbers_expect_m_count(4, 1, &rank_cards).map(|&r| Hand { name: FourOfAKind, card: RankCard { value: r } })
        }
    }
}

fn straight_flush(cards: &Cards) -> Option<Hand> {
    straight(cards).and_then(|_| flush(cards)).map(|hand| Hand { name: StraightFlush, card: hand.card })
}

fn royal_straight_flush(cards: &Cards) -> Option<Hand> {
    match cards {
        JokerCards { joker: _, rank_cards } => {
            if rank_cards[0].number == 10 || rank_cards[0].number == 11 {
                straight(cards).and_then(|_| flush(cards)).map(|hand| Hand { name: RoyalStraightFlush, card: hand.card })
            } else {
                None
            }
        }
        NoJokerCards { rank_cards } => {
            if rank_cards[0].number == 10 {
                straight(cards).and_then(|_| flush(cards)).map(|hand| Hand { name: RoyalStraightFlush, card: hand.card })
            } else {
                None
            }
        }
    }
}

fn n_same_numbers_expect_m_count(n: usize, m: usize, ranks: &[Rank]) -> Option<&Rank> {
    let mut grouped = ranks.iter()
        .into_group_map_by(|rank| rank.number).into_iter()
        .filter(|(_, ranks)| ranks.len() == n).collect_vec();
    grouped.sort();
    grouped.reverse();

    if grouped.len() == m {
        Some(grouped[0].1[0])
    } else {
        None
    }
}

fn sequential(ranks: &[Rank], pattern: Vec<u8>) -> Option<&Rank> {
    let head = ranks[0].number;
    let numbers = ranks.iter().map(|rank| rank.number).collect_vec();
    if numbers == pattern.iter().map(|n| n + head).collect_vec() {
        Some(&ranks[ranks.len() - 1])
    } else {
        None
    }
}

fn same_suits(ranks: &[Rank]) -> Option<&Rank> {
    let head = ranks[0].suit;
    if ranks.iter().map(|rank| rank.suit).all(|suit| suit == head) {
        Some(&ranks[ranks.len() - 1])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Suit::*;
    use crate::hand::*;

    #[test]
    fn one_pair_test() {
        let cards = Cards::parse("S-2 H-3 S-4 D-5 Joker");
        assert_eq!(one_pair(&cards), Some(Hand { name: OnePair, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 H-3 S-4 D-5 C-5");
        assert_eq!(one_pair(&cards), Some(Hand { name: OnePair, card: RankCard { value: Rank { number: 5, suit: D } } }));

        let cards = Cards::parse("S-2 H-3 S-4 D-5 C-6");
        assert_eq!(one_pair(&cards), None);
    }

    #[test]
    fn two_pair_test() {
        let cards = Cards::parse("S-2 H-3 S-4 D-4 Joker");
        assert_eq!(two_pair(&cards), Some(Hand { name: TwoPair, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 H-3 S-3 D-5 C-5");
        assert_eq!(two_pair(&cards), Some(Hand { name: TwoPair, card: RankCard { value: Rank { number: 5, suit: D } } }));

        let cards = Cards::parse("S-2 H-3 S-4 D-5 C-6");
        assert_eq!(two_pair(&cards), None);
    }

    #[test]
    fn three_of_a_kind_test() {
        let cards = Cards::parse("S-2 H-3 S-4 D-4 Joker");
        assert_eq!(three_of_a_kind(&cards), Some(Hand { name: ThreeOfAKind, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 H-3 S-5 D-5 C-5");
        assert_eq!(three_of_a_kind(&cards), Some(Hand { name: ThreeOfAKind, card: RankCard { value: Rank { number: 5, suit: S } } }));

        let cards = Cards::parse("S-2 H-3 S-4 D-5 C-6");
        assert_eq!(three_of_a_kind(&cards), None);
    }

    #[test]
    fn straight_test() {
        let cards = Cards::parse("S-2 H-3 S-4 D-5 Joker");
        assert_eq!(straight(&cards), Some(Hand { name: Straight, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 H-3 S-4 Joker D-6");
        assert_eq!(straight(&cards), Some(Hand { name: Straight, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 H-3 Joker S-5 D-6");
        assert_eq!(straight(&cards), Some(Hand { name: Straight, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 Joker H-4 S-5 D-6");
        assert_eq!(straight(&cards), Some(Hand { name: Straight, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("Joker S-3 H-4 S-5 D-6");
        assert_eq!(straight(&cards), Some(Hand { name: Straight, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 H-3 S-4 D-5 C-6");
        assert_eq!(straight(&cards), Some(Hand { name: Straight, card: RankCard { value: Rank { number: 6, suit: C } } }));

        let cards = Cards::parse("S-2 H-3 S-4 D-5 C-7");
        assert_eq!(straight(&cards), None);
    }

    #[test]
    fn flush_test() {
        let cards = Cards::parse("S-2 S-3 S-4 S-5 Joker");
        assert_eq!(flush(&cards), Some(Hand { name: Flush, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 S-3 S-4 S-5 S-6");
        assert_eq!(flush(&cards), Some(Hand { name: Flush, card: RankCard { value: Rank { number: 6, suit: S } } }));

        let cards = Cards::parse("S-2 S-3 S-4 S-5 C-7");
        assert_eq!(flush(&cards), None);
    }

    #[test]
    fn full_house_test() {
        let cards = Cards::parse("S-2 H-2 S-4 D-4 Joker");
        assert_eq!(full_house(&cards), Some(Hand { name: FullHouse, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 H-2 S-5 D-5 C-5");
        assert_eq!(full_house(&cards), Some(Hand { name: FullHouse, card: RankCard { value: Rank { number: 5, suit: S } } }));

        let cards = Cards::parse("S-2 H-2 S-4 D-6 C-6");
        assert_eq!(full_house(&cards), None);
    }

    #[test]
    fn four_of_a_kind_test() {
        let cards = Cards::parse("S-2 H-4 S-4 D-4 Joker");
        assert_eq!(four_of_a_kind(&cards), Some(Hand { name: FourOfAKind, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 H-5 S-5 D-5 C-5");
        assert_eq!(four_of_a_kind(&cards), Some(Hand { name: FourOfAKind, card: RankCard { value: Rank { number: 5, suit: S } } }));

        let cards = Cards::parse("S-2 H-3 S-5 D-5 C-5");
        assert_eq!(four_of_a_kind(&cards), None);
    }

    #[test]
    fn straight_flush_test() {
        let cards = Cards::parse("S-2 S-3 S-4 S-5 Joker");
        assert_eq!(straight_flush(&cards), Some(Hand { name: StraightFlush, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-2 S-3 S-4 S-5 S-6");
        assert_eq!(straight_flush(&cards), Some(Hand { name: StraightFlush, card: RankCard { value: Rank { number: 6, suit: S } } }));

        let cards = Cards::parse("S-2 S-3 S-4 S-5 H-6");
        assert_eq!(straight_flush(&cards), None);

        let cards = Cards::parse("S-2 S-3 S-4 S-5 S-7");
        assert_eq!(straight_flush(&cards), None);
    }

    #[test]
    fn royal_straight_flush_test() {
        let cards = Cards::parse("S-10 S-J S-Q S-K Joker");
        assert_eq!(royal_straight_flush(&cards), Some(Hand { name: RoyalStraightFlush, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("Joker S-J S-Q S-K S-A");
        assert_eq!(royal_straight_flush(&cards), Some(Hand { name: RoyalStraightFlush, card: JokerCard { value: Joker {} } }));

        let cards = Cards::parse("S-10 S-J S-Q S-K S-A");
        assert_eq!(royal_straight_flush(&cards), Some(Hand { name: RoyalStraightFlush, card: RankCard { value: Rank { number: 14, suit: S } } }));

        let cards = Cards::parse("S-9 S-10 S-J S-Q S-K");
        assert_eq!(royal_straight_flush(&cards), None);
    }

    #[test]
    fn judge_test() {
        let cards = Cards::parse("S-2 H-3 S-4 D-4 Joker");
        assert_eq!(judge(&cards), Hand { name: ThreeOfAKind, card: JokerCard { value: Joker {} } });

        let cards = Cards::parse("S-2 H-4 S-7 D-9 C-K");
        assert_eq!(judge(&cards), Hand { name: HighCards, card: RankCard { value: Rank { number: 13, suit: C } } });
    }
}
