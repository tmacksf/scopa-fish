pub enum Suit {
    Swords,
    Clubs,
    Diamonds,
    Hearts,
}

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

pub enum Card {
    Suit(Suit),
    Val(Val),
}

pub struct Player {
    Hand: Vec<Card>,
}

fn main() {
    println!("Hello, world!");
}
