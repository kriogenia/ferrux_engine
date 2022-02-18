use crate::actors::mesh_actor::MeshActor;
use crate::geometry::Mesh;

pub fn get_3d_cube() -> MeshActor {
    let mesh = Mesh::try_from(
        "0 0 0,0 1 0,1 1 0;0 0 0,1 1 0,1 0 0;1 0 0,1 1 0,1 1 1;1 0 0,1 1 1,1 0 1;\
		1 0 1,1 1 1,0 1 1;1 0 1,0 1 1,0 0 1;0 0 1,0 1 1,0 1 0;0 0 1,0 1 0,0 0 0;\
		0 1 0,0 1 1,1 1 1;0 1 0,1 1 1,1 1 0;1 0 1,0 0 1,0 0 0;1 0 1,0 0 0,1 0 0"
            .to_string(),
    )
    .unwrap();
    MeshActor::new(mesh)
}
