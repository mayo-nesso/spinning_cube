use std::io::{stdout, Write};
use std::{thread, time::Duration};
use colored::*;
use once_cell::unsync::Lazy;
use std::collections::HashMap;

const CUBE_WIDTH: usize = 25;
const HALF_CUBE_WIDTH: usize = (CUBE_WIDTH as f32 / 2.0) as usize;
const CANVAS_WIDTH: usize = 80;
const CANVAS_HEIGHT: usize = 40;
const ASPECT_RATIO: f32 = CANVAS_WIDTH as f32 / CANVAS_HEIGHT as f32;
const BACKGROUND_ASCII_CODE: char = ' ';

// DISTANCE_FROM_CAMERA:
// Should be at least, HALF_CUBE_WIDTH, since this is 
// the distance from the center to one of the cube's faces.
const DISTANCE_FROM_CAMERA: f32 = 70.0 + HALF_CUBE_WIDTH as f32;

// PROJECTION_SCALE:
// Scale up the screen.
// It seems like a good idea to have 
// into consideration the DISTANCE_FROM_CAMERA 
const PROJECTION_SCALE: f32 = DISTANCE_FROM_CAMERA / 2.0;

//
const RESOLUTION_STEP: f32 = 0.5;

const COLOR_MAPPINGS: Lazy<HashMap<char, ColoredString>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('F', "F".yellow());
    map.insert('K', "K".red());
    map.insert('L', "L".cyan());
    map.insert('R', "R".magenta());
    map.insert('T', "T".blue());
    map.insert('B', "B".green());
    map.insert('_', "_".white()); // Default case
    map
});

fn get_colored(ch: char) -> ColoredString {
    COLOR_MAPPINGS.get(&ch).cloned().unwrap_or_else(|| " ".white())
}

fn clear_screen() {
    // Clear the terminal screen
    print!("\x1b[2J");
    // Move cursor to home position (0,0) using ANSI escape code
    print!("\x1b[H");
    // Ensure the buffer is flushed immediately so that the command takes effect
    stdout().flush().unwrap();
}

fn print_buffer(buffer:[char; CANVAS_WIDTH * CANVAS_HEIGHT]) {
    // Move cursor to home position (0,0) using ANSI escape code
    print!("\x1b[H");

    for k in 0..(CANVAS_WIDTH * CANVAS_HEIGHT) {
        if k % CANVAS_WIDTH != 0 {
            print!("{}", get_colored(buffer[k]));
            
        } else {
            print!("\n");
        }
    }
}

fn calculate_x(
    i: f32, 
    j: f32, 
    k: f32, 
    alpha: f32,
    beta: f32,
    gamma: f32,
) -> f32 {
    let sin_a = alpha.to_radians().sin();
    let cos_a = alpha.to_radians().cos();
    let sin_b = beta.to_radians().sin();
    let cos_b = beta.to_radians().cos();
    let sin_c = gamma.to_radians().sin();
    let cos_c = gamma.to_radians().cos();

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
    alpha: f32,
    beta: f32,
    gamma: f32,
) -> f32 {
    let sin_a = alpha.to_radians().sin();
    let cos_a = alpha.to_radians().cos();
    let sin_b = beta.to_radians().sin();
    let cos_b = beta.to_radians().cos();
    let sin_c = gamma.to_radians().sin();
    let cos_c = gamma.to_radians().cos();

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
    alpha: f32,
    beta: f32,
) -> f32 {
    let sin_a = alpha.to_radians().sin();
    let cos_a = alpha.to_radians().cos();
    let sin_b = beta.to_radians().sin();
    let cos_b = beta.to_radians().cos();

    k as f32 * cos_a * cos_b
        - j as f32 * sin_a * cos_b
        + i as f32 * sin_b
}

fn calculate_for_surface(
    cube_x: f32,
    cube_y: f32,
    cube_z: f32,
    ch: char,
    z_buffer: &mut [f32; CANVAS_WIDTH * CANVAS_HEIGHT],
    buffer: &mut [char; CANVAS_WIDTH * CANVAS_HEIGHT],
    alpha: f32,
    beta: f32,
    gamma: f32,
) {
    let x = calculate_x(cube_x, cube_y, cube_z, alpha, beta, gamma);
    let y = calculate_y(cube_x, cube_y, cube_z, alpha, beta, gamma);
    let z = calculate_z(cube_x, cube_y, cube_z, alpha, beta);
    let z = z + DISTANCE_FROM_CAMERA;

    // Inverse of z = 
    // this give us the idea of 'how far' the resulting point will be from the camera
    // bigger values => closer to camera, smaller values => further away...
    let ooz =  1.0 / z; 

    let xp = (CANVAS_WIDTH as f32 /2.0 + PROJECTION_SCALE * ooz * x * ASPECT_RATIO) as isize;
    let yp = (CANVAS_HEIGHT as f32 / 2.0 - PROJECTION_SCALE * ooz * y) as isize;

    // Out of canvas limits..
    if xp < 0 || (xp as usize) >= CANVAS_WIDTH {
        return;
    }
    if yp < 0 || (yp as usize) >= CANVAS_HEIGHT {
        return;
    }
        
    let idx = xp + yp * CANVAS_WIDTH as isize;
    let idx = idx as usize;
    if ooz > z_buffer[idx] {
        z_buffer[idx] = ooz;
        buffer[idx] = ch;
    }
}


fn main() {
    let mut z_buffer: [f32; CANVAS_WIDTH * CANVAS_HEIGHT] = [0.0; CANVAS_WIDTH * CANVAS_HEIGHT];
    let mut buffer: [char; CANVAS_WIDTH * CANVAS_HEIGHT] = [BACKGROUND_ASCII_CODE; CANVAS_WIDTH * CANVAS_HEIGHT];

    clear_screen();

    // Rotation values...
    let mut alpha: f32 = 0.0;
    let mut beta: f32 = 0.0;
    let mut gamma: f32 = 0.0;
    
    loop {
        // Clear screen and z buffers
        buffer.fill(BACKGROUND_ASCII_CODE);
        z_buffer.fill(0.0);
        
        let mut cube_x = -1.0 * HALF_CUBE_WIDTH as f32;
        while cube_x < HALF_CUBE_WIDTH as f32 {
            
            let mut cube_y = -1.0 * HALF_CUBE_WIDTH as f32;
            while cube_y < HALF_CUBE_WIDTH as f32 {
                // Plane F; Front side, We update X and Y, and keep Z constant 
                //       ______
                //     /      /|
                //    /______/ |
                //    |      | |
                //    |  F   | /
                //    |______|/
                //
                let x_value = cube_x;
                let y_value = cube_y;
                let z_value = -1.0 * (HALF_CUBE_WIDTH as f32);
                calculate_for_surface(x_value, y_value, z_value, 'F', &mut z_buffer, &mut buffer, alpha, beta, gamma);
                
                // //  Plane K; Back side, We update X and Y, and keep Z constant 
                // //        ______
                // //      /|  K   |
                // //     / |      |
                // //    |  |______|
                // //    | /      /
                // //    |/______/
                // //
                let x_value = cube_x;
                let y_value = cube_y;
                let z_value = 1.0 * (HALF_CUBE_WIDTH as f32);
                calculate_for_surface(x_value, y_value, z_value, 'K', &mut z_buffer, &mut buffer, alpha, beta, gamma);

                //  Plane L; Left side, we update Y and Z, and keep X constant
                //            ______
                //          /|      |
                //         / |      |
                // L -->  |  |______|
                //        | /      /
                //        |/______/
                //
                let x_value = -1.0 * (HALF_CUBE_WIDTH as f32);
                let y_value = cube_y;
                let z_value = cube_x;
                calculate_for_surface(x_value, y_value, z_value, 'L', &mut z_buffer, &mut buffer, alpha, beta, gamma);
                
                //  Plane R; Right side, we update Y and Z, and keep X constant
                //            ______
                //          /|      /|
                //         / |     / | <-- R
                //        |  |____|  |
                //        | /     | /
                //        |/______|/
                // 
                let x_value = 1.0 * (HALF_CUBE_WIDTH as f32);
                let y_value = cube_y;
                let z_value = cube_x;
                calculate_for_surface(x_value, y_value, z_value, 'R', &mut z_buffer, &mut buffer, alpha, beta, gamma);

                //  Plane B; Bottom, we update X and Z, and keep Y constant
                //            ______
                //         / |      |
                //        /  |      |
                //        |  |______|
                //        | /   B  /
                //        |/______/
                //
                let x_value = cube_x;
                let y_value = -1.0 * (HALF_CUBE_WIDTH as f32);
                let z_value = cube_y;
                calculate_for_surface(x_value, y_value, z_value, 'B', &mut z_buffer, &mut buffer, alpha, beta, gamma);
                
                //  Plane T; Top, we update X and Z, and keep Y constant
                //           ______
                //         /   T   /|
                //        /______ / | 
                //        |      |  |
                //        |      | /
                //        |______|/
                //
                let x_value = cube_x;
                let y_value = 1.0 * (HALF_CUBE_WIDTH as f32);
                let z_value = cube_y;
                calculate_for_surface(x_value, y_value, z_value, 'T', &mut z_buffer, &mut buffer, alpha, beta, gamma);

                cube_y += RESOLUTION_STEP;
            }
            
            cube_x += RESOLUTION_STEP;
        }
        
        print_buffer(buffer);

        thread::sleep(Duration::from_millis(16)); // 60 fps!
        
        alpha += 2.5;
        beta += 2.0;
        gamma += 1.5;
    }

}
