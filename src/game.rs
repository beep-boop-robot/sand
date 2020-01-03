use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Air,
    Sand,
}

pub struct CellBlock {
    cells: Vec<Cell>,
    dirty: bool
}

impl CellBlock {

    pub fn new() -> Self {
        let cells = vec![Cell::Air; (REGION_SIZE * REGION_SIZE) as usize];
        CellBlock {
            cells,
            dirty: true
        }
    }

    pub fn cells(&self) -> Vec<(Cell, i32, i32)> {
        let mut v = Vec::new();
        for i in 0..REGION_SIZE {
            for j in 0..REGION_SIZE {
                v.push((self.cells[(i + (j * REGION_SIZE)) as usize], i, j));
            }
        }
        v
    }
}

pub struct GameState {
    pub blocks : HashMap<(i32, i32), CellBlock>
}

pub const REGION_SIZE: i32 = 8;

impl GameState {

    pub fn new() -> Self {
        let mut blocks = HashMap::new();
        blocks.insert((0, 0), CellBlock::new());
        blocks.insert((1, 0), CellBlock::new());
        blocks.insert((0, 1), CellBlock::new());
        blocks.insert((1, 1), CellBlock::new());
        GameState {
            blocks,

        }
    }

    pub fn reset_block(&mut self, x: i32, y: i32) {
        // TODO index blocks based on global x/y instead of block x/y?
        self.blocks.insert((x, y), CellBlock::new());
    }

    pub fn write_cell(&mut self, cell: Cell, x: i32, y: i32, dirty: bool) {
        let bx = x / REGION_SIZE;
        let by = y / REGION_SIZE;
        let ix = x % REGION_SIZE;
        let iy = y % REGION_SIZE;
        let b = self.blocks.get_mut(&(bx,by)).unwrap();
        if dirty {
            b.dirty = dirty;
        }
        b.cells[(ix + (iy * REGION_SIZE)) as usize] = cell;
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        let bx = x / REGION_SIZE;
        let by = y / REGION_SIZE;
        let ix = x % REGION_SIZE;
        let iy = y % REGION_SIZE;
        let b = self.blocks.get(&(bx, by)).unwrap();
        b.cells[(ix + (iy * REGION_SIZE)) as usize] == Cell::Air
    }
}

pub fn update(read_state: &GameState, write_state: &mut GameState) {

    // clear any blocks that will be changed
    // copy any blocks that won't
    for (pos, block) in read_state.blocks.iter() {
        if block.dirty {
            write_state.reset_block(pos.0, pos.1);
        }
        else{
            // copy before any potential updates. so that updates from other blocks into this one aren't lost
            write_state.blocks.get_mut(&(pos.0, pos.1)).unwrap().cells = block.cells.clone();
        }
    }

    // reset every block in target
    for (_, block) in write_state.blocks.iter_mut() {
        block.dirty = false;
    }

    for (pos, block) in read_state.blocks.iter() {
        let block_offset = (pos.0 * REGION_SIZE, pos.1 * REGION_SIZE);
        if block.dirty {
            println!("Updating: ({}, {})", pos.0, pos.1);
            for (c, i, j) in block.cells() {
                let world_pos = (i + block_offset.0, j + block_offset.1);
                match c {
                    Cell::Sand => {
                        let down = world_pos.1 + 1;
                        let left = world_pos.0 - 1 ;
                        let right = world_pos.0 + 1 ;
                        // TODO use actual world size
                        if  down <= 15 && read_state.is_empty(world_pos.0, down) {
                            write_state.write_cell(Cell::Sand, world_pos.0, down, true);
                        }
                        //else if down <= 15 && left >= 0 && read_state.is_empty(left, down) {
                        //    write_state.write_cell(Cell::Sand, left, down, true);
                        //}
                        //else if down <= 15 && right <= 15 && read_state.is_empty(right, down) {
                        //    write_state.write_cell(Cell::Sand, right, down, true);
                        //}
                        else {
                            write_state.write_cell(Cell::Sand, world_pos.0, world_pos.1, false);
                        }
                    },
                    _ => {}
                }
            }
        }            
    }
}