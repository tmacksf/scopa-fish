use rand::prelude::*;
// use std::io;

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

impl Suit {
    pub fn suits() -> Vec<Suit> {
        vec![Suit::Spades, Suit::Clubs, Suit::Diamonds, Suit::Hearts]
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Val {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Jack,
    Queen,
    King,
}

impl Val {
    pub fn vals() -> Vec<Val> {
        vec![
            Val::One,
            Val::Two,
            Val::Three,
            Val::Four,
            Val::Five,
            Val::Six,
            Val::Seven,
            Val::Jack,
            Val::Queen,
            Val::King,
        ]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Card {
    suit: Suit,
    val: Val,
}

impl Card {
    pub fn new(suit: Suit, val: Val) -> Card {
        Card { suit, val }
    }
}

pub struct Player {
    hand: Vec<Card>,
    pond: Vec<Card>,
}

impl Player {
    pub fn split_the_deck(cards: Vec<Card>) {
        todo!();
    }
}

pub struct Game {
    players: Vec<Player>,
}

fn new_deck() -> Vec<Card> {
    let vals = Val::vals();
    let suits = Suit::suits();
    let mut deck = vec![];
    for v in vals {
        for s in &suits {
            deck.push(Card::new(*s, v));
        }
    }
    deck
}

fn main() {
    let mut rng = rand::rng();
    // create a deck
    let mut deck = new_deck();
    deck.shuffle(&mut rng);

    for i in 0..deck.len() {
        print!("{:?}, ", deck[i]);
    }
    println!();

    println!("Hello, world!");
}
