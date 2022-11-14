use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::card::Suit::*;
use crate::Cards::{JokerCards, NoJokerCards};

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum Cards {
    JokerCards { joker: Joker, rank_cards: Vec<Rank> },
    NoJokerCards { rank_cards: Vec<Rank> },
}

impl Debug for Cards {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JokerCards { joker, rank_cards: values } => write!(f, "{:?}, {:?}", values, joker),
            NoJokerCards { rank_cards: values } => write!(f, "{:?}", values)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Card {
    RankCard { value: Rank },
    JokerCard { value: Joker },
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub struct Joker {}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub struct Rank {
    pub number: u8,
    pub suit: Suit,
}

impl Debug for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}-{:?}", self.suit, self.number)
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum Suit {
    S,
    H,
    D,
    C,
}

impl Cards {
    pub fn parse(s: &str) -> Self {
        if s.split(' ').collect_vec().len() != 5 {
            panic!("parse error");
        }
        if s.split(' ').unique().collect_vec().len() != 5 {
            panic!("parse error");
        }

        let jokers = s.split(' ').filter(|&word| word == "Joker").collect_vec().len();

        if 1 < jokers {
            panic!("parse error");
        }
        let rank_card_opts;
        if jokers == 1 {
            rank_card_opts = s.split(' ').filter(|&word| word != "Joker").map(|s2| Rank::parse(s2)).collect_vec();
        } else {
            rank_card_opts = s.split(' ').map(|s2| Rank::parse(s2)).collect_vec();
        }
        if rank_card_opts.iter().all(|card_opt| card_opt.is_some()) {
            let mut rank_kards = rank_card_opts.into_iter().map(|card_opt| card_opt.unwrap()).collect_vec();
            rank_kards.sort();
            if jokers == 1 {
                JokerCards { joker: Joker {}, rank_cards: rank_kards }
            } else {
                NoJokerCards { rank_cards: rank_kards }
            }
        } else {
            panic!("parse error");
        }
    }
}

impl Rank {
    fn parse(s: &str) -> Option<Self> {
        let number_opt = match s.split('-').collect_vec()[1] {
            "J" => Some(11),
            "Q" => Some(12),
            "K" => Some(13),
            "A" => Some(14),
            s => s.parse::<u8>().ok()
        };
        let suit_opt = match s.split('-').collect_vec()[0] {
            "S" => Some(S),
            "H" => Some(H),
            "D" => Some(D),
            "C" => Some(C),
            _ => None
        };
        suit_opt.and_then(|suit| number_opt.map(|number| Rank { number, suit }))
    }
}
