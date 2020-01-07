use rand::prelude::*;

use crate::game::GameState;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Air,
    Sand,
}

pub struct Spawner;

impl Spawner {

    pub fn spawn(&self, write_state: &mut GameState) {
        for (x, y) in vec![(0,0),(2,0),(4,0),(6,0),(8,0)] {
            write_state.write_cell(Cell::Sand, x, y, false);
        }
    }

}

pub fn update_cell(cell: &Cell, x: i32, y: i32, read_state: &GameState, write_state: &mut GameState) {
    match cell {
        Cell::Sand => {
            let down = y + 1;
            let mut sideways = x - 1;
            if rand::random() {
                sideways =  x + 1;
            }
            let height = (read_state.size as i32) - 1;
            if  down <= height && read_state.is_empty(x, down) && write_state.is_empty(x, down) {
                write_state.write_cell(Cell::Sand, x, down, true);
            }
            else if down <= height && sideways >= 0 && sideways <= height && read_state.is_empty(sideways, down)  && write_state.is_empty(sideways, down) {
                 write_state.write_cell(Cell::Sand, sideways, down, true);
            }
            else {
                write_state.write_cell(Cell::Sand, x, y, false);
            }
        },
        _ => {}
    }
}