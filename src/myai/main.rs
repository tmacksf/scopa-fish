#![recursion_limit = "256"]
// use burn::backend::wgpu;
// use rand::prelude::*;
// use burn::train::{metric, train};
use scopa_fish::game;

use crate::{mcts::*, nn::ModelConfig};
mod mcts;
mod nn;
mod training;

fn main() {
    let device = Default::default();
    let model = ModelConfig::new().init::<NNBackend>(&device);
    println!("{model}");
    let mut game = game::Game::new();
    game.init();
    game.debug_state(true);
    let mut t = training::MctsTraining::new(game.clone(), &model);
    t.do_single_game(game);
}
