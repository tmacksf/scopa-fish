use burn::{
    config::Config,
    nn::{Linear, LinearConfig, Relu, Tanh},
    prelude::*,
};
use scopa_fish::game;
use scopa_fish::game::{Card, Game};

// The neural network is based on the following:
// Input:
// - cards on table
// - cards in hand
// - cards that have been picked up by me
// - cards that have been picked up by opponent
// All of these will look like [1, 0, 1, 1, 0, ...] meaning a card exists at that point in the deck
// Additional data (normalised):
// - turn
// - my scopa count
// - their scopa count

const NUM_CARD_SLICES: usize = 4;
const NUM_ADDITIONAL: usize = 4;

pub fn encode_tensor(g: &Game) -> Vec<f32> {
    let mut out = vec![0.0; NUM_CARD_SLICES * game::NUM_CARDS + NUM_ADDITIONAL];

    let cds = Card::all_cards();
    let turn = g.turn;
    for i in 0..cds.len() {
        // 1: cards on table
        out[i] = if g.table.contains(&cds[i]) { 1.0 } else { 0.0 };
        // 2: cards in hand
        out[i + 40] = if g.players[turn].hand.contains(&cds[i]) {
            1.0
        } else {
            0.0
        };
        // 3: cards that have been picked up
        out[i + 80] = if g.players[turn].pond.contains(&cds[i]) {
            1.0
        } else {
            0.0
        };
        // 4: cards that have been picked up by opponent
        out[i + 120] = if g.players[(turn + 1) % 2].pond.contains(&cds[i]) {
            1.0
        } else {
            0.0
        };
        // do a bunch of things
    }

    // turn
    out[160] = g.turn as f32;
    // scopa count
    out[161] = g.players[g.turn].score as f32 / 21.0;
    out[162] = g.players[(g.turn + 1) % 2].score as f32 / 21.0;
    out[163] = g.last_pickup as f32;
    out
}

// Output: 40 cards
// - Output will determine which card to play.
// - Heuristic will determine which card to pick up if possible.

// Model design:
// - A two headed network (similar to alpha zero approach)
// - a linear MLP. Input is a single large 1d tensor
//
// Possible extension:
// - reward shaping: adding an immediate reward to the nn's eval score (+1 for scopa, +.25 for 7,
//   etc)
// -

const INPUT_LAYER: usize = NUM_CARD_SLICES * game::NUM_CARDS + NUM_ADDITIONAL;
const HIDDEN_SIZE_1: usize = 128;
const HIDDEN_SIZE_2: usize = 256;
const HIDDEN_SIZE_3: usize = 512;
const OUTPUT_POLICY: usize = 40;

#[derive(Config, Debug)]
pub struct ModelConfig {}

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model {
            linear1: LinearConfig::new(INPUT_LAYER, HIDDEN_SIZE_2).init(device),
            linear2: LinearConfig::new(HIDDEN_SIZE_2, HIDDEN_SIZE_2).init(device),
            linear3: LinearConfig::new(HIDDEN_SIZE_2, HIDDEN_SIZE_3).init(device),
            linear4: LinearConfig::new(HIDDEN_SIZE_3, HIDDEN_SIZE_2).init(device),

            policy_fc1: LinearConfig::new(HIDDEN_SIZE_2, HIDDEN_SIZE_1).init(device),
            policy_out: LinearConfig::new(HIDDEN_SIZE_1, OUTPUT_POLICY).init(device),

            value_fc1: LinearConfig::new(HIDDEN_SIZE_2, HIDDEN_SIZE_1).init(device),
            value_out: LinearConfig::new(HIDDEN_SIZE_1, 1).init(device),

            activation: Relu::new(),
            normal: Tanh::new(),
        }
    }
}

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    linear3: Linear<B>,
    linear4: Linear<B>,

    // policy head
    policy_fc1: Linear<B>,
    policy_out: Linear<B>,

    // value head
    value_fc1: Linear<B>,
    value_out: Linear<B>,

    activation: Relu,
    normal: Tanh,
}

impl<B: Backend> Model<B> {
    // Input -> defined above in encode_tensor
    // Output -> Cards to put down with probabilities and an overall score
    pub fn forward(&self, state: Tensor<B, 2>) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let x = self.linear1.forward(state);
        let x = self.activation.forward(x);

        let x = self.linear2.forward(x);
        let x = self.activation.forward(x);

        let x = self.linear3.forward(x);
        let x = self.activation.forward(x);

        let x = self.linear4.forward(x);
        let x = self.activation.forward(x);

        let p = self.policy_fc1.forward(x.clone());
        let p = self.activation.forward(p);
        let p = self.policy_out.forward(p);

        let v = self.value_fc1.forward(x);
        let v = self.activation.forward(v);
        let v = self.value_out.forward(v);
        let v = self.normal.forward(v);
        (v, p)
    }

    pub fn infer(&self, state: Vec<f32>, device: &B::Device) -> (f32, Vec<f32>) {
        let state_1d = Tensor::<B, 1>::from_data(TensorData::new(state, [INPUT_LAYER]), device);
        let state_2d = state_1d.reshape([1, INPUT_LAYER]);

        let (v_tensor, p_tensor) = self.forward(state_2d);
        let value: f32 = v_tensor.into_scalar().to_f32();
        let policy = p_tensor.into_data().to_vec().expect("Cannot unwrap tensor");

        (value, policy)
    }
}
