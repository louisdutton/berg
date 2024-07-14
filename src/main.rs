mod score;
use std::fs;

use score::Score;
use serde::Deserialize;

fn main() {
    let src = fs::read_to_string("./examples/simple.berg").unwrap();
    let document = parse(&src);
    let _ = Score::from(&document).print();
}

#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    pub title: String,
    pub composer: String,
    pub bpm: f32,
}

pub struct Content {
    beats: Vec<Vec<u32>>,
}

pub struct Document {
    pub meta: Meta,
    pub content: Content,
}

fn parse(src: &String) -> Document {
    let (meta_src, content_src) = src
        .split_once("---")
        .expect("one instance of meta delimeter");

    let meta: Meta = toml::from_str(meta_src).expect("meta is valid");
    let beats = content_src
        .split(',')
        .filter_map(|b| b.chars().map(|ch| ch.to_digit(10)).collect())
        .collect();

    Document {
        meta,
        content: Content { beats },
    }
}
