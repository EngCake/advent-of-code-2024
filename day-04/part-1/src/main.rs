use std::io::stdin;

fn main() {
    let grid = Grid::from_stdin();
    let mut count = 0;
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            count += count_word_at("XMAS", &grid, i, j);
        }
    }
    println!("{}", count);
}

fn count_word_at<'a>(word: &'a str, grid: &Grid, i: i32, j: i32) -> i32 {
    let mut count = 0;
    for delta_i in -1..=1 {
        for delta_j in -1..=1 {
            let mut buffer = String::new();

            for index in 0..word.len() {
                if let Some(letter) = grid.at(
                    i + delta_i as i32 * index as i32,
                    j + delta_j as i32 * index as i32,
                ) {
                    buffer.push(letter);
                } else {
                    continue;
                }
            }

            if buffer.as_str() == word {
                count += 1;
            }
        }
    }

    count
}

struct Grid {
    lines: Vec<String>,
}

impl Grid {
    fn from_stdin() -> Grid {
        let mut lines = Vec::new();
        while let Some(line) = read_line() {
            lines.push(line);
        }
        Grid { lines }
    }

    fn rows(&self) -> i32 {
        self.lines.len() as i32
    }

    fn cols(&self) -> i32 {
        self.lines[0].len() as i32
    }

    fn at(&self, i: i32, j: i32) -> Option<char> {
        if !self.is_within(i, j) {
            None
        } else {
            Some(self.lines[i as usize].as_bytes()[j as usize] as char)
        }
    }

    fn is_within(&self, i: i32, j: i32) -> bool {
        let rows = self.rows();
        let cols = self.cols();
        i >= 0 && i < rows && j >= 0 && j < cols
    }
}

fn read_line() -> Option<String> {
    let mut buffer = String::new();
    if stdin().read_line(&mut buffer).ok()? == 0 {
        return None;
    }
    if buffer == "\n" {
        return None;
    }
    return Some(buffer);
}
