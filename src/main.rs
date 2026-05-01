use scopa_fish::game;
use std::io;
use std::io::prelude::*;

fn get_input() -> Result<game::Move, String> {
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
                Some(b) => game::Card::parse(b)?,
                _ => return Err(format!("Need a second argument for down")),
            };
            return Ok(game::Move::Down(c));
            // TODO(all): Figure this out
            // could just ignore any garbage after
            // or we could interpret it as a second move
            // if iter.next().is_some() {
            //     return Err(format!("Too many arguments {}", c2));
            // }
        } else {
            let c = match iter.next() {
                Some(b) => game::Card::parse(b)?,
                _ => return Err(format!("Need a first argument for up")),
            };
            let mut v = vec![];
            for i in iter {
                // could have a wildcard for all ?
                let card = game::Card::parse(i)?;
                v.push(card);
            }
            return Ok(game::Move::new_up(c, v));
        }
    }
    return Err(format!("Could not parse input"));
}

fn main() {
    // create a game
    // TODO: Might have to change this to round and make game first to 11
    let mut game = game::Game::new();
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

        game.push_move(&mv);
        game.do_move(&mv);
        game.check_scopa(&mv);

        // after everything is done, switch to the other player's turn and continue
        game.next_turn();
    }
    game.debug_state(true);
    // score tally
}
