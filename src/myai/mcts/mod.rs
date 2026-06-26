use crate::nn;
use burn::{
    backend::Wgpu,
    module::Module,
    // tensor::{self},
};
use f32;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use scopa_fish::game::{self, Card, Game};

// Have to figure out tree caching
// as there are many possible ways to get to the same states in the tree.
// Need to consider a few things when looking for cache hits as eval is based on past and current
// values
// - cards in each pond
// - cards in each hand
// - cards on table
// - score
// - turn
//
// Could also pivot to a directed graph
// - Alphazero style DAG structure

pub type NNBackend = Wgpu<f32, i32>;

// Nodes for a mcts
#[derive(Debug)]
pub struct Node {
    t: f32,         // total win probability
    t_average: f32, // average win propability
    n: usize,       // number of visits
    mv: Option<game::Move>,
    nodes: Vec<Node>,
    expanded: bool,
    // NN section
    p: f32, // initial guess by nn of the quality of the node
}

impl Node {
    fn _new(mv: game::Move) -> Self {
        Node {
            t: 0.0,
            t_average: 0.0,
            n: 0,
            mv: Some(mv),
            nodes: vec![],
            p: 0.0,
            expanded: false,
        }
    }

    fn new_nn(mv: game::Move, p: f32) -> Self {
        Node {
            t: 0.0,
            t_average: 0.0,
            n: 0,
            mv: Some(mv),
            nodes: vec![],
            expanded: false,
            p,
        }
    }

    fn new_root(nodes: Vec<Node>, t: f32) -> Self {
        Self {
            t,
            t_average: 0.0,
            n: 1,
            mv: None,
            expanded: true,
            p: 0.0,
            nodes,
        }
    }

    fn select_child(&self) -> usize {
        let mut prev = f32::NEG_INFINITY;
        let mut out = 0;
        let parent_n = self.n as f32;
        let parent_n_sqrt = parent_n.sqrt();

        for (i, n) in self.nodes.iter().enumerate() {
            let u = EXPLORATION_PARAMETER * n.p * (parent_n_sqrt / (1.0 + (n.n as f32)));
            let q = -n.t_average; // the win prob here
            let puct = u + q;

            if puct > prev {
                prev = puct;
                out = i;
            }
        }
        out
    }

    fn print(&self, depth: usize) {
        println!(
            "Depth: {}, t: {}, t_avg: {}, n: {}, mv: {:?}, nodes: {}, expanded: {}, p: {}",
            depth,
            self.t,
            self.t_average,
            self.n,
            self.mv,
            self.nodes.len(),
            self.expanded,
            self.p
        );
    }

    fn _print_full(&self, depth: usize) {
        println!(
            "Depth: {}, t: {}, t_avg: {}, n: {}, mv: {:?}, nodes: {}, expanded: {}, p: {}",
            depth,
            self.t,
            self.t_average,
            self.n,
            self.mv,
            self.nodes.len(),
            self.expanded,
            self.p
        );
        for _i in &self.nodes {
            // i.print_full(depth + 1);
        }
    }
}

// pub struct MCTS {
//     player_id: usize,
//     game: Game,
// }
//
// impl MCTS {
//     pub fn new(game: Game, player_id: usize) -> Self {
//         Self { player_id, game }
//     }
//
//     pub fn find_move(&mut self) -> game::Move {
//         let mut nodes = self.generate_nodes();
//
//         if nodes.len() == 1 {
//             return nodes[0].mv.unwrap();
//         }
//         let mut n = 0;
//         for _ in 0..800 {
//             let game = self.game.clone();
//
//             self.selection(n, &mut nodes, 0);
//             self.game = game;
//             n += 1;
//         }
//
//         // select the node with the highest score
//         let mut score = f32::NEG_INFINITY;
//         let mut mv: Option<game::Move> = None;
//         for i in 0..nodes.len() {
//             let t = nodes[i].t_average;
//             if t > score {
//                 mv = nodes[i].mv;
//                 score = t;
//             }
//         }
//         mv.unwrap()
//     }
//
//     fn generate_nodes(&self) -> Vec<Box<Node>> {
//         let mvs = self.game.generate_moves();
//         let mut nodes = Vec::with_capacity(mvs.len());
//         for i in 0..mvs.len() {
//             nodes.push(Box::new(Node::new(mvs[i])));
//         }
//         nodes
//     }
//
//     fn selection(
//         &mut self,
//         mut prev_n: usize,
//         current_nodes: &mut Vec<Box<Node>>,
//         depth: usize,
//     ) -> f32 {
//         if current_nodes.len() == 0 {
//             // got to the end of the game so evalutate and return
//             return self.rollout(depth);
//         }
//
//         let mut highest = f32::NEG_INFINITY;
//         let mut index = 0;
//         let mut found_leaf = false;
//         for i in 0..current_nodes.len() {
//             // Check if leaf node
//             if current_nodes[i].n == 0 {
//                 index = i;
//                 found_leaf = true;
//                 break;
//             }
//             // Calculate UCB1
//             let ucb1 = current_nodes[i].t_average
//                 + EXPLORATION_PARAMETER
//                     * f32::sqrt(f32::ln(prev_n as f32) / current_nodes[i].n as f32);
//             if ucb1 > highest {
//                 index = i;
//                 highest = ucb1;
//                 prev_n = current_nodes[i].n;
//             }
//         }
//
//         self.game.do_full_move(&current_nodes[index].mv.unwrap());
//         let score = if found_leaf {
//             self.rollout(depth)
//         } else if current_nodes[index].nodes.len() == 0 {
//             // can explore
//             self.expansion(&mut current_nodes[index]);
//             self.rollout(depth)
//         } else {
//             self.selection(prev_n, &mut current_nodes[index].nodes, depth + 1)
//         };
//         current_nodes[index].n += 1;
//         current_nodes[index].t += score;
//         current_nodes[index].t_average = score;
//         return score;
//     }
//
//     fn expansion(&mut self, node: &mut Node) {
//         // expansion
//         if node.n == 0 {
//             panic!("Got a n of 0 in expansion");
//         }
//         node.nodes = self.generate_nodes();
//     }
//
//     fn rollout(&mut self, mut depth: usize) -> f32 {
//         let mut rng = rand::rng();
//         while !self.game.over() {
//             if self.game.all_hands_empty() {
//                 self.game.deal_users();
//             }
//             let mvs = self.game.generate_moves();
//             let mv = mvs.choose(&mut rng).unwrap();
//             self.game.do_full_move(&mv);
//         }
//         let s1 = self.game.players[self.player_id].calculate_score();
//         let s2 = self.game.players[(self.player_id + 1) % 2].calculate_score();
//         let res = s1 as f32 - s2 as f32;
//         if depth == 0 {
//             depth = 1;
//         }
//         // divide by depth to not get stuck
//         if depth % 2 == 1 {
//             // res / depth as f32
//             res
//         } else {
//             // (res * -0.1) / depth as f32
//             res * -0.1
//         }
//     }
// }

pub struct MctsNn<'a> {
    pub game: Game,
    simulation_count: usize,
    nn: &'a nn::Model<NNBackend>,
}

// SQRT(2)
// const EXPLORATION_PARAMETER: f32 = 1.41421356237;
const EXPLORATION_PARAMETER: f32 = 2.5; // (Silver et al., 2017) 

impl<'a> MctsNn<'a> {
    pub fn new(game: Game, simulation_count: usize, nn: &'a nn::Model<NNBackend>) -> Self {
        Self {
            game,
            nn,
            simulation_count,
        }
    }

    pub fn find_move(&mut self) -> (game::Move, nn::TrainingSample) {
        let tensor = nn::encode_tensor(&self.game);
        let (s, nodes) = self.expand();
        let mut root = Node::new_root(nodes, s);

        root = self.selection(root);
        // TODO: during actual gameplay select the node with the highest visit count

        let mut weights: Vec<f32> = Vec::with_capacity(root.nodes.len());
        let mut target_policy: [f32; 40] = [0.0; 40];

        for n in &root.nodes {
            if n.n > 0 {
                let num = match n.mv {
                    None => {
                        panic!("None move with n > 0");
                    }
                    Some(m) => match m {
                        game::Move::Down(c) => c.num(),
                        game::Move::Up(c, _) => c.num(),
                    },
                };
                let weight = (n.n as f32) / self.simulation_count as f32;
                weights.push(weight);
                target_policy[num] = weight;
            }
        }
        let training_data = nn::TrainingSample::new(tensor, target_policy);
        if weights.len() == 0 {
            self.game.debug_state(true);
            panic!("No move!");
        } else if weights.len() == 1 {
            return (root.nodes[0].mv.unwrap(), training_data);
        }

        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::rng();
        let chosen = root.nodes[dist.sample(&mut rng)].mv.unwrap();

        (chosen, training_data)
    }

    // Neural network eval
    fn selection(&mut self, mut root: Node) -> Node {
        // This works differently here:
        // 1. Evaluate to find moves
        // 2. Do PUCT on the moves then expand based on that
        // 3. put the cards back and roll back to start
        let game = self.game.clone();
        let mut search_path: Vec<usize> = Vec::with_capacity(32);

        for _ in 0..self.simulation_count {
            self.game = game.clone();
            search_path.clear();

            let mut node = &mut root;

            while node.expanded && !self.game.over() {
                if self.game.all_hands_empty() {
                    self.game.deal_users();
                }
                // select child node
                // push the selected search path
                let idex = node.select_child();

                search_path.push(idex);
                node = &mut node.nodes[idex];
                match node.mv {
                    Some(m) => self.game.do_full_move(&m),
                    None => panic!("Should not be none, tree: {:?}", &root),
                }
            }

            // Check if terminal state is reached (if it is then evaluate using a 1,0,-1)
            let score = if self.game.over() {
                self.game.end();
                self.game.calculate_win_for_current_player() as f32
            } else {
                // expand the node and evaluate with NN
                let (score, nodes) = self.expand();
                node.nodes = nodes;
                node.expanded = true;
                score
            };

            self.backprop(0, score, &search_path, &mut root);
        }

        root
    }

    fn backprop(&self, index: usize, score: f32, search_path: &Vec<usize>, node: &mut Node) -> f32 {
        if index == search_path.len() {
            node.t += score;
            node.n += 1;
            node.t_average = node.t / node.n as f32;

            return score;
        }
        let next_child = &mut node.nodes[search_path[index]];
        let this_score = -self.backprop(index + 1, score, search_path, next_child);

        node.n += 1;
        node.t += this_score;
        node.t_average = node.t / node.n as f32;
        this_score
    }

    fn expand(&mut self) -> (f32, Vec<Node>) {
        if self.game.all_hands_empty() {
            self.game.deal_users();
        }
        let mut mvs = self.game.generate_moves();
        let (eval, evals) = self.nn_evaluate(&mvs);

        if evals.len() != mvs.len() {
            mvs = Self::multi_pickup_heuristic(&mvs);
        }

        let mut eval_lookup = [0.0; Card::NUM_CARDS];
        for &(score, card) in evals.iter() {
            eval_lookup[card.num()] = score;
        }

        let mut nodes = Vec::with_capacity(mvs.len());

        for mv in &mvs {
            nodes.push(Node::new_nn(*mv, eval_lookup[mv.get_down_card().num()]));
        }

        (eval, nodes)
    }

    fn nn_evaluate(&self, mvs: &[game::Move]) -> (f32, Vec<(f32, Card)>) {
        let t = nn::encode_tensor(&self.game);
        let (win, mut logits) = self.nn.infer(t, &self.nn.devices()[0]);

        // mask the logits
        // could do this in a different way with the value index
        let mut legal_mask = [false; Card::NUM_CARDS];
        for mv in mvs {
            legal_mask[mv.get_down_card().num()] = true;
        }

        for (logit, &is_legal) in logits.iter_mut().zip(legal_mask.iter()) {
            if !is_legal {
                *logit = f32::NEG_INFINITY;
            }
        }
        logits = softmax(logits);
        let mut out = Vec::<(f32, Card)>::with_capacity(mvs.len());

        for (i, (&logit, &is_legal)) in logits.iter().zip(legal_mask.iter()).enumerate() {
            if is_legal {
                // valid move so push
                out.push((logit, Card::from_num(i)));
            }
        }

        (win, out)
    }

    // This function is used to decode the best move if there are multiple possible pickups
    // Limitations of this:
    // - could result in a bad situation for the engine forcing a scopa for the opponent
    // - limits engine's visibility
    //
    // Alternative approach:
    // Prior splitting
    // - keep all valid child nodes but split engine's probability
    // - could do it as a heuristic so a better probability is assigned to seven, etc
    fn multi_pickup_heuristic(mvs: &[game::Move]) -> Vec<game::Move> {
        // 1. Find the overlapping cards
        let mut buf: [u8; 40] = [0; 40];
        for mv in mvs {
            match mv {
                game::Move::Down(c) => buf[c.num()] += 1,
                game::Move::Up(c, _) => buf[c.num()] += 1,
            }
        }

        // 2. score and select the nonoverlapping moves
        // using a sentinel for tight array packing (Option would be larger)
        let mut vals: [(f32, usize); 40] = [(0.0, usize::MAX); 40];
        let mut out = Vec::with_capacity(mvs.len());
        for (i, mv) in mvs.iter().enumerate() {
            let num = match mv {
                game::Move::Down(c) => c.num(),
                game::Move::Up(c, _) => c.num(),
            };

            if buf[num] > 1 {
                let mv_eval = mvs[i].heuristic();
                let (prev_val, prev_idex) = vals[num];

                if prev_idex == usize::MAX || mv_eval > prev_val {
                    vals[num] = (mvs[i].heuristic(), i);
                }
            } else {
                out.push(*mv);
            }
        }

        // 3. push best overlapping moves
        for i in 0..vals.len() {
            let idx = vals[i].1;
            if idx == usize::MAX {
                continue;
            }
            out.push(mvs[idx]);
        }
        out
    }
}

pub fn softmax(array: Vec<f32>) -> Vec<f32> {
    let max_val = array.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let mut softmax_array = array;

    let mut sum = 0.0;
    for value in &mut softmax_array {
        if *value != f32::NEG_INFINITY {
            *value = (*value - max_val).exp();
            sum += *value;
        } else {
            *value = 0.0;
        }
    }

    if sum > 0.0 {
        for value in &mut softmax_array {
            *value /= sum;
        }
    }

    softmax_array
}

#[cfg(test)]
mod tests {
    use crate::game::{Card, Move, Suit, Value};
    use crate::mcts::MctsNn;

    #[test]
    fn multi_move_heuristic() {
        let m1 = Move::Down(Card {
            val: Value::Jack,
            suit: Suit::Diamonds,
        });

        let m2 = Move::new_up(
            Card::new(Value::King, Suit::Hearts),
            vec![
                Card::new(Value::Seven, Suit::Diamonds),
                Card::new(Value::Three, Suit::Spades),
            ],
        );

        let m3 = Move::new_up(
            Card::new(Value::King, Suit::Hearts),
            vec![
                Card::new(Value::Seven, Suit::Diamonds),
                Card::new(Value::Three, Suit::Diamonds),
            ],
        );

        let m4 = Move::new_up(
            Card::new(Value::King, Suit::Hearts),
            vec![
                Card::new(Value::Six, Suit::Diamonds),
                Card::new(Value::Four, Suit::Diamonds),
            ],
        );

        let m5 = Move::new_up(
            Card::new(Value::Queen, Suit::Clubs),
            vec![
                Card::new(Value::Five, Suit::Hearts),
                Card::new(Value::Four, Suit::Diamonds),
            ],
        );

        let m6 = Move::new_up(
            Card::new(Value::Queen, Suit::Clubs),
            vec![
                Card::new(Value::Five, Suit::Hearts),
                Card::new(Value::Four, Suit::Spades),
            ],
        );

        let mvs = vec![m1, m2, m3, m4, m5, m6];

        let mvs2 = MctsNn::multi_pickup_heuristic(&mvs);
        assert_eq!(3, mvs2.len());

        assert!(mvs2.contains(&m1));
        assert!(!mvs2.contains(&m2));
        assert!(mvs2.contains(&m3));
        assert!(!mvs2.contains(&m4));
        assert!(mvs2.contains(&m5));
        assert!(!mvs2.contains(&m6));
    }
}
