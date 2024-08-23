use std::io::{stdout, Write};
use std::{thread, time::Duration};
use colored::*;

// Initialize cubeWidth, then canvas width, and height
const CUBE_WIDTH: usize = 20;
const CANVAS_WIDTH: usize = 50;
const CANVAS_HEIGHT: usize = 40;
const BACKGROUND_ASCII_CODE: char = ' ';

const DISTANCE_FROM_CAMERA: f32 = 200.0;
const HORIZONTAL_OFFSET: f32 = 10.0;

const RESOLUTION_STEP: f32 = 1.0;

// Screen Constant 
const K1: f32 = 40.0;


fn clear_screen() {
    // Clear the terminal screen
    print!("\x1b[2J");
    // Move cursor to home position (0,0) using ANSI escape code
    print!("\x1b[H");
    // Ensure the buffer is flushed immediately so that the command takes effect
    stdout().flush().unwrap();
}

fn print_buffer(buffer:&Vec<char>) {
    // Move cursor to home position (0,0) using ANSI escape code
    print!("\x1b[H");

    for k in 0..(CANVAS_WIDTH * CANVAS_HEIGHT) {
        if k % CANVAS_WIDTH != 0 {
            print!("{}", buffer[k]);
        } else {
            print!("\n");
        }
    }
}



fn calculate_x(
    i: f32, 
    j: f32, 
    k: f32, 
    A: f32,
    B: f32,
    C: f32,
) -> f32 {
    let sin_a = A.to_radians().sin();
    let cos_a = A.to_radians().cos();
    let sin_b = B.to_radians().sin();
    let cos_b = B.to_radians().cos();
    let sin_c = C.to_radians().sin();
    let cos_c = C.to_radians().cos();

    j as f32 * sin_a * sin_b * cos_c
        - k as f32 * cos_a * sin_b * cos_c
        + j as f32 * cos_a * sin_c
        + k as f32 * sin_a * sin_c
        + i as f32 * cos_b * cos_c
}

fn calculate_y(
    i: f32, 
    j: f32,
    k: f32,
    A: f32,
    B: f32,
    C: f32,
) -> f32 {
    let sin_a = A.to_radians().sin();
    let cos_a = A.to_radians().cos();
    let sin_b = B.to_radians().sin();
    let cos_b = B.to_radians().cos();
    let sin_c = C.to_radians().sin();
    let cos_c = C.to_radians().cos();

    j as f32 * cos_a * cos_c
        + k as f32 * sin_a * cos_c
        - j as f32 * sin_a * sin_b * sin_c
        + k as f32 * cos_a * sin_b * sin_c
        - i as f32 * cos_b * sin_c
}

fn calculate_z(
    i: f32, 
    j: f32,
    k: f32,
    A: f32,
    B: f32,
    C: f32,
) -> f32 {
    let sin_a = A.to_radians().sin();
    let cos_a = A.to_radians().cos();
    let sin_b = B.to_radians().sin();
    let cos_b = B.to_radians().cos();

    k as f32 * cos_a * cos_b
        - j as f32 * sin_a * cos_b
        + i as f32 * sin_b
}

fn calculate_for_surface(
    cube_x: f32,
    cube_y: f32,
    cube_z: f32,
    ch: char,
    z_buffer: &mut Vec<f32>,
    buffer: &mut Vec<char>,
    A: f32,
    B: f32,
    C: f32,
) {
    let x = calculate_x(cube_x, cube_y, cube_z, A, B, C);
    let y = calculate_y(cube_x, cube_y, cube_z, A, B, C);
    let z = calculate_z(cube_x, cube_y, cube_z, A, B, C) + DISTANCE_FROM_CAMERA;

    let ooz = 1.0 / z;

    let xp = (CANVAS_WIDTH as f32 / 2.0 + HORIZONTAL_OFFSET + K1 * ooz * x * 2.0) as isize;
    let yp = (CANVAS_HEIGHT as f32 / 2.0 + K1 * ooz * y) as isize;

    let idx = xp + yp * CANVAS_WIDTH as isize;
    if idx >= 0 && (idx as usize) < CANVAS_WIDTH * CANVAS_HEIGHT {
        let idx = idx as usize;

        if ooz > z_buffer[idx] {
            z_buffer[idx] = ooz;
            buffer[idx] = ch;
        }
    }
}

fn main() {
    // Create a vector for the zBuffer initialized to 0.0 (f32 values)
    let mut z_buffer: Vec<f32> = vec![0.0; CANVAS_WIDTH * CANVAS_HEIGHT];
    // Create a vector for the buffer initialized to the BACKGROUND_ASCII_CODE
    let mut buffer: Vec<char> = vec![BACKGROUND_ASCII_CODE; CANVAS_WIDTH * CANVAS_HEIGHT];
    clear_screen();

    let mut A: f32 = 0.0;
    let mut B: f32 = 0.0;
    let mut C: f32 = 0.0;
    
    loop {
        // Clear screen and z buffers
        buffer.fill(BACKGROUND_ASCII_CODE);
        z_buffer.fill(0.0);
        
        let mut cube_x = -1.0 * (CUBE_WIDTH as f32);
        while cube_x < CUBE_WIDTH as f32 {
            
            let mut cube_y = -1.0 * (CUBE_WIDTH as f32);
            while cube_y < CUBE_WIDTH as f32 {
                // Plane F; Front side, We update X and Y, and keep Z constant 
                //      _______
                //     /      /|
                //    /______/ |
                //    |  F   | |
                //    |      | /
                //    |______|/
                //
                // (Multiplying by -1 means closer to camera)
                let z_value = -1.0 * (CUBE_WIDTH as f32);
                calculate_for_surface(cube_x, cube_y, z_value, 'F', &mut z_buffer, &mut buffer, A, B, C);
                
                //  Plane K; Back side, We update X and Y, and keep Z constant 
                //       ________
                //      /|  K   |
                //     / |      |
                //    |  |______|
                //    | /      /
                //    |/______/
                //
                // (Multiplying by positive 1 means further from camera)
                let z_value = 1.0 * (CUBE_WIDTH as f32);
                calculate_for_surface(cube_x, cube_y, z_value, 'K', &mut z_buffer, &mut buffer, A, B, C);

                //  Plane L; Left side, we update Y and Z, and keep X constant
                //           _______
                //         / |     |
                // L -->  /  |     |
                //        |  |_____|
                //        | /      /
                //        |/______/
                //
                // (Multiplying by -1 means to the left)
                let x_value = -1.0 * (CUBE_WIDTH as f32);
                calculate_for_surface(x_value, cube_y, cube_x, 'L', &mut z_buffer, &mut buffer, A, B, C);
                
                //  Plane R; Right side, we update Y and Z, and keep X constant
                //           _______
                //         / |    /|
                //        /  |   / | <-- R
                //        |  |___| |
                //        | /    | /
                //        |/_____|/
                //
                // (Multiplying by 1 means to the right)
                let x_value = 1.0 * (CUBE_WIDTH as f32);
                calculate_for_surface(x_value, cube_y, cube_x, 'R', &mut z_buffer, &mut buffer, A, B, C);

                //  Plane T; Top, we update X and Z, and keep Y constant
                //           _______
                //         /   T  /|
                //        /______/ | 
                //        |      | |
                //        |      | /
                //        |______|/
                //
                // (Multiplying by -1 means to the top)
                let y_value = -1.0 * (CUBE_WIDTH as f32);
                calculate_for_surface(cube_x, y_value, cube_y, 'T', &mut z_buffer, &mut buffer, A, B, C);

                //  Plane B; Bottom, we update X and Z, and keep Y constant
                //           _______
                //         / |     |
                //        /  |     |
                //        |  |_____|
                //        | /   B  /
                //        |/______/
                //
                // (Multiplying by 1 means to the bottom)
                let y_value = 1.0 * (CUBE_WIDTH as f32);
                calculate_for_surface(cube_x, y_value, cube_y, 'B', &mut z_buffer, &mut buffer, A, B, C);

                // Increment the cube_y by RESOLUTION_STEP
                cube_y += RESOLUTION_STEP;
            }
            
            // Increment the cube_x by RESOLUTION_STEP
            cube_x += RESOLUTION_STEP;
        }
        
        print_buffer(&buffer);

        thread::sleep(Duration::from_millis(16)); // 60 fps!
        
        A += 2.5; // x
        B += 2.5; // y
        C += 2.0; //z
    }

}
