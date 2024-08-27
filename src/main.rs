use std::{thread, time::Duration};
mod constants;
mod cube_actions;
mod cube_parameters;
mod input;
mod rendering;

use constants::*;
use cube_actions::process_input;
use cube_parameters::CubeParameters;
use input::setup_input_listener;
use rendering::{clear_screen, draw_cube, print_buffer, print_vars};

fn main() {
    let rx = setup_input_listener();

    let mut params = CubeParameters::new();
    let mut z_buffer: [f32; CANVAS_WIDTH * CANVAS_HEIGHT] = [0.0; CANVAS_WIDTH * CANVAS_HEIGHT];
    let mut buffer: [char; CANVAS_WIDTH * CANVAS_HEIGHT] = [BACKGROUND_ASCII_CODE; CANVAS_WIDTH * CANVAS_HEIGHT];

    clear_screen();
    
    loop {
        // Clear screen and z buffers
        buffer.fill(BACKGROUND_ASCII_CODE);
        z_buffer.fill(0.0);
        
        // 'draw' cube on buffer
        draw_cube(&mut z_buffer, &mut buffer, &params);
        
        // print buffer...
        print_buffer(buffer);
        print_vars(&params);
        
        thread::sleep(Duration::from_millis(16)); // 60 fps!
        
        process_input(&rx, &mut params);

        // Auto Rotation 
        params.update_auto_rotations();
    }
}
