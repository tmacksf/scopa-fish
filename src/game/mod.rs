use rand::prelude::*;
use rand::rng;
use std::cmp::min;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::ops::Add;

// TODOS:
// Proper error handling (replace strings in Result with errors) - Tommy

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Suit {
    Spades = 1,
    Clubs = 2,
    Diamonds = 3,
    Hearts = 4,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Diamonds, Suit::Hearts, Suit::Spades, Suit::Clubs];

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

    pub fn decode_bitmask(mut val: u8) -> Self {
        let mask = 0b00000011;
        val = val & mask;
        match val {
            0b00000000 => Suit::Spades,
            0b00000001 => Suit::Clubs,
            0b00000010 => Suit::Diamonds,
            0b00000011 => Suit::Hearts,
            _ => panic!("Could not decode other suit: {}", val),
        }
    }

    pub fn from_num(val: usize) -> Suit {
        match val {
            0 => Suit::Spades,
            1 => Suit::Clubs,
            2 => Suit::Diamonds,
            3 => Suit::Hearts,
            _ => panic!("Cannot decode num: {}", val),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let out = match self {
            Self::Spades => "Spades",
            Self::Clubs => "Clubs",
            Self::Diamonds => "Diamonds",
            Self::Hearts => "Hearts",
        };
        write!(f, "{}", out)
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
    pub const ALL: [Value; 10] = [
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
    ];

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
            Value::One => 0b00000100,
            Value::Two => 0b00001000,
            Value::Three => 0b00001100,
            Value::Four => 0b00010000,
            Value::Five => 0b00010100,
            Value::Six => 0b00011000,
            Value::Seven => 0b00011100,
            Value::Jack => 0b00100000,
            Value::Queen => 0b00100100,
            Value::King => 0b00101100,
        }
    }

    pub fn decode_bitmask(mut val: u8) -> Self {
        let mask = 0b00111100;
        val = val & mask;
        match val {
            0b00000100 => Value::One,
            0b00001000 => Value::Two,
            0b00001100 => Value::Three,
            0b00010000 => Value::Four,
            0b00010100 => Value::Five,
            0b00011000 => Value::Six,
            0b00011100 => Value::Seven,
            0b00100000 => Value::Jack,
            0b00100100 => Value::Queen,
            0b00101100 => Value::King,
            _ => panic!("Could not decode other value: {}", val),
        }
    }

    pub fn from_num(val: usize) -> Value {
        match val {
            0 => Value::One,
            1 => Value::Two,
            2 => Value::Three,
            3 => Value::Four,
            4 => Value::Five,
            5 => Value::Six,
            6 => Value::Seven,
            7 => Value::Jack,
            8 => Value::Queen,
            9 => Value::King,
            _ => panic!("Cannot decode value: {}", val),
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let out = match self {
            Self::One => "Ace",
            Self::Two => "Two",
            Self::Three => "Three",
            Self::Four => "Four",
            Self::Five => "Five",
            Self::Six => "Six",
            Self::Seven => "Seven",
            Self::Jack => "Jack",
            Self::Queen => "Queen",
            Self::King => "King",
        };
        write!(f, "{}", out)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Card {
    pub val: Value,
    pub suit: Suit,
}

// Every card can be stored in 6 bits meaning a full 40 card deck needs
// 6*40 bits -> 240 bits or 24 (will use 32) bytes
impl Card {
    pub const NUM_CARDS: usize = 40;

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

    fn bitmask(&self) -> u8 {
        self.val.bitmask() | self.suit.bitmask()
    }

    fn decode_bitmask(val: u8) -> Card {
        let v = Value::decode_bitmask(val);
        let s = Suit::decode_bitmask(val);
        Card::new(v, s)
    }

    pub fn encode_vec(v: &Vec<Card>) -> (u128, u128) {
        // Encode cds1 first then cds2
        let mut shift: usize = 0;
        let mut cds1: u128 = 0;
        let mut cds2: u128 = 0;
        for c in v {
            // 6 * 21 = 126 (once 21 cards have been inserted we have to deal with overflow)
            if shift < 126 {
                cds1 |= (c.bitmask() as u128) << shift;
            } else {
                let sh = shift % 126;
                cds2 |= (c.bitmask() as u128) << sh;
            }
            shift += 6;
        }
        (cds1, cds2)
    }

    fn decode(mut cds1: u128, mut cds2: u128) -> Vec<Card> {
        let mut res = vec![];
        if cds1 == 0 {
            return res;
        }
        let mask: u8 = 0b00111111;
        while cds1 != 0 {
            res.push(Card::decode_bitmask((cds1 as u8) & mask));
            cds1 = cds1 >> 6;
        }
        while cds2 != 0 {
            res.push(Card::decode_bitmask((cds2 as u8) & mask));
            cds2 = cds2 >> 6;
        }
        return res;
    }

    pub fn all_cards() -> Vec<Card> {
        let mut vals: Vec<Card> = Suit::ALL
            .iter()
            .flat_map(|suit| {
                Value::ALL.iter().map(|val| Card {
                    suit: *suit,
                    val: *val,
                })
            })
            .collect();
        vals.sort_by(|c1, c2| c1.num().cmp(&c2.num()));
        vals
    }

    pub fn num(&self) -> usize {
        (self.suit as usize - 1) * 10 + (self.val as usize - 1)
    }

    pub fn from_num(c: usize) -> Card {
        Card {
            val: Value::from_num(c % 10),
            suit: Suit::from_num(c / 10),
        }
    }

    pub fn heuristic(&self) -> f32 {
        let mut total = match self.val {
            Value::Seven => {
                1.0 / 4.0
                    + (if self.suit == Suit::Diamonds {
                        1.0
                    } else {
                        0.0
                    })
            }
            Value::Six => 1.0 / 8.0,
            Value::Five => 1.0 / 16.0,
            _ => 0.0,
        };

        total += match self.suit {
            Suit::Diamonds => 1.0 / 10.0,
            _ => 0.0,
        };

        total
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({} of {})", self.val, self.suit)
    }
}

#[derive(Clone)]
pub struct Player {
    pub hand: HashSet<Card>,
    pub pond: Vec<Card>,
    pub score: u16,
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

    pub fn remove_card(&mut self, c: Card) {
        match self.hand.remove(&c) {
            true => {}
            false => panic!("Card not present in players hand: {}", c),
        };
    }

    pub fn find_card(&self, v: Option<Value>, s: Option<Suit>) -> usize {
        let mut count = 0;

        for c in &self.pond {
            let v_match = match v {
                Some(v) => c.val == v,
                None => true,
            };
            let s_match = match s {
                Some(s) => c.suit == s,
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
    pub fn calculate_score(&mut self) -> u16 {
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
        let values_to_check = [
            Value::Seven,
            Value::Six,
            Value::Five,
            Value::Four,
            Value::Three,
            Value::Two,
            Value::One,
        ];
        for v in values_to_check {
            let count = self.find_card(Some(v), None);
            if count > 2 {
                self.score += 1;
                break;
            } else if count < 2 {
                break;
            }
        }
        return self.score;
    }
}

#[derive(Clone)]
pub struct Game {
    pub players: Vec<Player>,
    pub turn: usize, // turn corresponds to the index of a player in the players vec
    pub table: HashSet<Card>,
    deck: Vec<Card>,
    pub moves: Vec<Move>,
    ace_sweeps: bool,
    pub last_pickup: usize,
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
            last_pickup: 0,
        };
        g.new_deck();
        return g;
    }

    pub fn init(&mut self) {
        self.shuffle();
        self.init_table();
        self.deal_users();
    }

    fn new_deck(&mut self) {
        for v in Value::ALL.iter() {
            for s in &Suit::ALL {
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
            for p in &mut self.players {
                p.give_card(self.deck.pop().unwrap());
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

    pub fn last_move(&self) -> Option<Move> {
        match self.moves.last() {
            None => None,
            Some(m) => Some(*m),
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

    pub fn do_full_move(&mut self, mv: &Move) {
        self.push_move(mv);
        self.do_move(mv);
        self.check_scopa(mv);
        self.next_turn();
    }

    pub fn do_move(&mut self, mv: &Move) {
        // This assumes a move has been checked to be valid.
        // if the move is not valid the program will panic
        let p = self.turn;
        match mv {
            Move::Down(c) => {
                self.players[p].remove_card(*c);
                self.play_card(*c);
            }
            Move::Up(c, cs) => {
                self.last_pickup = self.turn;
                self.players[p].remove_card(*c);
                self.players[p].give_pond(*c);

                let cards = Card::decode(*cs, 0);
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
                if c.val != Value::One && self.table.len() == 0 {
                    let t = self.turn;
                    self.players[t].score += 1;
                }
            }
        }
    }

    pub fn all_hands_empty(&self) -> bool {
        self.players.iter().fold(true, |e, p| p.hand_empty() && e)
    }

    pub fn calculate_scores(&mut self) {
        for p in &mut self.players {
            p.calculate_score();
        }
    }

    // TODO: Adapt this for more players in the future
    pub fn calculate_win_for_current_player(&mut self) -> i8 {
        let p_id = self.turn;
        let other = (self.turn + 1) % 2;

        let mut ps = [0; 2];
        for (i, p) in self.players.iter_mut().enumerate() {
            ps[i] = p.calculate_score();
        }

        if ps[p_id] == ps[other] {
            0
        } else if ps[p_id] > ps[other] {
            1
        } else {
            -1
        }
    }

    fn recursive_move_helper(
        &self,
        index: usize,
        target: Card,
        table: &Vec<Card>,
        moves: &mut Vec<Vec<Card>>,
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
            moves.push(current_cards.clone());
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
                moves.push(Move::new_up(*card, table_cards.clone()));
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
                // find the moves that contain the least amount of cards
                let mut min_moves = 100;
                for mv in 0..mvs.len() {
                    min_moves = min(min_moves, mvs[mv].len());
                }
                // only append moves with the min count
                for mv in 0..mvs.len() {
                    if mvs[mv].len() == min_moves {
                        moves.push(Move::new_up(*card, mvs[mv].clone()));
                    }
                }
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

    pub fn summary(&self) {
        println!("Player 0: {}", self.players[0].score);
        println!("Player 1: {}", self.players[1].score);

        for mv in &self.moves {
            mv.print();
        }
    }

    pub fn end(&mut self) {
        let mut cards: [Option<Card>; 40] = [None; 40];
        for (i, c) in self.table.iter().enumerate() {
            cards[i] = Some(*c);
        }

        for c in cards {
            match c {
                Some(card) => {
                    self.take_card(&card);
                    let last_pickup = self.last_pickup;
                    self.players[last_pickup].give_pond(card);
                }
                None => break,
            }
        }
    }

    // pub fn rollback_to_move(&mut self, mv: Option<&Move>) {
    //     for i in 0..self.moves.len() {
    //         match mv {
    //             Some(m) => {
    //                 if *m == self.moves[i] {
    //                     return;
    //                 }
    //             }
    //             None => {}
    //         }
    //         match self.moves[i] {
    //             Move::Down(c) => {
    //                 self.table.remove(&c);
    //                 self.players[self.turn].give_card(c);
    //             }
    //             Move::Up(c, cds1, cds2) => {}
    //         }
    //     }
    // }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Move {
    Down(Card),
    // Up(Card, Vec<Card>),
    // TODO(all): Calculate the maximum possible number of pickups (i think its 12 or 13 without ace sweep)
    // so probably don't need the insane amount of storage and can cut this down further to 12 * 6
    // (72 bits) or 13 & 6 (78 bits).
    Up(Card, u128),
}

impl Move {
    pub fn new_up(c: Card, mut cds: Vec<Card>) -> Move {
        cds.sort_unstable();
        // after the sort we encode the cards
        let (cds1, cds2) = Card::encode_vec(&cds);
        if cds2 > 0 {
            panic!(
                "The impossible has happened: Vec {:?}, cds1: {}, cds2: {}",
                cds, cds1, cds2
            );
        }
        Move::Up(c, cds1)
    }

    // moves are sorted when created so need to use the new_up function for this one to work
    pub fn equal(&self, m: &Self) -> bool {
        match (self, m) {
            (Move::Down(c1), Move::Down(c2)) => c1 == c2,
            (Move::Up(c1, cds11), Move::Up(c2, cds21)) => c1 == c2 && cds11 == cds21,
            (_, _) => false,
        }
    }

    pub fn print(&self) {
        match self {
            Move::Down(c) => println!("Down: {}", c),
            Move::Up(c, cds1) => {
                let mvs = Card::decode(*cds1, 0);
                print!("Up: {},", c);
                for mv in &mvs {
                    print!(" {}, ", mv);
                }
                println!();
            }
        }
    }

    pub fn get_down_card(&self) -> Card {
        match self {
            Move::Down(c) => *c,
            Move::Up(c, _) => *c,
        }
    }

    pub fn heuristic(&self) -> f32 {
        match self {
            Move::Up(c, cs) => {
                let mut total = c.heuristic();
                total += Card::decode(*cs, 0)
                    .iter()
                    .map(|c: &Card| c.heuristic())
                    .sum::<f32>();

                total
            }
            Self::Down(c) => c.heuristic(),
        }
    }
}

pub struct GameInfo {
    pub display_debug: bool,
    pub display_all_debug: bool,
}

pub fn game_loop(info: GameInfo, p1: fn(&Game) -> Move, p2: fn(&Game) -> Move) -> (u16, u16) {
    let mut game = Game::new();
    game.init();
    if info.display_debug {
        game.debug_state(info.display_all_debug);
    }
    loop {
        if game.over() {
            break;
        }
        if game.all_hands_empty() {
            game.deal_users();
            continue; // to show cards
        }

        let mv = if game.turn == 0 { p1(&game) } else { p2(&game) };

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

        game.push_move(&mv);
        game.do_move(&mv);
        game.check_scopa(&mv);

        // after everything is done, switch to the other player's turn and continue
        game.next_turn();
    }

    // don't care about performance here because it's game end
    game.end();

    game.calculate_scores();
    // summarise
    game.summary();
    (game.players[0].score, game.players[1].score)
}

pub fn get_input(game: &Game) -> Move {
    game.debug_state(false);
    'outer: loop {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let l = line.unwrap();
            let mut iter = l.split_whitespace();
            let down = match iter.next() {
                Some("u") | Some("U") => false,
                Some("d") | Some("D") => true,
                _ => {
                    println!("Needs to start with u/U or d/D");
                    continue;
                }
            };

            // down is just (value, suit); up is a list of (value, suits)
            if down {
                let c = match iter.next() {
                    Some(b) => Card::parse(b),
                    _ => {
                        println!("Need a second argument for down");
                        continue;
                    }
                };
                let card = match c {
                    Ok(c) => Move::Down(c),
                    Err(e) => {
                        println!("Could not parse card: {}", e);
                        continue;
                    }
                };
                return card;
            } else {
                let c = match iter.next() {
                    Some(b) => match Card::parse(b) {
                        Ok(c) => c,
                        Err(e) => {
                            println!("Could not parse card {}", e);
                            continue;
                        }
                    },
                    _ => {
                        println!("Need a first argument for up");
                        break;
                    }
                };
                let mut v = vec![];
                for i in iter {
                    // could have a wildcard for all ?
                    let card = Card::parse(i);
                    if card.is_ok() {
                        v.push(card.unwrap());
                    } else {
                        continue 'outer;
                    }
                }
                return Move::new_up(c, v);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{self};

    use crate::game::{Card, Game, Suit, Value};

    #[test]
    fn test_move_gen() {
        let mut game = Game::new();
        game.table.insert(Card::new(Value::Five, Suit::Clubs));
        game.table.insert(Card::new(Value::Five, Suit::Spades));
        game.table.insert(Card::new(Value::King, Suit::Spades));
        game.table.insert(Card::new(Value::King, Suit::Clubs));
        game.table.insert(Card::new(Value::Seven, Suit::Spades));
        game.table.insert(Card::new(Value::Three, Suit::Spades));

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
        game.table.insert(Card::new(Value::Six, Suit::Spades));
        game.table.insert(Card::new(Value::Seven, Suit::Spades));

        game.players[0]
            .hand
            .insert(Card::new(Value::King, Suit::Diamonds));

        let moves = game.generate_moves();
        assert_eq!(3, moves.len());
    }

    // This is basically a fuzzer
    #[test]
    fn encode_decode() {
        let mut game = Game::new();
        game.shuffle();

        let l = rand::random::<u8>() % 40;
        let mut cds = vec![];

        for i in 0..l {
            cds.push(game.deck[i as usize]);
        }

        let (v1, v2) = Card::encode_vec(&cds);
        let cds2 = Card::decode(v1, v2);

        assert_eq!(cds, cds2);
    }

    #[test]
    fn nums() {
        let mut deck = vec![];
        for v in Value::ALL.iter() {
            for s in &Suit::ALL {
                deck.push(Card::new(*v, *s));
            }
        }

        let mut hm: [u8; 40] = [0; 40];
        for i in deck.iter() {
            if hm[i.num()] == 1 {
                dbg!(i.num());
                assert!(false);
            }
            hm[i.num()] = 1;
            assert_eq!(Card::from_num(i.num()), *i);
        }

        let cds = Card::all_cards();
        for (i, c) in cds.iter().enumerate() {
            assert_eq!(c.num(), i)
        }
    }
}
