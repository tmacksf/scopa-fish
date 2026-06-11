#![recursion_limit = "256"]
// use burn::backend::wgpu;
// use rand::prelude::*;
use scopa_fish::game::{self, Game};

use crate::{mcts::*, nn::ModelConfig};
mod mcts;
mod nn;

fn main() {
    let device = Default::default();
    let model = ModelConfig::new().init::<NNBackend>(&device);

    let mut ai = MctsNn::new(game::Game::new(), 0, 8000, &model);
    let _ = ai.find_move();

    println!("{model}");
    println!("What");
}
