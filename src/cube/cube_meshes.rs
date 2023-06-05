use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

#[derive(Resource)]
pub struct CubeMeshes {
    up: Handle<Mesh>,
    down: Handle<Mesh>,
    left: Handle<Mesh>,
    right: Handle<Mesh>,
    front: Handle<Mesh>,
    back: Handle<Mesh>,
}

impl FromWorld for CubeMeshes {
    fn from_world(world: &mut World) -> Self {
        let create_quad_mesh = |vertices: (Vec3, Vec3, Vec3, Vec3)| -> Mesh {
            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

            let (a, b, c, d) = vertices;
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![a, b, c, d]);

            mesh.set_indices(Some(Indices::U16(vec![0, 1, 2, 0, 2, 3])));

            mesh
        };

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().expect("Failed to get Assets<Mesh>");

        let up = create_quad_mesh((
            Vec3::new(-0.5, 0.5, 0.5),  // 0
            Vec3::new(0.5, 0.5, 0.5),   // 1
            Vec3::new(0.5, 0.5, -0.5),  // 2
            Vec3::new(-0.5, 0.5, -0.5), // 3
        ));

        let down = create_quad_mesh((
            Vec3::new(-0.5, -0.5, -0.5), // 0
            Vec3::new(0.5, -0.5, -0.5),  // 1
            Vec3::new(0.5, -0.5, 0.5),   // 2
            Vec3::new(-0.5, -0.5, 0.5),  // 3
        ));

        let left = create_quad_mesh((
            Vec3::new(-0.5, -0.5, -0.5), // 0
            Vec3::new(-0.5, -0.5, 0.5),  // 1
            Vec3::new(-0.5, 0.5, 0.5),   // 2
            Vec3::new(-0.5, 0.5, -0.5),  // 3
        ));

        let right = create_quad_mesh((
            Vec3::new(0.5, 0.5, -0.5),  // 0
            Vec3::new(0.5, 0.5, 0.5),   // 1
            Vec3::new(0.5, -0.5, 0.5),  // 2
            Vec3::new(0.5, -0.5, -0.5), // 3
        ));

        let front = create_quad_mesh((
            Vec3::new(-0.5, -0.5, 0.5), // 0
            Vec3::new(0.5, -0.5, 0.5),  // 1
            Vec3::new(0.5, 0.5, 0.5),   // 2
            Vec3::new(-0.5, 0.5, 0.5),  // 3
        ));

        let back = create_quad_mesh((
            Vec3::new(0.5, -0.5, -0.5),  // 0
            Vec3::new(-0.5, -0.5, -0.5), // 1
            Vec3::new(-0.5, 0.5, -0.5),  // 2
            Vec3::new(0.5, 0.5, -0.5),   // 3
        ));

        Self {
            up: meshes.add(up),
            down: meshes.add(down),
            left: meshes.add(left),
            right: meshes.add(right),
            front: meshes.add(front),
            back: meshes.add(back),
        }
    }
}

impl CubeMeshes {
    pub fn up(&self) -> Handle<Mesh> { self.up.clone() }

    pub fn down(&self) -> Handle<Mesh> { self.down.clone() }

    pub fn left(&self) -> Handle<Mesh> { self.left.clone() }

    pub fn right(&self) -> Handle<Mesh> { self.right.clone() }

    pub fn front(&self) -> Handle<Mesh> { self.front.clone() }

    pub fn back(&self) -> Handle<Mesh> { self.back.clone() }
}
