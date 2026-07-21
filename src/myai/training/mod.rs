use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use crate::mcts::MctsNn;
use crate::nn::{self, EvalRequest, TrainingSample};
// use burn::tensor::backend::AutodiffBackend;
use scopa_fish::game;

const SIMULATION_COUNT: usize = 1000;
const THREAD_COUNT: usize = 10;
const SAMPLE_COUNT: usize = 100;

pub struct MctsTraining {
    samples: usize,
    artifact_dir: String,
    send: mpsc::Sender<EvalRequest>,
}

impl MctsTraining {
    pub fn new(send: mpsc::Sender<EvalRequest>, artifact_dir: String) -> MctsTraining {
        MctsTraining {
            samples: SAMPLE_COUNT,
            artifact_dir,
            send,
        }
    }

    pub fn do_single_game(send: mpsc::Sender<EvalRequest>) -> Vec<TrainingSample> {
        println!("Starting game, {:?}", thread::current().id());
        let mut master_game = game::Game::new();
        master_game.init();
        let mut ai = MctsNn::new(master_game.clone(), SIMULATION_COUNT, send.clone());

        // ai1 gets id 0
        // ai2 gets id 1
        let mut ai1_data: Vec<TrainingSample> = Vec::with_capacity(20);
        let mut ai2_data: Vec<TrainingSample> = Vec::with_capacity(20);

        println!("Starting game loop, {:?}", thread::current().id());
        while !master_game.over() {
            let mv: game::Move = if master_game.turn == 0 {
                ai.game = master_game.clone();
                let (mv, s) = ai.find_move();
                ai.print_wait_timings();
                ai1_data.push(s);
                mv
            } else {
                ai.game = master_game.clone();
                let (mv, s) = ai.find_move();
                ai.print_wait_timings();
                ai2_data.push(s);
                mv
            };
            // mv.print();
            // master_game.debug_state(false);
            master_game.do_full_move(&mv);
            if master_game.all_hands_empty() && !master_game.over() {
                master_game.deal_users();
            }
        }
        println!("Game done, {:?}", thread::current().id());
        master_game.calculate_scores();
        let ai1_score = master_game.players[0].score;
        let ai2_score = master_game.players[1].score;
        let ai1_score = if ai1_score > ai2_score {
            1.0
        } else if ai2_score > ai1_score {
            -1.0
        } else {
            0.0
        };

        for i in 0..ai1_data.len() {
            ai1_data[i].target_value = ai1_score;
        }
        for i in 0..ai2_data.len() {
            ai2_data[i].target_value = -1.0 * ai1_score;
        }

        // master_game.debug_state(true);

        ai1_data.extend(ai2_data);
        ai1_data
    }

    pub fn alphazero_iteration(&mut self) -> Vec<TrainingSample> {
        let res = Arc::new(Mutex::new(Vec::<nn::TrainingSample>::new()));

        (0..10).for_each(|_| {
            let mut threads = Vec::with_capacity(THREAD_COUNT);
            (0..THREAD_COUNT).for_each(|i| {
                let res_clone = Arc::clone(&res);
                let sender_clone = self.send.clone();
                threads.push(thread::spawn(move || {
                    println!("Starting thread: {:?}", thread::current().id());
                    let sample = { Self::do_single_game(sender_clone) };

                    res_clone.lock().unwrap().extend(sample);
                }));
            });

            // wait on threads then go again
            threads.into_iter().for_each(|thread| {
                thread
                    .join()
                    .expect("The thread creating or execution failed !")
            });
        });
        res.lock().unwrap().iter().cloned().collect()
    }

    pub fn train(&mut self) {
    }
}
