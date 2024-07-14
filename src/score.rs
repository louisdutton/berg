use crate::{Document, Meta};

const BLANK: char = '-';
const NOTE: char = 'o';
const BORDER_HORIZONTAL: char = '─';
const BORDER_VERTICAL: char = '│';
const BORDER_TOP_LEFT: char = '╭';
const BORDER_TOP_RIGHT: char = '╮';
const BORDER_BOTTOM_LEFT: char = '╰';
const BORDER_BOTTOM_RIGHT: char = '╯';

#[derive(Debug)]
pub struct Score {
    meta: Meta,
    slices: Vec<Vec<char>>,
}

impl Score {
    pub fn from(document: &Document) -> Self {
        Score {
            meta: document.meta.clone(),
            slices: document
                .content
                .beats
                .iter()
                .map(|b| {
                    let mut v = vec![BLANK; 12];
                    v[12 - b[0] as usize] = NOTE;
                    v
                })
                .collect(),
        }
    }

    pub fn print(&self) {
        let height = 12;

        println!("{}", self.meta.title);
        println!("By {}", self.meta.composer);
        println!(
            "{}{:3}{}",
            BORDER_TOP_LEFT, BORDER_HORIZONTAL, BORDER_TOP_RIGHT,
        );
        for row in 0..height {
            print!("{}", BORDER_VERTICAL);
            for slice in &self.slices {
                print!("{}", slice.get(row).unwrap_or(&BLANK));
            }
            print!("{}", BORDER_VERTICAL);
            println!();
        }
        println!(
            "{}{:width$}{}",
            BORDER_BOTTOM_LEFT,
            BORDER_HORIZONTAL,
            BORDER_BOTTOM_RIGHT,
            width = self.slices.len()
        );
    }
}
