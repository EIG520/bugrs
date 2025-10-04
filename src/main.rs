use std::env;
use std::time::Duration;

use bugrs::generators::anneal::*;
use bugrs::generators::mcts::TreeSearcher;
use bugrs::search::multisearch::{FullSearcher, Search};
use bugrs::util::parse_usize_const;
use bugrs::{bug::bitbug::BitBug};

const WIDTH: usize = 31;
const HEIGHT: usize= 21;

const SIZE: usize = WIDTH * HEIGHT;

fn main() {
    let args = std::env::args();
    let mut anneal = false;
    let mut testc = false;
    let mut mcts = false;

    for arg in args {
        match arg.as_str() {
            "anneal" => {anneal = true;}
            "mcts" => {mcts = true;}
            "test" => {testc = true;}
            _ => {}
        }
        if arg == "anneal" {
            anneal = true;
        }
    }

    if mcts {
        let mut trs = TreeSearcher::<WIDTH, SIZE>::default();

        loop {
            trs.playout();
        }

        return;
    }

    if testc {
        let bug: BitBug<WIDTH, SIZE> = BitBug::from(vec![1,2,3,4,5]);

        println!("{}", bug.to_string());
        return;
    }

    if anneal {
        let mut annealer: Annealer<WIDTH, SIZE> = Annealer::from(gen_random());
        annealer.run();
        return;
    }

    let searcher = FullSearcher::new(BitBug::<WIDTH,SIZE>::default());
    let args = std::env::args().collect::<Vec<String>>();
    let mut threads = 1;

    for (a, b) in args.iter().zip(args.iter().skip(1)) {
        match (a.as_str(), b) {
            ("threads", b) => {threads = b.parse::<usize>().expect("improper thread value")},
            _ => {}
        }
    }

    for i in 0..threads {
        searcher.spawn_search_thread(format!("{i}"));
    }

    loop {
        std::thread::sleep(Duration::from_secs(10));
    }

}
