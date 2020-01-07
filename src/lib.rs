extern crate sdl2;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
use std::time::{Instant, Duration};

mod game;
mod cells;

use cells::Cell;

const TEX_SIZE: u32 = 16;

pub fn dummy_texture<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Texture<'a>,String> {
    let mut tex = texture_creator.create_texture_target(None, TEX_SIZE, TEX_SIZE).map_err(|x| x.to_string())?;
    canvas.with_texture_canvas(&mut tex, |c| {
        for i in 0..TEX_SIZE {
            for j in 0..TEX_SIZE {
                if i == 0 || j == 0 || i == TEX_SIZE - 1 || j == TEX_SIZE - 1 {
                    c.set_draw_color(Color::RGB(255, 0, 0));
                }
                else{
                    c.set_draw_color(Color::RGB(0, 0, 0));
                }
                c.draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
            }
        }
    }).map_err(|x| x.to_string())?;
    Ok(tex)
}

const CELL_DRAW_SIZE: i32 = 4;
const MAP_SIZE: i32 = 8;

pub fn start() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("sand", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // TODO create a texture that acts as each sqaure
    let texture_creator = canvas.texture_creator();
    let tex = dummy_texture(&mut canvas, &texture_creator).unwrap();

    let mut read_state = game::GameState::new(MAP_SIZE);
    let mut write_state = game::GameState::new(MAP_SIZE);
    
    // TEST SETUP
    read_state.write_cell(Cell::Sand, 0, 0, true);
    read_state.write_cell(Cell::Sand, 1, 0, true);
    read_state.write_cell(Cell::Sand, 2, 0, true);
    read_state.write_cell(Cell::Sand, 0, 7, true);
    read_state.write_cell(Cell::Sand, 1, 7, true);
    read_state.write_cell(Cell::Sand, 2, 7, true);
    read_state.write_cell(Cell::Sand, 3, 7, true);
    read_state.write_cell(Cell::Sand, 1, 15, true);
    read_state.write_cell(Cell::Sand, 2, 15, true);
    read_state.write_cell(Cell::Sand, 3, 15, true);
    //read_state.write_cell(Cell::Sand, 15, 15, true);
    // TEST

    let mut frame_start = Instant::now();
    let mut ms_since_update = 0u128;

    'running: loop {
        frame_start = Instant::now();
        // TODO get constant framerate

        for event in event_pump.poll_iter() { 
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }

        if ms_since_update >= 32 {
            ms_since_update = 0;
            let update_start = Instant::now();
            game::update(&read_state, &mut write_state);
            //println!("{}", update_start.elapsed().as_micros());
            //println!("---------UPDATE END--------------");
            let tmp = read_state;
            read_state = write_state;
            write_state = tmp;
        }


        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for (pos, b) in read_state.blocks.iter() {  // todo include offset in block
            let block_offset = (pos.0 * game::REGION_SIZE, pos.1 * game::REGION_SIZE);
            for (c, i, j) in b.cells() {
                match c {
                    Cell::Sand{..} => {
                        canvas.copy(
                            &tex,
                            None,
                            Rect::new(((block_offset.0 + i) * CELL_DRAW_SIZE), ((block_offset.1 + j) * CELL_DRAW_SIZE), CELL_DRAW_SIZE as u32, CELL_DRAW_SIZE as u32)).unwrap();
                    },
                    _ => {}
                }
            }
        }
        

        canvas.present();

        let mut d = frame_start.elapsed().as_millis();
        if d == 0 {
            d = 1;
        }
        ms_since_update += d;
    }
}