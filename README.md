# Rust 3D Engine

Project made to improve my ability with Rust at the same time I 
learn about the intrinsics of graphic engines. It's developed using 
Winit to manage the GUI and the crate Pixels as the drawing buffer.
Everything else, aside from a few secondary tools, is made with pure
Rust.

## Features

* Rendering of 3D meshes - TBI
* Configuration for screen sizes, FOV and view distance - TBI
* Free movable camera - TBI

## Usage

TBI

## Architecture

TBI

## Dependencies

* [bresenham](https://crates.io/crates/bresenham/0.1.0). Provides the Bresenham's algorithm to generate lines between 
points. Developing it myself would have been just a time sink with little value as I already did it in the past.
* [env_logger](https://crates.io/crates/env_logger) and [log](https://crates.io/crates/log). Logging logic.
* [pixels](https://crates.io/crates/pixels). Pixel buffer to draw the pixels on the screen.
* [winit](https://crates.io/crates/winit) and [winit_input_helper](https://crates.io/crates/winit_input_helper). Window 
creation and event management.