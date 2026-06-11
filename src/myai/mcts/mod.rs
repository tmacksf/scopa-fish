use burn::{
    backend::Wgpu,
    module::Module,
    // tensor::{self},
};
use f32;
use rand::prelude::*;
use scopa_fish::game::{self, Card, Game};
use std::collections::HashSet;

use crate::nn;

pub type NNBackend = Wgpu<f32, i32>;

// Nodes for a mcts
#[derive(Debug)]
pub struct Node {
    t: f32,
    t_average: f32,
    n: usize,
    mv: Option<game::Move>,
    nodes: Vec<Box<Node>>,
    expanded: bool,

    // NN section
    p: f32, // initial guess by nn of the quality of the node
}

// t: total score
// n: number of visits
// t_average: t/n

impl Node {
    fn new(mv: game::Move) -> Self {
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

    fn new_root(nodes: Vec<Box<Node>>) -> Self {
        Self {
            t: 0.0,
            t_average: 0.0,
            n: 0,
            mv: None,
            expanded: true,
            p: 0.0,
            nodes,
        }
    }

    fn select_child(&self) -> usize {
        // do puct here
        return 0;
    }
}

pub struct MCTS {
    player_id: usize,
    game: Game,
}

impl MCTS {
    pub fn new(game: Game, player_id: usize) -> Self {
        Self { player_id, game }
    }

    pub fn find_move(&mut self) -> game::Move {
        let mut nodes = self.generate_nodes();

        if nodes.len() == 1 {
            return nodes[0].mv.unwrap();
        }
        let mut n = 0;
        for _ in 0..800 {
            let game = self.game.clone();

            self.selection(n, &mut nodes, 0);
            self.game = game;
            n += 1;
        }

        // select the node with the highest score
        let mut score = f32::NEG_INFINITY;
        let mut mv: Option<game::Move> = None;
        for i in 0..nodes.len() {
            let t = nodes[i].t_average;
            if t > score {
                mv = nodes[i].mv;
                score = t;
            }
        }
        mv.unwrap()
    }

    fn generate_nodes(&self) -> Vec<Box<Node>> {
        let mvs = self.game.generate_moves();
        let mut nodes = Vec::new();
        for i in 0..mvs.len() {
            nodes.push(Box::new(Node::new(mvs[i])));
        }
        nodes
    }

    fn selection(
        &mut self,
        mut prev_n: usize,
        current_nodes: &mut Vec<Box<Node>>,
        depth: usize,
    ) -> f32 {
        if current_nodes.len() == 0 {
            // got to the end of the game so evalutate and return
            return self.rollout(depth);
        }

        let mut highest = f32::NEG_INFINITY;
        let mut index = 0;
        let mut found_leaf = false;
        for i in 0..current_nodes.len() {
            // Check if leaf node
            if current_nodes[i].n == 0 {
                index = i;
                found_leaf = true;
                break;
            }
            // Calculate UCB1
            let ucb1 = current_nodes[i].t_average
                + EXPLORATION_PARAMETER
                    * f32::sqrt(f32::ln(prev_n as f32) / current_nodes[i].n as f32);
            if ucb1 > highest {
                index = i;
                highest = ucb1;
                prev_n = current_nodes[i].n;
            }
        }

        self.game.do_full_move(&current_nodes[index].mv.unwrap());
        let score = if found_leaf {
            self.rollout(depth)
        } else if current_nodes[index].nodes.len() == 0 {
            // can explore
            self.expansion(&mut current_nodes[index]);
            self.rollout(depth)
        } else {
            self.selection(prev_n, &mut current_nodes[index].nodes, depth + 1)
        };
        current_nodes[index].n += 1;
        current_nodes[index].t += score;
        current_nodes[index].t_average = score;
        return score;
    }

    fn expansion(&mut self, node: &mut Node) {
        // expansion
        if node.n == 0 {
            panic!("Got a n of 0 in expansion");
        }
        node.nodes = self.generate_nodes();
    }

    fn rollout(&mut self, mut depth: usize) -> f32 {
        let mut rng = rand::rng();
        while !self.game.over() {
            if self.game.all_hands_empty() {
                self.game.deal_users();
            }
            let mvs = self.game.generate_moves();
            let mv = mvs.choose(&mut rng).unwrap();
            self.game.do_full_move(&mv);
        }
        let s1 = self.game.players[self.player_id].calculate_score();
        let s2 = self.game.players[(self.player_id + 1) % 2].calculate_score();
        let res = s1 as f32 - s2 as f32;
        if depth == 0 {
            depth = 1;
        }
        // divide by depth to not get stuck
        if depth % 2 == 1 {
            // res / depth as f32
            res
        } else {
            // (res * -0.1) / depth as f32
            res * -0.1
        }
    }
}

pub struct MctsNn<'a> {
    player_id: usize,
    game: Game,
    simulation_count: usize,
    nn: &'a nn::Model<NNBackend>,
}

// SQRT(2)
// const EXPLORATION_PARAMETER: f32 = 1.41421356237;
const EXPLORATION_PARAMETER: f32 = 2.0; // I like 2 more for this because it means more exploring

impl<'a> MctsNn<'a> {
    pub fn new(
        game: Game,
        player_id: usize,
        simulation_count: usize,
        nn: &'a nn::Model<NNBackend>,
    ) -> Self {
        Self {
            player_id,
            game,
            nn,
            simulation_count,
        }
    }

    pub fn find_move(&mut self) -> game::Move {
        let (_score, nodes) = self.selection();
        // select the node with the highest score
        let mut visit_count = 0;
        let mut mv: Option<game::Move> = None;
        for i in 0..nodes.len() {
            if nodes[i].n > visit_count {
                mv = nodes[i].mv;
                visit_count = nodes[i].n;
            }
        }
        match mv {
            Some(m) => m,
            None => {
                self.game.debug_state(true);
                println!("Tree: {:?}", nodes);
                panic!("None move");
            }
        }
    }

    // Neural network eval
    fn selection(&mut self) -> (f32, Vec<Box<Node>>) {
        // This works differently here:
        // 1. Evaluate to find moves
        // 2. Do PUCT on the moves then expand based on that
        // 3. put the cards back and roll back to start
        let (s, nodes) = self.expand();
        let mut root = Node::new_root(nodes);
        let game = self.game.clone();

        for _ in 0..self.simulation_count {
            // for now will keep the game cloned
            self.game = game.clone();
            let mut search_path: Vec<usize> = vec![];
            let mut node = &root;

            while node.expanded {
                // select child node
                // push the selected search path
                let idex = node.select_child();

                search_path.push(idex);
                node = &node.nodes[idex];
                match node.mv {
                    Some(m) => self.game.do_move(&m),
                    None => panic!("Should not be none"),
                }
            }
            // expand the node and evaluate with NN
            let (score, nodes) = self.expand();

            // self.selection(tree.n, &mut tree.nodes, 0);

            // Need to make sure that the games are equal first
            self.backprop(0, &search_path, &mut root);
        }

        todo!();
        // return 0.0;
    }

    fn backprop(&self, index: usize, search_path: &Vec<usize>, node: &mut Node) -> f32 {
        if index == search_path.len() - 1 {
            return node.nodes[search_path[index]].t;
        }

        // Negamax
        let val = -self.backprop(
            index + 1,
            search_path,
            node.nodes[search_path[index]].as_mut(),
        );
        node.t += val;
        node.n += 1;
        node.t_average = node.t / node.n as f32;
        val
    }

    fn expand(&self) -> (f32, Vec<Box<Node>>) {
        let mut mvs = self.game.generate_moves();
        let (eval, evals) = self.nn_evaluate(&mvs);

        if evals.len() != mvs.len() {
            mvs = self.multi_pickup_heuristic(&mvs);
        }
        let mut nodes = Vec::new();
        for i in 0..mvs.len() {
            // find the
            nodes.push(Box::new(Node::new_nn(mvs[i], 0.0)));
        }

        // we retun a score and a vector of nodes.
        (eval, nodes)
    }

    fn nn_evaluate(&self, mvs: &Vec<game::Move>) -> (f32, Vec<(f32, Card)>) {
        let t = nn::encode_tensor(&self.game);
        let (win, mut logits) = self.nn.infer(t, &self.nn.devices()[0]);

        // mask the logits
        let legal_down: HashSet<game::Card> = mvs.iter().map(|c| c.get_down_card()).collect();

        // all cards is deterministic because it uses constants
        let cards = Card::all_cards();
        for i in 0..cards.len() {
            if !legal_down.contains(&cards[i]) {
                logits[i] = f32::NEG_INFINITY;
            }
        }
        logits = softmax(logits);

        (
            win,
            logits.iter().copied().zip(cards.iter().copied()).collect(),
        )
    }

    // This function is used to decode the best move if there are multiple possible pickups
    // could train another NN for this but will see
    fn multi_pickup_heuristic(
        &self,
        mvs: &Vec<game::Move>,
        // evals: &Vec<(f32, game::Card)>,
    ) -> Vec<game::Move> {
        // look for sevens then look for diamonds
        // how this will work:
        // - each set of moves will have a heuristic score
        // - the one with the highest score will be used
        // - the score will be calculated based on how many parts of a point each card is worth

        // 1. Find the overlapping cards
        let mut buf: [u8; 40] = [0; 40];
        for i in 0..mvs.len() {
            match mvs[i] {
                game::Move::Down(c) => buf[c.num()] += 1,
                game::Move::Up(c, _) => buf[c.num()] += 1,
            }
        }

        // 2. score the overlapping moves
        let mut vals: [(f32, usize); 40] = [(0.0, 0); 40];
        for i in 0..mvs.len() {
            let (num, val) = match mvs[i] {
                game::Move::Down(c) => (c.num(), buf[c.num()]),
                game::Move::Up(c, _) => (c.num(), buf[c.num()]),
            };

            if val > 1 {
                // do what?
                vals[num] = (mvs[i].heuristic(), i);
            }
        }

        // 3. Create a buffer with the correct moves
        todo!()
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
