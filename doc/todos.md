# TODOs

List of possible implementations than can be considered to be added
but have been put apart for the moment:

## Extract the canvas a library
The canvas is a nice feature of the project that reduces the burden
of writing the pixels in the screen. I would like to make some new
projects that will probably take some advantage of those functions
so a possible future will take out all the screen drawing staff out
of the project into its own basic library. Importing and using that
as a dependency of this project.

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

## Naming
As it's possible that I end up publishing this tools I think that it's
a good idea to start thinking about some naming instead of "rust-3d-engine".
I'm giving a look to the *ferruxe* word that we have in galician, which means: *Rust*.
Yeah, it's kinda similar to **Ferris** as both of them come from the same 
latin root, I think.