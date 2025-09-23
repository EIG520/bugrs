use std::time::Instant;

use bugrs::{bug::bitbug::BitBug, search::search::Searcher};

const WIDTH: usize = 9;
const SIZE: usize = WIDTH * WIDTH;

fn main() {
    let mut searcher = Searcher::new(BitBug::<WIDTH,SIZE>::default());

    let time = Instant::now();

    let mut i: u64 = 0;
    while let Some(_) = searcher.search_last() {
        if i % 10000000 == 0 {
            println!("{i} boards checked in {:?}", time.elapsed());
        }

        i+=1;
    }

    println!("{i} boards checked in {:?}", time.elapsed());
}
