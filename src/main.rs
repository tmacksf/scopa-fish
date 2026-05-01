use rand::prelude::*;
use rand::rng;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::ops::Add;

// use rustc_hash::FxHashSet;
// Could move to this because the hashes are tiny
// could also build our own hashset (just a vector of len 40 (80 bytes for cards))
// Cargo.toml: rustc-hash = "1.1"

// TODOS:
// Proper error handling (replace strings in Result with errors) - Tommy
// Randomness (shuffling) - Theo
// Checking if moves take the least amount of cards - Alex
// Scoring - Tommy

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

    pub fn bitmask(&self) -> u8 {
        match self {
            Suit::Spades => 0b00000000,
            Suit::Clubs => 0b00000001,
            Suit::Diamonds => 0b00000010,
            Suit::Hearts => 0b00000011,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Value {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Jack = 8,
    Queen = 9,
    King = 10,
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
            '1' | 'a' | 'A' => Ok(Value::One),
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

    pub fn bitmask(&self) -> u8 {
        match self {
            Value::One => 0b00000000,
            Value::Two => 0b00000100,
            Value::Three => 0b00001000,
            Value::Four => 0b00001100,
            Value::Five => 0b00010000,
            Value::Six => 0b00010100,
            Value::Seven => 0b00011000,
            Value::Jack => 0b00011100,
            Value::Queen => 0b00100000,
            Value::King => 0b00100100,
        }
    }
}

impl Add for Value {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        let res = (self as u8) + (other as u8);
        match res {
            1 => Some(Value::One),
            2 => Some(Value::Two),
            3 => Some(Value::Three),
            4 => Some(Value::Four),
            5 => Some(Value::Five),
            6 => Some(Value::Six),
            7 => Some(Value::Seven),
            8 => Some(Value::Jack),
            9 => Some(Value::Queen),
            10 => Some(Value::King),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Card {
    val: Value,
    suit: Suit,
}

// Every card can be stored in 6 bits meaning a full 40 card deck needs
// 6*40 bits -> 240 bits or 3 bytes
impl Card {
    pub fn new(val: Value, suit: Suit) -> Card {
        Card { val, suit }
    }

    pub fn parse(s: &str) -> Result<Card, String> {
        if s.len() != 2 {
            return Err(format!("string must be of length two: {}", s));
        }
        // already know that len is 2 so unwrap is fine
        let v = Value::from_char(s.chars().nth(0).unwrap())?;
        let s = Suit::from_char(s.chars().nth(1).unwrap())?;
        Ok(Card::new(v, s))
    }

    fn _bitmask(&self) -> u8 {
        self.val.bitmask() | self.suit.bitmask()
    }
}

impl fmt::Display for Card {
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

    pub fn find_card(&self, v: Option<Value>, s: Option<Suit>) -> usize {
        let mut count = 0;

        for i in 0..self.pond.len() {
            let v_match = match v {
                Some(v) => self.pond[i].val == v,
                None => true,
            };
            let s_match = match s {
                Some(s) => self.pond[i].suit == s,
                None => true,
            };
            if v_match && s_match {
                count += 1;
            }
        }

        return count;
    }

    // Calculate score is only for two and four people
    // TODO(Tommy): adapt a 3 person calculate score
    pub fn calculate_score(&mut self) {
        // check if the player has card majority
        if self.pond.len() > 20 {
            self.score += 1;
        }
        // check if the player has diamond majority
        if self.find_card(None, Some(Suit::Diamonds)) > 5 {
            self.score += 1;
        }
        // see if there is a 7 of diamonds
        if self.find_card(Some(Value::Seven), Some(Suit::Diamonds)) > 0 {
            self.score += 1;
        }

        // see if there are majority sevens, sixes, fives, fours, threes, twos, aces..
        // TODO: Tommy
    }
}

pub struct Game {
    players: Vec<Player>,
    turn: usize, // turn corresponds to the index of a player in the players vec
    table: HashSet<Card>,
    deck: Vec<Card>,
    moves: Vec<Move>,
    ace_sweeps: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut g = Game {
            players: vec![Player::new(), Player::new()], // TODO(tommy): Add more player option
            deck: Vec::new(),
            table: HashSet::new(),
            turn: 0,
            moves: Vec::new(),
            ace_sweeps: true,
        };
        g.new_deck();
        return g;
    }

    fn new_deck(&mut self) {
        let suits = Suit::suits();
        for v in Value::vals().iter() {
            for s in &suits {
                self.deck.push(Card::new(*v, *s));
            }
        }
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
        self.turn = (self.turn + 1) % self.players.len()
    }

    pub fn play_card(&mut self, card: Card) {
        match self.table.insert(card) {
            true => {}
            false => panic!("Could not put card ({}) on table", card),
        }
    }

    pub fn take_card(&mut self, card: &Card) {
        match self.table.remove(card) {
            true => {}
            false => panic!("Card not present on table: {}", card),
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.deck.shuffle(&mut rng);
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
            Move::Up(m, cards) => {
                self.players[p].remove_card(m);
                self.players[p].give_pond(*m);

                for c in cards.iter() {
                    self.take_card(c);
                    self.players[p].give_pond(*c);
                }
            }
        };
    }

    pub fn push_move(&mut self, mv: &Move) {
        self.moves.push(mv.clone());
    }

    pub fn check_scopa(&mut self, mv: &Move) {
        match mv {
            Move::Down(_) => {}
            Move::Up(c, _) => {
                if c.val != Value::One {
                    let t = self.turn;
                    self.players[t].score += 1;
                }
            }
        }
    }

    pub fn all_hands_empty(&self) -> bool {
        let mut empty = true;
        for i in 0..self.players.len() {
            empty &= self.players[i].hand_empty();
        }
        empty
    }

    pub fn calculate_scores(&mut self) {
        for i in 0..self.players.len() {
            self.players[i].calculate_score();
        }
    }

    fn recursive_move_helper(
        &self,
        index: usize,
        target: Card,
        table: &Vec<Card>,
        moves: &mut Vec<Move>,
        total: usize,
        current_cards: &mut Vec<Card>,
    ) {
        if index >= table.len() {
            return;
        }
        let target_value = target.val as usize;
        let new_total = total + table[index].val as usize;
        if new_total > target_value {
            return;
        }
        current_cards.push(table[index]);
        if target_value == new_total {
            moves.push(Move::Up(target, current_cards.clone()));
        }
        self.recursive_move_helper(index + 1, target, table, moves, new_total, current_cards);
        current_cards.pop();
        self.recursive_move_helper(index + 1, target, table, moves, total, current_cards);
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let mut table_cards: Vec<Card> = self.table.iter().map(|&c| return c).collect();
        table_cards.sort();
        let mut moves = Vec::new();

        for card in self.players[self.turn].hand.iter() {
            if self.ace_sweeps && card.val == Value::One {
                moves.push(Move::Up(*card, table_cards.clone()));
                continue;
            }
            // if no cards on the table then only down is possible
            if self.table.len() == 0 {
                moves.push(Move::Down(*card));
                continue;
            }

            // find if a pickup is possible
            let mut mvs = Vec::new();
            let mut current_cards = vec![];
            self.recursive_move_helper(0, *card, &table_cards, &mut mvs, 0, &mut current_cards);
            if mvs.len() > 0 {
                moves.append(&mut mvs);
                continue;
            }

            // if no pickup is possible
            moves.push(Move::Down(*card));
        }

        return moves;
    }

    pub fn over(&self) -> bool {
        let mut over = true;
        for i in 0..self.players.len() {
            over &= self.players[i].hand.len() == 0;
        }
        return over && self.deck.len() == 0;
    }
}

#[derive(Clone, Debug)]
pub enum Move {
    Down(Card),
    Up(Card, Vec<Card>),
}

impl Move {
    fn equal(&self, m: &Self) -> bool {
        // TODO: This is so slow so need to optimise
        match (self, m) {
            (Move::Down(c1), Move::Down(c2)) => c1 == c2,
            (Move::Up(c1, cds1), Move::Up(c2, cds2)) => {
                if c1 != c2 {
                    return false;
                }
                let mut cds1_ = cds1.clone();
                let mut cds2_ = cds2.clone();
                cds1_.sort();
                cds2_.sort();
                cds1 == cds2
            }
            (_, _) => false,
        }
    }
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
            let c = match iter.next() {
                Some(b) => Card::parse(b)?,
                _ => return Err(format!("Need a first argument for up")),
            };
            let mut v = vec![];
            for i in iter {
                // could have a wildcard for all ?
                let card = Card::parse(i)?;
                v.push(card);
            }
            v.sort();
            return Ok(Move::Up(c, v));
        }
    }
    return Err(format!("Could not parse input"));
}

fn main() {
    // create a game
    // TODO: Might have to change this to round and make game first to 11
    let mut game = Game::new();
    game.shuffle();
    game.init_table();
    game.deal_users();

    loop {
        game.debug_state(false);
        if game.over() {
            break;
        }
        if game.all_hands_empty() {
            game.deal_users();
            continue; // to show cards
        }

        let mv = match get_input() {
            Ok(c) => c,
            Err(e) => {
                println!("Err: {}", e);
                continue;
            }
        };
        let mvs = game.generate_moves();
        let mut valid_move = false;
        for i in mvs {
            if mv.equal(&i) {
                valid_move = true;
                break;
            }
        }
        if !valid_move {
            println!("Invalid move: {:?}", mv);
            continue;
        }

        game.do_move(&mv);
        game.push_move(&mv);
        game.check_scopa(&mv);

        // after everything is done, switch to the other player's turn and continue
        game.next_turn();
    }
    // score tally
}

#[cfg(test)]
mod tests {
    use crate::Card;
    use crate::Game;
    use crate::Suit;
    use crate::Value;

    #[test]
    fn test_move_gen() {
        let mut game = Game::new();
        game.table.insert(Card::new(Value::Five, Suit::Clubs));
        game.table.insert(Card::new(Value::Five, Suit::Spades));
        game.table.insert(Card::new(Value::King, Suit::Spades));
        game.table.insert(Card::new(Value::Seven, Suit::Spades));

        game.players[0]
            .hand
            .insert(Card::new(Value::King, Suit::Diamonds));
        game.players[0]
            .hand
            .insert(Card::new(Value::Five, Suit::Diamonds));

        let moves = game.generate_moves();
        assert_eq!(4, moves.len());
    }

    #[test]
    fn test_move_gen2() {
        let mut game = Game::new();
        game.table.insert(Card::new(Value::Three, Suit::Clubs));
        game.table.insert(Card::new(Value::Three, Suit::Spades));
        game.table.insert(Card::new(Value::Four, Suit::Spades));
        game.table.insert(Card::new(Value::Seven, Suit::Spades));

        game.players[0]
            .hand
            .insert(Card::new(Value::King, Suit::Diamonds));

        let moves = game.generate_moves();
        assert_eq!(3, moves.len());
    }
}
