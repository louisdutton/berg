mod score;
use std::fs;

use score::Score;

fn main() {
    let src = fs::read_to_string("./examples/simple.berg").unwrap();

    let beats: Vec<Vec<u32>> = src
        .split(',')
        .filter_map(|b| b.chars().map(|ch| ch.to_digit(10)).collect())
        .collect();

    let _ = Score::from(beats).print();
}
