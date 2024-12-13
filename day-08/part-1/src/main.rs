use std::collections::{HashMap, HashSet};

use shared::{position::Position, stdin::read_line};

fn main() {
    let map = Map::from_stdin();
    let antennas = get_antennas(&map);
    let mut antinodes_positions = HashSet::new();

    for positions in antennas.values() {
        for (i, &first_position) in positions.iter().enumerate() {
            for &second_position in &positions[i + 1..] {
                let delta = first_position - second_position;

                let antinode_1 = first_position + delta;
                let antinode_2 = second_position - delta;

                if map.contains(antinode_1) {
                    antinodes_positions.insert(antinode_1);
                }
                if map.contains(antinode_2) {
                    antinodes_positions.insert(antinode_2);
                }
            }
        }
    }
    println!("{}", antinodes_positions.len());
}

fn get_antennas(map: &Map) -> HashMap<char, Vec<Position>> {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    for i in 0..map.rows() {
        for j in 0..map.cols() {
            let position = (i, j).into();
            let cell = map.at(position);
            if cell != '.' {
                antennas.entry(cell).or_default().push(position);
            }
        }
    }
    antennas
}

struct Map {
    cells: Vec<String>,
}

impl Map {
    fn from_stdin() -> Self {
        let mut cells = Vec::new();
        while let Some(line) = read_line() {
            cells.push(line.trim().to_string());
        }
        Self { cells }
    }

    fn rows(&self) -> usize {
        self.cells.len()
    }

    fn cols(&self) -> usize {
        self.cells[0].len()
    }

    fn at(&self, position: Position) -> char {
        self.cells[position.i as usize].as_bytes()[position.j as usize] as char
    }

    fn contains(&self, position: Position) -> bool {
        position.i >= 0
            && position.i < self.cells.len() as i32
            && position.j >= 0
            && position.j < self.cells[0].len() as i32
    }
}
