use std::io;
use std::io::prelude::*;

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

    pub fn from_char(c: char) -> Option<Suit> {
        match c {
            'S' | 's' => Some(Suit::Spades),
            'C' | 'c' => Some(Suit::Clubs),
            'D' | 'd' => Some(Suit::Diamonds),
            'H' | 'h' => Some(Suit::Spades),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Value {
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

impl Value {
    pub fn vals() -> Vec<Value> {
        vec![
            Value::One,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Jack,
            Value::Queen,
            Value::King,
        ]
    }

    pub fn from_char(c: char) -> Option<Value> {
        match c {
            '1' => Some(Value::One),
            '2' => Some(Value::Two),
            '3' => Some(Value::Three),
            '4' => Some(Value::Four),
            '5' => Some(Value::Five),
            '6' => Some(Value::Six),
            '7' => Some(Value::Seven),
            'J' | 'j' => Some(Value::Jack),
            'Q' | 'q' => Some(Value::Queen),
            'K' | 'k' => Some(Value::King),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Card {
    suit: Suit,
    val: Value,
}

impl Card {
    pub fn new(suit: Suit, val: Value) -> Card {
        Card { suit, val }
    }
}

pub struct Player {
    hand: Vec<Card>,
    pond: Vec<Card>,
    score: u64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hand: Vec::new(),
            pond: Vec::new(),
            score: 0,
        }
    }

    pub fn give_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn count_points(&mut self) {
        todo!();
    }

    pub fn debug_print(&self) {
        println!("Hand: {:?}", self.hand);
        println!("Pond: {:?}", self.pond);
        println!("Score: {:?}", self.score);
    }
}

pub struct Game {
    players: Vec<Player>,
    table: Vec<Card>,
    deck: Vec<Card>,
    turn: usize,
    moves: Vec<Move>,
}

impl Game {
    pub fn new() -> Game {
        let mut g = Game {
            players: vec![Player::new(), Player::new()], // TODO(tommy): Add more player option
            deck: Vec::new(),
            table: Vec::new(),
            turn: 0,
            moves: Vec::new(),
        };
        g.new_deck();
        return g;
    }

    fn new_deck(&mut self) {
        let vals = Value::vals();
        let suits = Suit::suits();
        let mut deck = Vec::new();
        for v in 0..vals.len() {
            for s in 0..suits.len() {
                let card = Card::new(suits[s], vals[v]);
                deck.push(card);
            }
        }
        self.deck = deck;
    }

    pub fn init_table(&mut self) {
        for _ in 0..4 {
            self.table.push(self.deck.pop().unwrap());
        }
    }

    pub fn deal_users(&mut self) {
        for _ in 0..3 {
            for i in 0..self.players.len() {
                self.players[i].give_card(self.deck.pop().unwrap());
            }
        }
    }

    pub fn play_card(&mut self) {}

    pub fn take_card(&mut self) {}

    pub fn shuffle(&mut self) {
        // todo!("Theo");
    }

    pub fn debug_state(&self) {
        for i in 0..self.players.len() {
            self.players[i].debug_print();
        }

        println!("\nTable: ");
        let _ = self
            .table
            .iter()
            .map(|c| println!("{:?}", c))
            .collect::<Vec<()>>();
        println!("\nDeck: ");
        let _ = self
            .deck
            .iter()
            .map(|c| println!("{:?}", c))
            .collect::<Vec<()>>();
        println!("\nTurn: {}", self.turn);
    }
}

enum Move {
    Down(Card),
    Up(Vec<Card>),
}

fn get_input() -> Option<Move> {
    let mut m: Option<Move> = None;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let mut iter = l.split_whitespace();
        let mut down = false;
        match iter.next() {
            Some("u") | Some("U") => down = true,
            Some("d") | Some("D") => down = false,
            _ => return None,
        }
        // down is just (value, suit); up is a list of (value, suits)
        if down {
            let c = match iter.next() {
                Some(b) => {
                    // TODO(tommy): parse
                    Card::new(Suit::Spades, Value::Six)
                }
                _ => return None,
            };
            return Some(Move::Down(c));
        } else {
            let v = vec![];
            for i in iter {
                // TODO(tommy): parse and push
                // v.push();
                println!("{:?}", i);
            }
            if v.len() == 0 {
                return None;
            }
            return Some(Move::Up(v));
        }
    }
    return m;
}

fn main() {
    // create a game
    let mut game = Game::new();
    game.shuffle();
    game.init_table();
    game.deal_users();
    game.debug_state();

    loop {
        get_input();
        break;
    }

    println!("Hello, world!");
}
