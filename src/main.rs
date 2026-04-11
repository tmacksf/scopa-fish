use std::fmt;
use std::io;
use std::io::prelude::*;

// TODOS:
// Proper error handling
//

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

    pub fn from_char(c: char) -> Result<Suit, String> {
        match c {
            'S' | 's' => Ok(Suit::Spades),
            'C' | 'c' => Ok(Suit::Clubs),
            'D' | 'd' => Ok(Suit::Diamonds),
            'H' | 'h' => Ok(Suit::Hearts),
            _ => Err(format!("Coult not convert '{}' to suit", c)),
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

    pub fn from_char(c: char) -> Result<Value, String> {
        match c {
            '1' => Ok(Value::One),
            '2' => Ok(Value::Two),
            '3' => Ok(Value::Three),
            '4' => Ok(Value::Four),
            '5' => Ok(Value::Five),
            '6' => Ok(Value::Six),
            '7' => Ok(Value::Seven),
            'J' | 'j' => Ok(Value::Jack),
            'Q' | 'q' => Ok(Value::Queen),
            'K' | 'k' => Ok(Value::King),
            _ => Err(format!("Could not convert '{}' to a value", c)),
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

    pub fn parse(s: &str) -> Result<Card, String> {
        if s.len() != 2 {
            return Err(format!("string must be of length two: {}", s));
        }
        let v = Value::from_char(s.chars().nth(0).unwrap())?;
        let s = Suit::from_char(s.chars().nth(1).unwrap())?;
        Ok(Card::new(s, v))
    }
}

impl fmt::Display for Card {
    // Required method
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({:?}, {:?})", self.val, self.suit)
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
        println!("Score: {}", self.score);
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
            .map(|c| println!("{}", c))
            .collect::<Vec<()>>();
        println!("\nDeck: ");
        let _ = self
            .deck
            .iter()
            .map(|c| println!("{}", c))
            .collect::<Vec<()>>();
        println!("\nTurn: {}", self.turn);
        println!("\nMoves: {:?}", self.moves);
    }
}

#[derive(Clone, Debug)]
enum Move {
    Down(Card),
    Up(Vec<Card>),
}

fn get_input() -> Result<Move, String> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let mut iter = l.split_whitespace();
        let down = match iter.next() {
            Some("u") | Some("U") => false,
            Some("d") | Some("D") => true,
            _ => return Err(format!("Needs to start with u/U or d/D")),
        };
        // down is just (value, suit); up is a list of (value, suits)
        if down {
            let c = match iter.next() {
                Some(b) => Card::parse(b)?,
                _ => return Err(format!("Need a second argument for down")),
            };
            // TODO(tommy): Confirm it is the only card being played
            return Ok(Move::Down(c));
        } else {
            // TODO(tommy): Probably nicer to convert to some sort of map
            // let v: Vec<Card> = iter.map(|s: &str| { return Card::parse(s); }).collect();
            let mut v = vec![];
            for i in iter {
                let c = Card::parse(i)?;
                v.push(c);
            }
            if v.len() == 0 {
                return Err(format!("No arguments for pick up"));
            }
            return Ok(Move::Up(v));
        }
    }
    return Err(format!("Could not parse input"));
}

fn main() {
    // create a game
    let mut game = Game::new();
    game.shuffle();
    game.init_table();
    game.deal_users();
    game.debug_state();

    loop {
        let res = match get_input() {
            Ok(c) => c,
            Err(e) => {
                println!("Err: {}", e);
                continue;
            }
        };
        println!("{:?}", res);
    }
}
