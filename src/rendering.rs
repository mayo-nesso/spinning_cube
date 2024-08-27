use once_cell::unsync::Lazy;
use std::collections::HashMap;
use colored::*;
use std::io::{stdout, Write};
use crate::cube_parameters::CubeParameters;
use crate::constants::*;

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

pub fn clear_screen() {
  // Clear the terminal screen
  print!("\x1b[2J");
  // Move cursor to home position (0,0) using ANSI escape code
  print!("\x1b[H");
  // Ensure the buffer is flushed immediately so that the command takes effect
  stdout().flush().unwrap();
}


pub fn print_vars(params: &CubeParameters) {
  print!("\n");
  println!("alpha : {:.2} \t\t\t(e/r: +/- | a: auto)", params.alpha);
  println!("beta  : {:.2} \t\t\t(d/f: +/- | b: auto)", params.beta);
  println!("gamma : {:.2} \t\t\t(c/v: +/- | g: auto)", params.gamma);
  print!("\n");
  println!("DISTANCE_FROM_CAMERA : {:.2} \t(u/j: +/-)", params.distance_from_camera);
  println!("PROJECTION_SCALE     : {:.2} \t(i/k: +/-)", params.projection_scale);
  println!("RESOLUTION_STEP      : {:.2} \t(o/l: +/-)", params.resolution_step);
  print!("\n");
  println!("z: reset");
  println!("q: quit");
}


pub fn print_buffer(buffer:[char; CANVAS_WIDTH * CANVAS_HEIGHT]) {
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

pub fn draw_cube(
    z_buffer: &mut [f32; CANVAS_WIDTH * CANVAS_HEIGHT],
    buffer: &mut [char; CANVAS_WIDTH * CANVAS_HEIGHT],
    params: &CubeParameters,
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
            calculate_for_surface(
                x_value, y_value, z_value,
                'F',
                z_buffer, buffer,
                params.alpha, params.beta, params.gamma,
                params.distance_from_camera, params.projection_scale);
            
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
            calculate_for_surface(
                x_value, y_value, z_value,
                'K',
                z_buffer, buffer,
                params.alpha, params.beta, params.gamma,
                params.distance_from_camera, params.projection_scale);

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
            calculate_for_surface(
                x_value, y_value, z_value,
                'L',
                z_buffer, buffer,
                params.alpha, params.beta, params.gamma,
                params.distance_from_camera, params.projection_scale);
            
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
            calculate_for_surface(
                x_value, y_value, z_value,
                'R',
                z_buffer, buffer,
                params.alpha, params.beta, params.gamma,
                params.distance_from_camera, params.projection_scale);

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
            calculate_for_surface(
                x_value, y_value, z_value,
                'B',
                z_buffer, buffer,
                params.alpha, params.beta, params.gamma,
                params.distance_from_camera, params.projection_scale);
            
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
            calculate_for_surface(
                x_value, y_value, z_value,
                'T',
                z_buffer, buffer,
                params.alpha, params.beta, params.gamma,
                params.distance_from_camera, params.projection_scale);

            cube_y += params.resolution_step;
        }
        
        cube_x += params.resolution_step;
    }
}
