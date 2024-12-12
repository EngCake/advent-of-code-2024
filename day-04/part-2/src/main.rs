use std::io::stdin;

fn main() {
    let grid = Grid::from_stdin();
    let mut count = 0;
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            if check_xmas_at(&grid, i, j) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn check_xmas_at(grid: &Grid, i: i32, j: i32) -> bool {
    let deltas = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

    if i == 0 || i == grid.rows() - 1 || j == 0 || j == grid.cols() - 1 {
        return false;
    }

    if grid.at(i, j).unwrap() == 'A' {
        let diagonals: String = deltas
            .iter()
            .map(|(delta_i, delta_j)| grid.at(i + delta_i, j + delta_j).unwrap())
            .collect();
        match diagonals.as_str() {
            "MMSS" | "MSSM" | "SSMM" | "SMMS" => true,
            _ => false,
        }
    } else {
        false
    }
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
