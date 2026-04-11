use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::prelude::*;

// TODOS:
// Proper error handling
//

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
    hand: HashSet<Card>,
    pond: Vec<Card>,
    score: u64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hand: HashSet::new(),
            pond: Vec::new(),
            score: 0,
        }
    }

    pub fn give_card(&mut self, card: Card) {
        self.hand.insert(card);
    }

    pub fn give_pond(&mut self, card: Card) {
        self.pond.push(card);
    }

    pub fn count_points(&mut self) {
        todo!();
    }

    pub fn debug_print(&self) {
        println!("Hand: {:?}", self.hand);
        println!("Pond: {:?}", self.pond);
        println!("Score: {}", self.score);
    }

    pub fn hand_empty(&self) -> bool {
        self.hand.len() == 0
    }

    pub fn remove_card(&mut self, c: &Card) {
        match self.hand.remove(c) {
            true => {}
            false => panic!("Card not present in players hand: {}", c),
        };
    }
}

pub struct Game {
    players: Vec<Player>,
    turn: usize, // turn corresponds to the index of a player in the players vec
    table: HashSet<Card>,
    deck: Vec<Card>,
    moves: Vec<Move>,
}

impl Game {
    pub fn new() -> Game {
        let mut g = Game {
            players: vec![Player::new(), Player::new()], // TODO(tommy): Add more player option
            deck: Vec::new(),
            table: HashSet::new(),
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
            let c = self.deck.pop().unwrap();
            self.play_card(c);
        }
    }

    pub fn deal_users(&mut self) {
        for _ in 0..3 {
            for i in 0..self.players.len() {
                self.players[i].give_card(self.deck.pop().unwrap());
            }
        }
    }

    pub fn next_turn(&mut self) {
        self.turn = (self.turn + 1) % 2
    }

    pub fn play_card(&mut self, card: Card) {
        match self.table.insert(card) {
            true => {}
            false => panic!("Could not put card on table"),
        }
    }

    pub fn take_card(&mut self, card: &Card) {
        match self.table.remove(card) {
            true => {}
            false => panic!("Card not present on table: {}", card),
        }
    }

    pub fn shuffle(&mut self) {
        // todo!("Theo");
    }

    pub fn debug_state(&self, all_data: bool) {
        println!("\nTurn: {}", self.turn);
        for i in 0..self.players.len() {
            self.players[i].debug_print();
        }

        println!("\nTable: ");
        let _ = self
            .table
            .iter()
            .map(|c| println!("{}", c))
            .collect::<Vec<()>>();
        if all_data {
            println!("\nDeck: ");
            let _ = self
                .deck
                .iter()
                .map(|c| println!("{}", c))
                .collect::<Vec<()>>();
            println!("\nMoves: {:?}", self.moves);
        }
    }

    pub fn valid_move(&self, _mv: &Move) -> bool {
        true
    }

    pub fn do_move(&mut self, mv: &Move) {
        // This assumes a move has been checked to be valid.
        // if the move is not valid the program will panic
        let p = self.turn;
        match mv {
            Move::Down(m) => {
                self.players[p].remove_card(m);
                self.play_card(*m);
            }
            Move::Up(m) => {
                for c in m.iter() {
                    self.take_card(c);
                    self.players[p].give_pond(*c);
                }
            }
        };
    }

    pub fn push_move(&mut self, mv: &Move) {
        self.moves.push(mv.clone());
    }

    pub fn all_hands_empty(&self) -> bool {
        let mut empty = true;
        for i in 0..self.players.len() {
            empty &= self.players[i].hand_empty();
        }
        empty
    }
}

#[derive(Clone, Debug)]
pub enum Move {
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
            return Ok(Move::Down(c));
            // TODO(all): Figure this out
            // could just ignore any garbage after
            // or we could interpret it as a second move
            // if iter.next().is_some() {
            //     return Err(format!("Too many arguments {}", c2));
            // }
        } else {
            let mut v = vec![];
            for i in iter {
                let c = Card::parse(i)?;
                v.push(c);
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
    game.debug_state(true);

    loop {
        game.debug_state(false);
        // TODO: End the game
        if game.all_hands_empty() {
            game.deal_users();
        }

        let mv = match get_input() {
            Ok(c) => c,
            Err(e) => {
                println!("Err: {}", e);
                continue;
            }
        };

        // maybe can have something here that checks if a move is valid and resturns an enum for
        // the followup. Could be Invalid, Continue, PickupRequired or something like that.
        // For invalid: just continue and get user input again (nothing has happened to the game)
        // For Continue: do the move and next turn
        // For PickupRequired: do the move and enter a new loop where the user has to do a second
        // valid pickup move. Once that is done then break out of the second loop and continue the
        // game
        if !game.valid_move(&mv) {
            println!("Invalid move: {:?}", mv);
            continue;
        }

        game.do_move(&mv);
        game.push_move(&mv);

        // after everything is done, switch to the other player's turn and continue
        game.next_turn();
    }
}
