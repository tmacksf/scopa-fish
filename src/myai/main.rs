#![recursion_limit = "256"]
use std::sync::mpsc;

use crate::nn::{Model, ModelConfig};
use burn::backend::Wgpu;
use burn::module::AutodiffModule;
use burn_autodiff;

mod mcts;
mod nn;
mod training;

fn main() {
    type Be = burn_autodiff::Autodiff<Wgpu<f32, i32>>;

    let device = Default::default();
    let artifact_dir = "./scopa_ai_artifacts";

    let training_model: &'static Model<Be> =
        Box::leak(Box::new(ModelConfig::new().init::<Be>(&device)));
    println!("{training_model}");
    let (send, recv) = mpsc::channel();
    nn::run_inference_thread(Box::leak(Box::new(training_model.valid())), recv);

    let mut t = training::MctsTraining::new(send, format!("{artifact_dir}"));

    for iteration in 1..=50 {
        println!("Iteration {}", iteration);

        let vals = t.alphazero_iteration();
        t.train();

        println!("Iteration {} done.", iteration)
    }
}
