use crate::bug::bitbug::BitBug;
use crate::search::search::Searcher;
use std::sync::{Mutex, Arc};
use std::time::Instant;

pub struct MSearcher<const WIDTH: usize, const SIZE: usize> {
    pub to_search: Mutex<Vec<BitBug<WIDTH, SIZE>>>,
    current_best: Mutex<BitBug<WIDTH, SIZE>>,
    best_score: Mutex<u64>
}

impl<const WIDTH: usize, const SIZE: usize> MSearcher<WIDTH, SIZE> {
    pub fn search_last(&self) -> Option<()>{
        let mut game = self.to_search.lock().ok()?.pop()?;

        while !game.is_done() {
            let mut game_cpy = game.clone();
            game.take_step();

            if game.unseen() {
                if game_cpy.set_wall(game.pos().0, game.pos().1) {
                    if game_cpy.steps <= 10 {
                        self.to_search.lock().ok()?.push(game_cpy);
                    } else {
                        let mut searcher = Searcher::new(game_cpy);

                        while let Some(_) = searcher.search_last() {}

                        if searcher.best_score >= *self.best_score.lock().ok()? {
                            print!("{}", searcher.best_score.clone());
                            println!("{}", searcher.current_best.to_string());
                            println!();

                            *self.best_score.lock().ok()? = searcher.best_score;
                            *self.current_best.lock().ok()? = searcher.current_best;
                        }
                    }
                }
            }
        }

        if game.steps >= *self.best_score.lock().ok()? {
            print!("{}", game.steps);
            println!("{}", game.to_string());
            println!();

            *self.best_score.lock().unwrap() = game.steps;
            *self.current_best.lock().unwrap() = game;
        }

        Some(())
    }

    pub fn new(position: BitBug<WIDTH, SIZE>) -> Self {
        Self {
            to_search: Mutex::new(vec![position.clone()]),
            current_best: Mutex::new(position),
            best_score: Mutex::new(0)
        }
    }
}

pub struct FullSearcher<const WIDTH: usize, const SIZE: usize> {
    search: Arc<MSearcher<WIDTH, SIZE>>,
}

pub trait Search {
    fn spawn_search_thread(&self, id: String);
}

impl<const WIDTH: usize, const SIZE: usize> Search for FullSearcher<WIDTH, SIZE> {
    fn spawn_search_thread(&self, id: String) {
        let search = Arc::clone(&self.search);

        std::thread::spawn(move || {
            let time = Instant::now();
            while let Some(_) = search.search_last() {}

            println!("Thread {id} Done {:?}", time.elapsed());
        });
    }
}

impl<const WIDTH: usize, const SIZE: usize> FullSearcher<WIDTH, SIZE> {
    pub fn new(position: BitBug<WIDTH, SIZE>) -> Self {
        Self {
            search: Arc::new(MSearcher::new(position)),
        }
    }

    pub fn best_score(&self) -> u64 {
        *self.search.best_score.lock().unwrap()
    }
}