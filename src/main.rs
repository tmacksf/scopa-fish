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
    pub fn new() -> Player {
        Player {
            hand: Vec::new(),
            pond: Vec::new(),
        }
    }
}

pub struct Game {
    players: Vec<Player>,
    deck: Vec<Card>,
}

impl Game {
    pub fn new() -> Game {
        let mut g = Game {
            players: vec![Player::new(), Player::new()], // TODO(tommy): Add more player option
            deck: Vec::new(),
        };
        g.new_deck();
        return g;
    }

    fn new_deck(&mut self) {
        let vals = Val::vals();
        let suits = Suit::suits();
        let mut deck = vec![];
        for v in vals {
            for s in &suits {
                deck.push(Card::new(*s, v));
            }
        }
        self.deck = deck;
    }

    pub fn shuffle(&mut self) {
        todo!("Theo");
    }
}

fn main() {
    // create a game
    let mut game = Game::new();
    game.shuffle();
    for i in 0..game.deck.len() {
        print!("{:?}, ", game.deck[i]);
    }
    println!();

    println!("Hello, world!");
}
