# TODOs

List of possible implementations than can be considered to be added
but have been put apart for the moment:

## Draw the whole mesh
Keep the ability to draw the whole mesh even if the face is hidden. 
Currently, it's a feature removed in the drawing logic of the MeshActor
with the Z-normal check. 

If the implementation is decided, the idea is to delegate the drawing 
logic, and the Z-normal check extract it a wrapper of it. 
This way, on bootstrap time we could generate the actors with the desired 
drawing strategy evading constant condition checks.

## Publish as library
Right now, excepting the mesh load, the library works perfectly without
the main function, so it could be published as a library.
I'm currently considering publishing it in crates.io as a way of learning
to do it.

## Improve error handling
Attemping to speed up the development I left the error management in a very improvable state.
The idea is to swap errors to use the `this_err` crate to start learning it and help ease the fix. 

## Change to point referencing
The current state copies the points into the triangles, so two triangle sharing a point are not really doing it, they just have two vector with the same value. Change the mesh to have a collection of points and build the triangles as a collection of three vector references.