use std::marker::PhantomData;

use bevy::prelude::*;

use self::{
    cube_colors::CubeColors, cube_manager::CubeManager, loaded_materials::LoadedMaterials,
    loaded_meshes::LoadedMeshes, traits::CubeId,
};

pub mod cube_colors;
pub mod cube_info;
pub mod cube_manager;

pub mod block_info;

// pub mod loaded_cubes;
pub mod loaded_materials;
pub mod loaded_meshes;

pub mod side;

pub mod bundles;
pub mod traits;

pub struct CubePlugin<Id> {
    _phantom_id: PhantomData<Id>,
}
impl<Id: CubeId> Plugin for CubePlugin<Id> {
    fn build(&self, app: &mut App) {
        app.init_resource::<CubeColors>()
            .init_resource::<LoadedMaterials>()
            .init_resource::<LoadedMeshes>()
            .insert_resource(AmbientLight {
                color: Color::rgb(0.8, 0.8, 0.7),
                brightness: 1.0,
            })
            .add_system(init_cube.on_startup());
    }
}

impl<Id: Default> Default for CubePlugin<Id> {
    fn default() -> Self {
        Self {
            _phantom_id: PhantomData::default(),
        }
    }
}

const CUBE_DIMENSION: u32 = 5;
const CUBE_SCALE: f32 = 0.5;
const CUBE_POSITION: f32 = -CUBE_SCALE * ((CUBE_DIMENSION as f32 - 1.0) / 2.0);

fn init_cube(mut cube_manager: CubeManager<u32>) {
    cube_manager.spawn_cube(0, "Cube", CUBE_DIMENSION, Transform {
        translation: Vec3::new(CUBE_POSITION, CUBE_POSITION, CUBE_POSITION),
        scale: Vec3::new(CUBE_SCALE, CUBE_SCALE, CUBE_SCALE),
        ..Default::default()
    });
}
