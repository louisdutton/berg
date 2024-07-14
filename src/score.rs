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
    slices: Vec<Vec<char>>,
}

impl Score {
    pub fn from(beats: Vec<Vec<u32>>) -> Self {
        Score {
            slices: beats
                .iter()
                .map(|b| {
                    let mut v = vec![BLANK; 12];
                    v[11 - b[0] as usize] = NOTE;
                    v
                })
                .collect(),
        }
    }

    pub fn print(&self) {
        let height = 12;

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
