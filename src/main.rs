use std::fs;

// ascii codes
const NUMERIC_START: u8 = 48;
const ALPHA_START: u8 = 97;
const DIFF: u8 = ALPHA_START - NUMERIC_START;

fn main() {
    let src = fs::read_to_string("./examples/simple.berg").unwrap();
    let beats: Vec<Vec<char>> = src
        .split(',')
        .map(|b| b.chars().map(|ch| (ch as u8 + DIFF) as char).collect())
        .collect();

    print!("{:?}", beats);
}
