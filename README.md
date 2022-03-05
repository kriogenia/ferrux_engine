# Ferrux Engine

Project made to improve my ability with Rust at the same time I 
learn about the intrinsics of graphic engines. It's developed using 
**Winit** to manage the GUI and the crate **Pixels** as the drawing buffer.
Everything else, aside from a few secondary tools, is made with pure
Rust.

## Features

* Rendering of 3D meshes
* Configuration for screen sizes, FOV and view distance
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

## Related libraries
During the development of this engine I made a couple of other libraries that started being
internal logic of this engine but I though that they could be useful as libraries for other
people.

* [ferrux_canvas](https://crates.io/crates/ferrux_canvas) contains the logic used to draw in
the winit window with the Pixels crate. It was made into its own library to allow its usage
with other projects that would also welcome easy drawing like this.
* [bresenham_zip](https://crates.io/crates/bresenham_zip), not really a direct dependency of
this project as this tool born create the drawing tools of the library above. It performs 
Bresenham's algorithm to draw two lines at the same time giving the points as vertical or horizontal
lines.

## Dependencies
* [env_logger](https://crates.io/crates/env_logger) and [log](https://crates.io/crates/log). Logging logic.
* [pixels](https://crates.io/crates/pixels). Pixel buffer to draw the pixels on the screen.
* [winit](https://crates.io/crates/winit) and [winit_input_helper](https://crates.io/crates/winit_input_helper). Window 
creation and event management.