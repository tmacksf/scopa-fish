#![recursion_limit = "256"]
use crate::{mcts::*, nn::ModelConfig};
mod mcts;
mod nn;
mod training;

fn main() {
    let device = Default::default();
    let model = ModelConfig::new().init::<NNBackend>(&device);
    println!("{model}");
    // spin up a bunch of producer threads
    // main function is a consumer thread
    let mut t = training::MctsTraining::new();
    t.do_single_game(&model);
}
