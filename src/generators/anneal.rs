use std::thread::Thread;

use rand::{rngs::ThreadRng, Rng};

use crate::bug::bitbug::{BitBug, TILE_MAX};

#[derive(Clone)]
pub struct VecBug {
    data: Vec<usize>,
    steps: u64
}

pub fn gen_random<const WIDTH: usize, const SIZE: usize>() -> BitBug<WIDTH, SIZE> {
    let mut bug = BitBug::default();
    
    for x in 1..(WIDTH - 1) {
        for y in 1..(SIZE / WIDTH - 1) {
            if rand::random::<bool>() {
                let mut b2 = bug.clone();
                if b2.set_wall(x, y) {
                    bug.set_wall(x, y);
                }
            }
        }
    }

    bug
}

pub fn anneal<const WIDTH: usize, const SIZE: usize>(bug: BitBug<WIDTH, SIZE>, sheat: f32) -> Option<BitBug<WIDTH, SIZE>> {
    let mut rng = rand::rng();
    let mut nbug = BitBug::default();

    let heat = 0.001f32;
    
    for x in 1..(WIDTH - 1) {
        for y in 1..(SIZE / WIDTH - 1) {
            if rng.random_range(0.0..1.0) < heat {
                if bug.tiles.get(x,y) < TILE_MAX - 2 {
                    let mut b2 = nbug.clone();
                    if b2.set_wall(x, y) {
                        nbug.set_wall(x, y);
                    } else {
                        return None
                    }
                }
            } else if bug.tiles.get(x,y) > TILE_MAX - 3 {
                let mut b2 = nbug.clone();
                if b2.set_wall(x, y) {
                    nbug.set_wall(x, y);
                }
            }
        }
    }

    Some(nbug)
}

pub fn vec_anneal<const WIDTH: usize, const SIZE: usize>(bug: &VecBug, rng: &mut ThreadRng) -> VecBug {
    let mut nvec = bug.data.clone();

    for _ in 0..10 {
        let idx = rng.random_range(0..bug.data.len());
        let delt = rng.random_range(0..=1) * 2;

        // println!("{idx}, {delt}");

        nvec[idx] += delt;

        if nvec[idx] > 0 {
            nvec[idx] -= 1;
        }

        // println!("{nvec:?}");
    }

    let mut bug: BitBug<WIDTH, SIZE> = BitBug::from(nvec.clone());

    VecBug { data: nvec, steps: bug.simulate() }
}

pub struct Annealer<const WIDTH: usize, const SIZE: usize> {
    best_bug: BitBug<WIDTH, SIZE>,
    best_score: u64,
    true_best: BitBug<WIDTH, SIZE>,
    true_score: u64
}

impl<const WIDTH: usize, const SIZE: usize> Annealer<WIDTH, SIZE> {
    pub fn run(&mut self) {
        let max = 100000;
        let mut rng = rand::rng();
        let mut li = 0;

        for _ in 0..5000 {
            println!("new epoch");
            for i in 0..max {
                let temp = 1.0 - (i as f32 + 1.0) / (max as f32);

                if let Some(mut nbug) = anneal(self.best_bug.clone(), temp) {
                    let steps = nbug.simulate();

                    if steps > self.true_score {
                        println!("Steps: {}", steps);
                        println!("{}", nbug.to_string());
                        println!("temp: {temp}");

                        self.true_best = nbug.clone();
                        self.true_score = steps;
                        li = i;
                    }

                    // println!("Steps: {}", steps);
                    // println!("{}", nbug.to_string());
                    // println!("temp: {temp}");

                    if (-(self.best_score as f32 - steps as f32) / temp).exp() >= rng.random_range(0.0..1.0)  {
                        self.best_score = steps;
                        self.best_bug = nbug;
                    }

                    if i - li > max / 10 {
                        self.best_bug = self.true_best.clone();
                        self.best_score = self.true_score;
                        li = i;
                    }
                }
            }
        }
    }
}

impl<const WIDTH: usize, const SIZE: usize> From<BitBug<WIDTH, SIZE>> for Annealer<WIDTH, SIZE> {
    fn from(value: BitBug<WIDTH, SIZE>) -> Self {
        Self {
            best_bug: value.clone(),
            best_score: value.clone().simulate(),
            true_best: value.clone(),
            true_score: value.clone().simulate(),
        }
    }
}


pub struct VecAnnealer<const WIDTH: usize, const SIZE: usize> {
    best_bug: VecBug,
    true_best: VecBug,
}

impl<const WIDTH: usize, const SIZE: usize> VecAnnealer<WIDTH, SIZE> {
    pub fn run(&mut self) {
        let max = 1000000;
        let mut rng = rand::rng();

        
        for i in 0..max {
            let temp = 1.0 - (i as f32 + 1.0) / (max as f32);
            let mut bug = vec_anneal::<WIDTH, SIZE>(&self.best_bug, &mut rng);

            if bug.steps > self.true_best.steps {
                println!("steps: {}", bug.steps);
                println!("data:  {:?}", bug.data);
                println!("temp:  {}", temp);

                self.true_best = bug.clone();
            }

            // println!("steps: {} ({})", bug.steps, self.true_best.steps);
            // println!("data:  {:?}", bug.data);
            // println!("temp:  {}", temp);

            if (-(self.best_bug.steps as f32 - bug.steps as f32) / temp).exp() >= rand::random_range(0.0..1.0)  {
                self.best_bug = bug;
            }
        }
    }

    pub fn random(size: usize) -> VecAnnealer<WIDTH, SIZE> {
        let mut vc = vec![0; size];

        for i in 0..vc.len() {
            vc[i] = rand::random_range(0..WIDTH);
        }

        let mut bug: BitBug<WIDTH, SIZE> = BitBug::from(vc.clone());
        let vbug = VecBug { data: vc, steps: bug.simulate() };

        Self {
            best_bug: vbug.clone(),
            true_best: vbug,
        }
    }
}