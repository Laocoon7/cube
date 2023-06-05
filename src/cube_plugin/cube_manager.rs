use std::{
    borrow::Cow,
    f32::consts::{FRAC_PI_2, PI},
};

use bevy::{ecs::system::SystemParam, prelude::*};

use super::{
    block_info::BlockInfo,
    bundles::{BlockBundle, ColorBundle, CubeBundle},
    cube_info::CubeInfo,
    loaded_materials::LoadedMaterials,
    loaded_meshes::LoadedMeshes,
    side::Side,
    traits::CubeId,
};

#[derive(SystemParam)]
pub struct CubeManager<'w, 's, Id: CubeId> {
    commands: Commands<'w, 's>,

    // loaded_cubes: Res<'w, LoadedCubes<Id>>,
    materials: Res<'w, LoadedMaterials>,
    meshes: Res<'w, LoadedMeshes>,

    q_cubes: Query<'w, 's, (Entity, &'static CubeInfo<Id>)>,
    q_blocks: Query<'w, 's, (Entity, &'static BlockInfo<Id>)>,
}

impl<'w, 's, Id: CubeId> CubeManager<'w, 's, Id> {
    pub fn spawn_cube(
        &mut self,
        id: Id,
        name: impl Into<Cow<'static, str>>,
        cube_dimension: u32,
        transform: Transform,
    ) -> Entity {
        let cube_entity = self
            .commands
            .spawn(CubeBundle {
                name: Name::new(name),
                cube_info: CubeInfo {
                    cube_id: id,
                    cube_dimension,
                },
                spatial_bundle: SpatialBundle {
                    transform,
                    ..Default::default()
                },
            })
            .id();

        let max = cube_dimension - 1;
        let mut sides = Side::empty();
        for z in 0..=max {
            sides &= !(Side::FRONT | Side::BACK);
            if z == 0 {
                sides |= Side::BACK;
            }
            if z == max {
                sides |= Side::FRONT;
            }
            for y in 0..=max {
                sides &= !(Side::TOP | Side::BOTTOM);
                if y == 0 {
                    sides |= Side::BOTTOM;
                }
                if y == max {
                    sides |= Side::TOP;
                }
                for x in 0..=max {
                    sides &= !(Side::LEFT | Side::RIGHT);
                    if x == 0 {
                        sides |= Side::LEFT;
                    }
                    if x == max {
                        sides |= Side::RIGHT;
                    }

                    if sides.is_empty() {
                        continue;
                    }

                    let current_position = Vec3::new(x as f32, y as f32, z as f32);

                    let block_entity = self.spawn_block(current_position, sides, id);

                    self.commands.entity(cube_entity).add_child(block_entity);
                }
            }
        }

        cube_entity
    }

    fn spawn_block(&mut self, position: Vec3, sides: Side, id: Id) -> Entity {
        let block_entity = self
            .commands
            .spawn(BlockBundle {
                block_info: BlockInfo { cube_id: id, sides },
                pbr_bundle: PbrBundle {
                    mesh: self.meshes.cube(),
                    material: self.materials.base(),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
            })
            .id();

        self.add_color(block_entity, sides);

        block_entity
    }

    fn add_color(&mut self, block_entity: Entity, sides: Side) {
        let mut spawn_color = |translation, rotate_x, rotate_z, material| {
            let mut transform = Transform::from_translation(translation);
            transform.rotate_x(rotate_x);
            transform.rotate_z(rotate_z);

            let color_entity = self
                .commands
                .spawn(ColorBundle {
                    pbr_bundle: PbrBundle {
                        mesh: self.meshes.plane(),
                        material,
                        transform,
                        ..Default::default()
                    },
                })
                .id();
            self.commands.entity(block_entity).add_child(color_entity);
        };

        if sides.contains(Side::FRONT) {
            spawn_color(
                Vec3::new(0.0, 0.0, 0.51),
                FRAC_PI_2,
                0.0,
                self.materials.front(),
            );
        }

        if sides.contains(Side::BACK) {
            spawn_color(
                Vec3::new(0.0, 0.0, -0.51),
                -FRAC_PI_2,
                0.0,
                self.materials.back(),
            );
        }

        if sides.contains(Side::LEFT) {
            spawn_color(
                Vec3::new(-0.51, 0.0, 0.0),
                0.0,
                FRAC_PI_2,
                self.materials.left(),
            );
        }

        if sides.contains(Side::RIGHT) {
            spawn_color(
                Vec3::new(0.51, 0.0, 0.0),
                0.0,
                -FRAC_PI_2,
                self.materials.right(),
            );
        }

        if sides.contains(Side::TOP) {
            spawn_color(Vec3::new(0.0, 0.51, 0.0), 0.0, 0.0, self.materials.top());
        }

        if sides.contains(Side::BOTTOM) {
            spawn_color(Vec3::new(0.0, -0.51, 0.0), PI, 0.0, self.materials.bottom());
        }
    }
}

impl<'w, 's, Id: CubeId> CubeManager<'w, 's, Id> {
    pub fn get_blocks_by_row(&self, id: Id) -> Vec<Entity> { Vec::new() }

    pub fn get_blocks_by_column(&self, id: Id) -> Vec<Entity> { Vec::new() }

    pub fn get_blocks_by_layer(&self, id: Id) -> Vec<Entity> { Vec::new() }

    pub fn get_blocks_on_side(&self, id: Id, side: Side) -> Vec<Entity> {
        let mut blocks = Vec::new();
        for (block_entity, block_info) in self.q_blocks.iter() {
            if block_info.cube_id == id {
                if block_info.sides.contains(side) {
                    blocks.push(block_entity);
                }
            }
        }

        blocks
    }
}

impl<'w, 's, Id: CubeId> CubeManager<'w, 's, Id> {
    fn get_cube(&self, id: Id) -> Option<Entity> {
        for (cube_entity, cube_info) in self.q_cubes.iter() {
            if cube_info.cube_id == id {
                return Some(cube_entity);
            }
        }
        None
    }

    fn get_blocks(&self, id: Id) -> Vec<Entity> {
        let mut blocks = Vec::new();
        for (block_entity, cube_group) in self.q_blocks.iter() {
            if cube_group.cube_id == id {
                blocks.push(block_entity);
            }
        }

        blocks
    }
}
