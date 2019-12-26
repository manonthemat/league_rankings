use league_rankings::{Game, Standings};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("please specify input file: {} filename", args[0]);
    }

    let filename = &args[1];

    // open fs stream
    let f = File::open(filename).expect("Cannot open file");
    let f = BufReader::new(f);

    let mut standings = Standings::default();

    for line in f.lines() {
        // lazy reading into buffer and ingesting lines one by one
        standings.ingest(Game::from_str(&line.unwrap()).unwrap());
    }
    standings.print_rankings();
}
