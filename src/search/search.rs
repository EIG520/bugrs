use crate::bug::bitbug::BitBug;

pub struct Searcher<const WIDTH: usize, const SIZE: usize> {
    pub to_search: Vec<BitBug<WIDTH, SIZE>>,
    current_best: BitBug<WIDTH, SIZE>,
    best_score: u64
}

impl<const WIDTH: usize, const SIZE: usize> Searcher<WIDTH, SIZE> {
    pub fn search_last(&mut self) -> Option<()>{
        let mut game = self.to_search.pop()?;

        while !game.is_done() {
            let mut game_cpy = game.clone();
            game.take_step();

            if game.unseen() {
                if game_cpy.set_wall(game.pos().0, game.pos().1) {
                    self.to_search.push(game_cpy);
                }
            }
        }

        if game.steps >= self.best_score {
            println!("{}", game.clone().to_string());
            println!("{}", game.steps);

            self.best_score = game.steps;
            self.current_best = game;
        }

        Some(())
    }

    pub fn new(position: BitBug<WIDTH, SIZE>) -> Self {
        Self {
            to_search: vec![position.clone()],
            current_best: position,
            best_score: 0
        }
    }
}