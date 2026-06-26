use crate::mcts::{MctsNn, NNBackend};
use crate::nn;
use scopa_fish::game;

pub struct MctsTraining<'a> {
    ai1: MctsNn<'a>,
    ai2: MctsNn<'a>,
}

impl<'a> MctsTraining<'a> {
    pub fn new(game: game::Game, nn: &'a nn::Model<NNBackend>) -> MctsTraining<'a> {
        MctsTraining {
            ai1: MctsNn::new(game.clone(), 1000, &nn),
            ai2: MctsNn::new(game.clone(), 1000, &nn),
        }
    }

    pub fn do_single_game(&mut self, mut master_game: game::Game) {
        // ai1 gets id 0
        // ai2 gets id 1
        let mut ai1_data: Vec<nn::TrainingSample> = Vec::with_capacity(20);
        let mut ai2_data: Vec<nn::TrainingSample> = Vec::with_capacity(20);
        while !master_game.over() {
            let mv: game::Move = if master_game.turn == 0 {
                self.ai1.game = master_game.clone();
                let (mv, s) = self.ai1.find_move();
                ai1_data.push(s);
                mv
            } else {
                self.ai2.game = master_game.clone();
                let (mv, s) = self.ai2.find_move();
                ai2_data.push(s);
                mv
                // ai2 turn
            };
            mv.print();
            master_game.debug_state(false);
            master_game.do_full_move(&mv);
            if master_game.all_hands_empty() && !master_game.over() {
                master_game.deal_users();
            }
        }
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

        master_game.debug_state(true);
    }
}
