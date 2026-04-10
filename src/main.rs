pub enum Suit {
    S,
    C,
    D,
    H,
}

pub enum Val {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    J,
    Q,
    K,
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
