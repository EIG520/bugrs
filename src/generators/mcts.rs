use std::{cell::RefCell, rc::Rc};

use rand::seq::IndexedRandom;

use crate::bug::bitbug::BitBug;

pub struct TreeSearcher<const WIDTH: usize, const SIZE: usize> {
    root: TreeNode<WIDTH, SIZE>,
    best_score: u64,
}

impl<const WIDTH: usize, const SIZE: usize> Default for TreeSearcher<WIDTH, SIZE> {
    fn default() -> Self {
        Self {
            root: TreeNode::<WIDTH, SIZE>::from(vec![]),
            best_score: 1,
        }
    }
}

impl<const WIDTH: usize, const SIZE: usize> TreeSearcher<WIDTH, SIZE> {
    pub fn playout(&mut self) {
        let sc = self.root.playout(self.best_score);

        if sc > self.best_score {
            self.best_score = sc;
        }
    
    }
}

pub struct TreeNode<const WIDTH: usize, const SIZE: usize> {
    nodes: Vec<Rc<RefCell<TreeNode<WIDTH, SIZE>>>>,
    score: f32,
    playouts: usize,
    game: Vec<usize>
}

impl<const WIDTH: usize, const SIZE: usize> TreeNode<WIDTH, SIZE> {
    fn playout(&mut self, target: u64) -> u64 {
        let mut best_uct = -1.0;
        let mut best_vec = vec![];

        self.expand();
        self.playouts += 1;

        

        for node in &self.nodes {
            let score = node.borrow().score;
            let playouts = node.borrow().playouts as f32 + 0.001;

            let uct = score / (playouts * target as f32) + (2.0 * (self.playouts as f32).ln() / playouts).sqrt();

            // println!("{uct}");

            if uct == best_uct && rand::random_range(0.0..1.0) > 0.25 {
                best_uct = uct;
                best_vec.push(node);
            }

            if uct > best_uct {
                best_uct = uct;
                best_vec.clear();
                best_vec.push(node);
            }
        }

        if let Some(node) = best_vec.choose(&mut rand::rng()) {
            let delta = node.borrow_mut().playout(target);
            self.score += delta as f32;
            self.playouts += 1;

            return delta;
        }

        let mut bug = BitBug::<WIDTH, SIZE>::from(self.game.clone());
        let score = bug.simulate();

        self.score += score as f32;
        
        if score > target {
            println!("{}", bug.to_string());
            println!("{}", score);
            println!("{}", self.playouts);
        }

        return score;
    }

    fn expand(&mut self) {
        if self.nodes.len() > 0 {return;}

        let mut bug = BitBug::<WIDTH, SIZE>::from(self.game.clone());

        let mut i = 0;
        while !bug.is_done() {
            if bug.can_wall_next() {
                let mut v2 = self.game.clone();
                v2.push(i);

                // println!("{:?}", v2);

                self.nodes.push(Rc::new(RefCell::new(TreeNode::<WIDTH, SIZE>::from(v2))));
                i += 1;
            }

            bug.take_step();
        }
    }
}

impl<const WIDTH: usize, const SIZE: usize> From<Vec<usize>> for TreeNode<WIDTH, SIZE> {
    fn from(value: Vec<usize>) -> Self {
        Self {
            nodes: vec![],
            score: 0.0,
            playouts: 0,
            game: value
        }
    }
}

fn sigmoid(num: f32) -> f32 {
    1.0 / (1.0 + (-num).exp())
}