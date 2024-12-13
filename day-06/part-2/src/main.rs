use core::panic;
use std::collections::HashSet;

use shared::{position::Position, stdin::read_line};

fn main() {
    let map = Map::from_stdin();
    let mut result = 0;

    let positions = get_partol_positions(&map);
    for position in positions.iter() {
        let modified_map = map.with_obstacle_at(position);
        if detect_cycles(&modified_map) {
            result += 1;
        }
    }

    println!("{}", result);
}

fn detect_cycles(map: &Map) -> bool {
    let mut guard_position = map.guard_starting_position;
    let mut guard_direction = (-1, 0).into();
    let mut collision_positions = HashSet::new();

    while map.contains(guard_position) {
        match map.at(guard_position + guard_direction) {
            Some(CellState::Empty) => guard_position += guard_direction,
            Some(CellState::Obstacle) => {
                if collision_positions.contains(&(guard_position, guard_position + guard_direction))
                {
                    return true;
                }
                collision_positions.insert((guard_position, (guard_position + guard_direction)));
                guard_direction = guard_direction.rotate_clockwise();
            }
            None => break,
        }
    }

    false
}

fn get_partol_positions(map: &Map) -> HashSet<Position> {
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

    visited_positions
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

    fn with_obstacle_at(&self, position: &Position) -> Map {
        let mut copy = Self {
            guard_starting_position: self.guard_starting_position,
            cells: self.cells.clone(),
        };
        copy.cells[position.i as usize][position.j as usize] = CellState::Obstacle;
        copy
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
