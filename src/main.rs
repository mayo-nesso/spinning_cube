use std::io::{stdout, Write};
use std::{thread, time::Duration};
use colored::*;
use once_cell::unsync::Lazy;
use std::collections::HashMap;
use crossterm::{
    event::{self, Event, KeyCode},
};
use std::sync::mpsc;

const CUBE_WIDTH: usize = 25;
const HALF_CUBE_WIDTH: usize = (CUBE_WIDTH as f32 / 2.0) as usize;
const CANVAS_WIDTH: usize = 80;
const CANVAS_HEIGHT: usize = 40;
const ASPECT_RATIO: f32 = CANVAS_WIDTH as f32 / CANVAS_HEIGHT as f32;
const BACKGROUND_ASCII_CODE: char = ' ';

// DISTANCE_FROM_CAMERA:
// Should be at least, HALF_CUBE_WIDTH, since this is 
// the distance from the center to one of the cube's faces.
const DISTANCE_FROM_CAMERA: f32 = 53.0 + HALF_CUBE_WIDTH as f32;

// PROJECTION_SCALE:
// Scale up the screen.
// It seems like a good idea to have 
// into consideration the DISTANCE_FROM_CAMERA 
const PROJECTION_SCALE: f32 = DISTANCE_FROM_CAMERA / 2.0;

//
const RESOLUTION_STEP: f32 = 0.6;

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

fn print_vars(
    alpha: f32,
    beta: f32,
    gamma: f32,
    distance_from_camera: f32,
    projection_scale: f32,
    resolution_step: f32,
){
    print!("\n");
    println!("alpha : {:.2} \t\t\t(e/r: +/- | a: auto)", alpha);
    println!("beta  : {:.2} \t\t\t(d/f: +/- | b: auto)", beta);
    println!("gamma : {:.2} \t\t\t(c/v: +/- | g: auto)", gamma);
    print!("\n");
    println!("DISTANCE_FROM_CAMERA : {:.2} \t(u/j: +/-)", distance_from_camera);
    println!("PROJECTION_SCALE     : {:.2} \t(i/k: +/-)", projection_scale);
    println!("RESOLUTION_STEP      : {:.2} \t(o/l: +/-)", resolution_step);
    print!("\n");
    println!("z: reset");
    println!("q: quit");

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
    distance_from_camera: f32,
    projection_scale: f32,
) {
    let x = calculate_x(cube_x, cube_y, cube_z, alpha, beta, gamma);
    let y = calculate_y(cube_x, cube_y, cube_z, alpha, beta, gamma);
    let z = calculate_z(cube_x, cube_y, cube_z, alpha, beta);
    let z = z + distance_from_camera;

    // Inverse of z = 
    // this give us the idea of 'how far' the resulting point will be from the camera
    // bigger values => closer to camera, smaller values => further away...
    let ooz =  1.0 / z; 

    let xp = (CANVAS_WIDTH as f32 /2.0 + projection_scale * ooz * x * ASPECT_RATIO) as isize;
    let yp = (CANVAS_HEIGHT as f32 / 2.0 - projection_scale * ooz * y) as isize;

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

fn draw_cube(
    z_buffer: &mut [f32; CANVAS_WIDTH * CANVAS_HEIGHT],
    buffer: &mut [char; CANVAS_WIDTH * CANVAS_HEIGHT],
    alpha: f32,
    beta: f32,
    gamma: f32,
    distance_from_camera: f32,
    projection_scale: f32,
    resolution_step: f32,
) {
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
            calculate_for_surface(x_value, y_value, z_value, 'F', z_buffer, buffer, alpha, beta, gamma, distance_from_camera, projection_scale);
            
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
            calculate_for_surface(x_value, y_value, z_value, 'K', z_buffer, buffer, alpha, beta, gamma, distance_from_camera, projection_scale);

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
            calculate_for_surface(x_value, y_value, z_value, 'L', z_buffer, buffer, alpha, beta, gamma, distance_from_camera, projection_scale);
            
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
            calculate_for_surface(x_value, y_value, z_value, 'R', z_buffer, buffer, alpha, beta, gamma, distance_from_camera, projection_scale);

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
            calculate_for_surface(x_value, y_value, z_value, 'B', z_buffer, buffer, alpha, beta, gamma, distance_from_camera, projection_scale);
            
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
            calculate_for_surface(x_value, y_value, z_value, 'T', z_buffer, buffer, alpha, beta, gamma, distance_from_camera, projection_scale);

            cube_y += resolution_step;
        }
        
        cube_x += resolution_step;
    }
}

fn setup_input_listener() -> mpsc::Receiver<KeyCode> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            // Wait for key inputs...
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    tx.send(key_event.code).unwrap();
                }
            }
        }
    });

    rx
}

fn process_input(
    rx: &mpsc::Receiver<KeyCode>, 
    alpha: &mut f32,
    beta: &mut f32,
    gamma: &mut f32,
    auto_alpha: &mut f32,
    auto_beta: &mut f32,
    auto_gamma: &mut f32,
    distance_from_camera: &mut f32,
    projection_scale: &mut f32,
    resolution_step: &mut f32,
) {
    // TODO; + and - values are hardcoded...
    // TODO2; command pattern?
    if let Ok(key_code) = rx.try_recv() {
        match key_code {
            KeyCode::Char('a') => {
                *auto_alpha = 1.0 - *auto_alpha;
            }
            KeyCode::Char('e') => {
                *alpha += 2.5;
            }
            KeyCode::Char('r') => {
                *alpha -= 5.0;
            }

            KeyCode::Char('b') => {
                *auto_beta = 1.0 - *auto_beta;
            }
            KeyCode::Char('d') => {
                *beta += 2.5;
            }
            KeyCode::Char('f') => {
                *beta -= 5.0;
            }

            KeyCode::Char('g') => {
                *auto_gamma = 1.0 - *auto_gamma;
            }
            KeyCode::Char('c') => {
                *gamma += 2.5;
            }
            KeyCode::Char('v') => {
                *gamma -= 5.0;
            }

            KeyCode::Char('u') => {
                *distance_from_camera += 1.5;
            }
            KeyCode::Char('j') => {
                *distance_from_camera -= 1.0;
            }

            KeyCode::Char('i') => {
                *projection_scale += 1.5;
            }
            KeyCode::Char('k') => {
                *projection_scale -= 1.0;
            }

            KeyCode::Char('o') => {
                *resolution_step += 0.1;
            }
            KeyCode::Char('l') => {
                *resolution_step -= 0.1;
            }
            
            KeyCode::Char('z') => {
                *auto_alpha = 0.0;
                *auto_beta = 0.0;
                *auto_gamma = 0.0;

                *alpha = 0.0;
                *beta = 0.0;
                *gamma = 0.0;

                *distance_from_camera = DISTANCE_FROM_CAMERA;
                *projection_scale = PROJECTION_SCALE;
                *resolution_step = RESOLUTION_STEP;
            }

            KeyCode::Char('q') => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

fn main() {
    let rx = setup_input_listener();

    let mut z_buffer: [f32; CANVAS_WIDTH * CANVAS_HEIGHT] = [0.0; CANVAS_WIDTH * CANVAS_HEIGHT];
    let mut buffer: [char; CANVAS_WIDTH * CANVAS_HEIGHT] = [BACKGROUND_ASCII_CODE; CANVAS_WIDTH * CANVAS_HEIGHT];

    clear_screen();

    // Rotation values...
    let mut alpha: f32 = 0.0;
    let mut beta: f32 = 0.0;
    let mut gamma: f32 = 0.0;
    let mut auto_alpha: f32 = 0.0;
    let mut auto_beta: f32 = 0.0;
    let mut auto_gamma: f32 = 0.0;
    // rendering parameters
    let mut distance_from_camera: f32 = DISTANCE_FROM_CAMERA;
    let mut projection_scale: f32 = PROJECTION_SCALE;
    let mut resolution_step: f32 = RESOLUTION_STEP;
    
    loop {
        // Clear screen and z buffers
        buffer.fill(BACKGROUND_ASCII_CODE);
        z_buffer.fill(0.0);
        
        // 'draw' cube on buffer
        draw_cube(&mut z_buffer, &mut buffer, alpha, beta, gamma, distance_from_camera, projection_scale, resolution_step);
        
        // print buffer...
        print_buffer(buffer);
        print_vars(alpha, beta, gamma, distance_from_camera, projection_scale, resolution_step);
        
        thread::sleep(Duration::from_millis(16)); // 60 fps!
        
        process_input(&rx, &mut alpha, &mut beta, &mut gamma, 
            &mut auto_alpha, &mut auto_beta, &mut auto_gamma, 
            &mut distance_from_camera,
            &mut projection_scale,
            &mut resolution_step);

        alpha += 2.5 * auto_alpha;
        beta += 2.0 * auto_beta;
        gamma += 1.5 * auto_gamma;
    }
}
