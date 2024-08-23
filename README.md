# Rotating ASCII Cube in Rust

![Its alive](./imgs/its_alive.gif)

This little experiment is a simple Rust application that renders a rotating ASCII cube in the terminal.
The idea of doing this in Rust and with some colors popped it into my mind after watching [YouTube video "ASCII Cube - An interactive 3D ASCII art animation"](https://www.youtube.com/watch?v=p09i_hoFdd0).

## Features

- **3D Rotating Cube**: The application renders a 3D cube that rotates around its axes.
- **Terminal Output**: Displays the rotating cube directly in the terminal.
- **ASCII Art**: Uses ASCII characters to draw the faces of the cube, not sure if I can call that Art, but ¯\_(ツ)_/¯
- **Colors**: Who doesn't like colors!

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

## Notes

The cube rotates continuously with ~~adjustable~~ speed (hardcoded by now, but it could be nice to control the rotation with the keyboard).
Colors are applied to different faces of the cube for better visual distinction, each face has its own 'Letter' (F = Front, K = Back, T = Top, B = Bottom, and so on...).

## Acknowledgments

Heavily inspired by the video ["ASCII Cube - An interactive 3D ASCII art animation"](https://www.youtube.com/watch?v=p09i_hoFdd0).

Rust conversion and some improvements (like the colors) made by me :P
