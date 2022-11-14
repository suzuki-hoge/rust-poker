use std::io::{stdout, Write};

use crate::card::Cards;
use crate::hand::judge;

mod card;
mod hand;

fn main() {
    println!("( example ) input: S-2 H-A D-J Joker C-10");
    print!("input: ");
    stdout().flush().unwrap();

    let cards = Cards::parse(&read_s());

    println!("\n{:?}", judge(&cards));
}

// stdin

fn read_s() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}
