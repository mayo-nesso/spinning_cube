# Rotating ASCII Cube in Rust

![Its alive](./imgs/its_alive.gif)

This little experiment is a simple Rust application that renders a rotating ASCII cube in the terminal.
The idea of doing this in Rust and with some colors popped into my mind after watching [YouTube video "ASCII Cube - An interactive 3D ASCII art animation"](https://www.youtube.com/watch?v=p09i_hoFdd0).

["A more detailed story about all of this..."](https://mayonesso.com/blog/2024/08/cube-projection-in-rust/)

## Features

- **3D Rotating Cube**: The application renders a 3D cube that rotates around its axes.
- **Terminal Output**: Displays the rotating cube directly in the terminal.
- **ASCII Art**: Uses ASCII characters to draw the faces of the cube, not sure if I can call that Art, but ¯\_(ツ)_/¯
- **Colors**: Who doesn't like colors!
- **Tweak Parameters in realtime**: Now using the keyboard you can tweak some parameters used.

## Prerequisites

To run this project, you'll need:

- Rust installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).

## Installation

i. Clone the repository:

   ```bash
   git clone git@github.com:mayo-nesso/spinning_cube.git
   cd rotating-ascii-cube
   ```

ii. Build the project:

   ```bash
   Copy code
   cargo build --release
   ```

## Usage

To run the application, simply execute:

   ```bash
   cargo run
   ```

You should see a rotating ASCII cube displayed in your terminal.

### Playing with parameters

Some parts of this program are 'easy' to understand, like the rotaton angle on one of the axis. But others are not that intuitive.
I added a sloppy menu to change those parameters in realtime, plus a reset command to go back to normal.

![Fooling around](./imgs/fooling_around.gif)

The options are:

- `a` : Turn on/off rotation over X axis
- `b` : Turn on/off rotation over Y axis
- `g` : Turn on/off rotation over Z axis

- `e`/`r`: increase/decrease X-axis rotation
- `d`/`f`: increase/decrease Y-axis rotation
- `c`/`v`: increase/decrease Z-axis rotation

- `u`/`j`: increase/decrease distance from camera
- `i`/`k`: increase/decrease projection scale
- `o`/`l`: increase/decrease resolution step

- `z`: reset
- `q`: quit

> It's interesting to see how, for instance, being too close to the cube can show the holes between the 'pixels', and reducing the resolution step can help to fix that.

## Notes

The cube rotates continuously with ~~adjustable~~ speed (hardcoded by now, but it could be nice to control the rotation with the keyboard).

Colors are applied to different faces of the cube for better visual distinction, where each face has its own 'Letter' (F = Front, K = Back, T = Top, B = Bottom, and so on...).

## Acknowledgments

Heavily inspired by the video ["ASCII Cube - An interactive 3D ASCII art animation"](https://www.youtube.com/watch?v=p09i_hoFdd0).

Rust conversion and some improvements (like the colors) made by me :P
