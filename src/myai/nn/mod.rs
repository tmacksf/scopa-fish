use burn::{
    config::Config,
    data::dataloader::batcher::Batcher,
    nn::{Linear, LinearConfig, Relu, Tanh},
    prelude::*,
    tensor::{activation::log_softmax, backend::AutodiffBackend},
    train::{
        ItemLazy, TrainOutput, TrainStep,
        metric::{Adaptor, LossInput},
    },
};
use scopa_fish::game;
use scopa_fish::game::{Card, Game};
use std::marker::PhantomData;

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
const NUM_ADDITIONAL: usize = 3;

pub fn encode_tensor(g: &Game) -> Vec<f32> {
    let mut out = vec![0.0; NUM_CARD_SLICES * game::Card::NUM_CARDS + NUM_ADDITIONAL];

    let cds = Card::all_cards();
    let turn = g.turn;
    for card in cds.iter() {
        // 1: cards on table
        out[card.num()] = if g.table.contains(card) { 1.0 } else { 0.0 };
        // 2: cards in hand
        out[card.num() + 40] = if g.players[turn].hand.contains(card) {
            1.0
        } else {
            0.0
        };
        // 3: cards that have been picked up
        out[card.num() + 80] = if g.players[turn].pond.contains(card) {
            1.0
        } else {
            0.0
        };
        // 4: cards that have been picked up by opponent
        out[card.num() + 120] = if g.players[(turn + 1) % 2].pond.contains(card) {
            1.0
        } else {
            0.0
        };
        // do a bunch of things
    }

    // Do not need turn because it is always the one doing the turn
    //out[160] = g.turn as f32;

    // scopa count
    out[160] = g.players[g.turn].score as f32 / 21.0;
    out[161] = g.players[(g.turn + 1) % 2].score as f32 / 21.0;
    out[162] = if g.last_pickup == turn { 1.0 } else { 0.0 };
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

const INPUT_LAYER: usize = NUM_CARD_SLICES * game::Card::NUM_CARDS + NUM_ADDITIONAL;
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

pub struct ModelOutput<B: Backend> {
    pub policy: Tensor<B, 2>,
    pub value: Tensor<B, 2>,
}

/*
impl<B: Backend> Adaptor<LossInput<B>> for Output<B> {
    fn adapt(&self) -> LossInput<B> {
        LossInput::new(self.loss.clone())
    }
}
*/

impl<B: Backend> Model<B> {
    // Input -> defined above in encode_tensor
    // Output -> Cards to put down with probabilities and an overall score
    pub fn forward(&self, state: Tensor<B, 2>) -> ModelOutput<B> {
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
        ModelOutput {
            value: v,
            policy: p,
        }
    }

    pub fn infer(&self, state: Vec<f32>, device: &B::Device) -> (f32, Vec<f32>) {
        let state_1d = Tensor::<B, 1>::from_data(TensorData::new(state, [INPUT_LAYER]), device);
        let state_2d = state_1d.reshape([1, INPUT_LAYER]);

        let out = self.forward(state_2d);
        let value: f32 = out.value.into_scalar().to_f32();
        let policy = out
            .policy
            .into_data()
            .to_vec()
            .expect("Cannot unwrap tensor");

        (value, policy)
    }

    pub fn loss(
        policy_pred: Tensor<B, 2>,
        target_policy: Tensor<B, 2>,
        value_pred: Tensor<B, 2>,
        target_value: Tensor<B, 2>,
    ) -> Tensor<B, 1> {
        // Get the MSE of the value loss
        let value_loss = value_pred.sub(target_value).powf_scalar(2.0).mean();

        // cross entropy with soft targets
        let log_softmax_policy = log_softmax(policy_pred, 1);
        let policy_loss = target_policy
            .mul(log_softmax_policy)
            .sum_dim(1)
            .mean()
            .neg();
        value_loss.add(policy_loss)
    }
}

pub struct Batch<B: Backend> {
    pub state: Tensor<B, 2>,
    pub target_policies: Tensor<B, 2>,
    pub target_values: Tensor<B, 2>,
}

pub struct TrainingOutput<B: Backend> {
    pub loss: Tensor<B, 1>,
    pub model_output: ModelOutput<B>,
}

impl<B: Backend> ItemLazy for TrainingOutput<B> {
    type ItemSync = TrainingOutput<B>;

    fn sync(self) -> Self::ItemSync {
        self
    }
}

impl<B: Backend> Adaptor<LossInput<B>> for TrainingOutput<B> {
    fn adapt(&self) -> LossInput<B> {
        LossInput::new(self.loss.clone())
    }
}

// impl<B: AutodiffBackend> TrainStep<Batch<B>, TrainingOutput<B>> for Model<B> {
impl<B: AutodiffBackend> TrainStep for Model<B> {
    type Input = Batch<B>;
    type Output = TrainingOutput<B>;

    fn step(&self, batch: Batch<B>) -> TrainOutput<TrainingOutput<B>> {
        let inference = self.forward(batch.state);

        let loss = Self::loss(
            inference.policy.clone(),
            batch.target_policies,
            inference.value.clone(),
            batch.target_values,
        );
        // inference.loss = Tensor::new(loss);

        let grads = loss.backward();

        TrainOutput::new(
            &loss,
            grads,
            TrainingOutput {
                loss: loss.clone(),
                model_output: inference,
            },
        )
    }
}

pub struct TrainingSample {
    pub state_tensor: Vec<f32>,
    pub target_policy: [f32; 40],
    pub target_value: f32,
}

impl TrainingSample {
    pub fn new(state_tensor: Vec<f32>, target_policy: [f32; 40]) -> TrainingSample {
        TrainingSample {
            state_tensor,
            target_policy,
            target_value: 0.0,
        }
    }
}

// Actual batching and training
pub struct ScopaBatcher<B: Backend> {
    phantom: PhantomData<B>,
}

impl<B: Backend> Batcher<B, TrainingSample, Batch<B>> for ScopaBatcher<B> {
    fn batch(&self, items: Vec<TrainingSample>, device: &B::Device) -> Batch<B> {
        let batch_size = items.len();

        let mut states_raw = Vec::with_capacity(batch_size * INPUT_LAYER);
        let mut policies_raw = Vec::with_capacity(batch_size * OUTPUT_POLICY);
        let mut values_raw = Vec::with_capacity(batch_size * 1);

        for item in items {
            states_raw.extend_from_slice(&item.state_tensor);
            policies_raw.extend_from_slice(&item.target_policy);
            values_raw.push(item.target_value);
        }

        let states = Tensor::<B, 1>::from_floats(states_raw.as_slice(), device)
            .reshape([batch_size, INPUT_LAYER]);
        let policies = Tensor::<B, 1>::from_floats(policies_raw.as_slice(), device)
            .reshape([batch_size, OUTPUT_POLICY]);
        let values =
            Tensor::<B, 1>::from_floats(values_raw.as_slice(), device).reshape([batch_size, 1]);

        Batch {
            state: states,
            target_policies: policies,
            target_values: values,
        }
    }
}
