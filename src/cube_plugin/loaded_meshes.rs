use bevy::prelude::*;

#[derive(Resource)]
pub struct LoadedMeshes {
    cube: Handle<Mesh>,
    plane: Handle<Mesh>,
}

impl FromWorld for LoadedMeshes {
    fn from_world(world: &mut World) -> Self {
        // add the meshes and keep the handles
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().expect("Failed to get Assets<Mesh>");
        let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
        let plane = meshes.add(Mesh::from(shape::Plane {
            size: 0.9,
            subdivisions: 0,
        }));

        Self { cube, plane }
    }
}

impl LoadedMeshes {
    pub fn cube(&self) -> Handle<Mesh> { self.cube.clone() }

    pub fn plane(&self) -> Handle<Mesh> { self.plane.clone() }
}
