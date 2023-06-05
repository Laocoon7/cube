use bevy::{ecs::system::SystemParam, prelude::*};

use super::{
    bundles::{CubeBundle, CubieBundle, CubieFaceBundle},
    cube::Cube,
    cube_materials::CubeMaterials,
    cube_meshes::CubeMeshes,
    cube_settings::CubeSettings,
    cube_side::CubeSide,
    cubie::Cubie,
};

#[derive(SystemParam)]
pub struct CubeBuilder<'w, 's> {
    commands: Commands<'w, 's>,

    materials: ResMut<'w, Assets<StandardMaterial>>,

    cube_meshes: Res<'w, CubeMeshes>,
}

impl<'w, 's> CubeBuilder<'w, 's> {
    pub fn spawn_cube(&mut self, cube_settings: &CubeSettings) -> Entity {
        let cube_entity = self.commands.spawn_empty().id();

        let mut cube = Cube::new(cube_settings.size);
        let cube_materials = CubeMaterials::new(&mut self.materials, &cube_settings.cube_colors);

        let mut sides = CubeSide::empty();
        for z in 0..cube_settings.size {
            sides &= !(CubeSide::FRONT | CubeSide::BACK);
            if z == 0 {
                sides |= CubeSide::BACK;
            }
            if z == cube_settings.size - 1 {
                sides |= CubeSide::FRONT;
            }
            for y in 0..cube_settings.size {
                sides &= !(CubeSide::UP | CubeSide::DOWN);
                if y == 0 {
                    sides |= CubeSide::DOWN;
                }
                if y == cube_settings.size - 1 {
                    sides |= CubeSide::UP;
                }
                for x in 0..cube_settings.size {
                    sides &= !(CubeSide::LEFT | CubeSide::RIGHT);
                    if x == 0 {
                        sides |= CubeSide::LEFT;
                    }
                    if x == cube_settings.size - 1 {
                        sides |= CubeSide::RIGHT;
                    }
                    if cube_settings.spawn_interior_cubies || !sides.is_empty() {
                        let position = Vec3::new(x as f32, y as f32, z as f32);
                        let cubie_entity = self.spawn_cubie(&cube_materials, position, sides);
                        cube.cubies.push(cubie_entity);
                        self.commands.entity(cube_entity).add_child(cubie_entity);
                    }
                }
            }
        }

        self.commands.entity(cube_entity).insert(CubeBundle {
            cube,
            spatial_bundle: SpatialBundle {
                transform: cube_settings.transform.clone(),
                ..Default::default()
            },
        });

        cube_entity
    }
}

impl<'w, 's> CubeBuilder<'w, 's> {
    fn spawn_cubie(&mut self, cube_materials: &CubeMaterials, position: Vec3, sides: CubeSide) -> Entity {
        let cubie_entity = self.commands.spawn_empty().id();

        let mut cubie = Cubie::default();

        let mut spawn_face = |mesh, material| -> Entity {
            let face_entity = self
                .commands
                .spawn(CubieFaceBundle {
                    pbr_bundle: PbrBundle {
                        mesh,
                        material,
                        ..Default::default()
                    },
                })
                .id();
            self.commands.entity(cubie_entity).add_child(face_entity);
            face_entity
        };

        if sides.contains(CubeSide::UP) {
            cubie.up = Some(spawn_face(self.cube_meshes.up(), cube_materials.up()));
        }

        if sides.contains(CubeSide::DOWN) {
            cubie.down = Some(spawn_face(self.cube_meshes.down(), cube_materials.down()));
        }

        if sides.contains(CubeSide::LEFT) {
            cubie.left = Some(spawn_face(self.cube_meshes.left(), cube_materials.left()));
        }

        if sides.contains(CubeSide::RIGHT) {
            cubie.right = Some(spawn_face(self.cube_meshes.right(), cube_materials.right()));
        }

        if sides.contains(CubeSide::FRONT) {
            cubie.front = Some(spawn_face(self.cube_meshes.front(), cube_materials.front()));
        }

        if sides.contains(CubeSide::BACK) {
            cubie.back = Some(spawn_face(self.cube_meshes.back(), cube_materials.back()));
        }

        self.commands.entity(cubie_entity).insert(CubieBundle {
            cubie,
            spatial_bundle: SpatialBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            },
        });

        cubie_entity
    }
}
