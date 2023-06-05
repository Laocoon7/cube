use bevy::prelude::*;

use self::cube_meshes::CubeMeshes;

pub mod cube;
pub mod cubie;

pub mod cube_builder;

pub mod cube_colors;
pub mod cube_materials;
pub mod cube_meshes;
pub mod cube_settings;
pub mod cube_side;

pub mod bundles;

pub mod prelude {
    pub use super::{cube_builder::CubeBuilder, cube_colors::CubeColors, cube_settings::CubeSettings};
}

pub struct CubePlugin;
impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) { app.init_resource::<CubeMeshes>(); }
}
