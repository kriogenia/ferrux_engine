# Rust 3D Engine

Project made to improve my ability with Rust at the same time I 
learn about the intrinsics of graphic engines. It's developed using 
Winit to manage the GUI and the crate Pixels as the drawing buffer.
Everything else, aside from a few secondary tools, is made with pure
Rust.

## Features

* Rendering of 3D meshes
* Configuration for screen sizes, FOV and view distance - TBI
* Free movable camera - TBI

## Usage

TBI

## Configuration

The class **EngineConfig** contains all the possible fields that you can
customize in the engine. To use it you can just build a new one with *default*
and then set all the properties as you like. Available properties:

* `title` of window. By default, "Rust 3D Engine".
* `width` of the window. By default, 960.
* `height` of the window. By default, 640.
* `fov` or field of view, in degrees. By default, 90.
* `screen_position` from the user. By default, 0.1.
* `view_limit`, max distance to render. By default, 1000.0
* `z_offset`, added distance in the Z-axis. By default, 3.0.

## Architecture

See the [class diagram](/uml/class.svg)

The basis of the program are the **EngineLoop** and the **Rust3DEngine**.
The first is a loop to run the engine on a different thread and manage
the different events. The Rust3DEngine on the other hand, contains all
the main components of the engine.

* First, the *Window* to render the engine and all the *inputs* to manage it.
* Second, the **EngineCanvas**, an abstraction over the *Pixels* library to 
manage the pixel buffer and with just basic drawing commands.
* And last, the **Environment**, the class holding the components in the scene.
It contains and manages all the actors that will be rendered.

There are two main traits to build components to draw. The first one is
**Drawable** to define structs capable of being drawn in the screen. And
the other are the **Actor**, which are Drawable entities that can also be
updated. An implementation of this one is the **MeshActor**, an actor with a Mesh.

The **Mesh** defines a group of **Triangles** to draw. 
Each triangle contains three different three-dimensional **Points**.
Triangles and points implement **Projectable**, a trait for those
entities that can be projected into a 2D space.

## Dependencies

* [bresenham](https://crates.io/crates/bresenham/0.1.0). Provides the Bresenham's algorithm to generate lines between 
points. Developing it myself would have been just a time sink with little value as I already did it in the past.
* [env_logger](https://crates.io/crates/env_logger) and [log](https://crates.io/crates/log). Logging logic.
* [pixels](https://crates.io/crates/pixels). Pixel buffer to draw the pixels on the screen.
* [winit](https://crates.io/crates/winit) and [winit_input_helper](https://crates.io/crates/winit_input_helper). Window 
creation and event management.