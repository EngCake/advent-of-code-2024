use core::panic;
use std::collections::HashSet;

use shared::{position::Position, stdin::read_line};

fn main() {
    let map = Map::from_stdin();
    let result = run_guard_partol(&map);
    println!("{}", result);
}

fn run_guard_partol(map: &Map) -> i32 {
    let mut guard_position = map.guard_starting_position;
    let mut guard_direction = (-1, 0).into();
    let mut visited_positions = HashSet::new();

    while map.contains(guard_position) {
        visited_positions.insert(guard_position);
        match map.at(guard_position + guard_direction) {
            Some(CellState::Empty) => guard_position += guard_direction,
            Some(CellState::Obstacle) => guard_direction = guard_direction.rotate_clockwise(),
            None => break,
        }
    }

    visited_positions.len() as i32
}

#[derive(Clone, Copy)]
enum CellState {
    Empty,
    Obstacle,
}

struct Map {
    cells: Vec<Vec<CellState>>,
    guard_starting_position: Position,
}

impl Map {
    fn from_stdin() -> Self {
        let mut cells = Vec::new();
        let mut guard_starting_position = (0, 0).into();

        while let Some(line) = read_line() {
            let mut row = Vec::new();
            for cell in line.trim().as_bytes() {
                row.push(match cell {
                    b'.' => CellState::Empty,
                    b'#' => CellState::Obstacle,
                    b'^' => {
                        guard_starting_position = (cells.len() as i32, row.len() as i32).into();
                        CellState::Empty
                    }
                    _ => panic!("Unrecognized input provided to map: {}", cell),
                })
            }
            cells.push(row);
        }
        Self {
            cells,
            guard_starting_position,
        }
    }

    fn contains(&self, position: Position) -> bool {
        let i = position.i;
        let j = position.j;
        i >= 0 && i < self.cells.len() as i32 && j >= 0 && j < self.cells[i as usize].len() as i32
    }

    fn at(&self, position: Position) -> Option<CellState> {
        if self.contains(position) {
            Some(self.cells[position.i as usize][position.j as usize])
        } else {
            None
        }
    }
}
