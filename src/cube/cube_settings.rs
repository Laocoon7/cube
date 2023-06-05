use bevy::prelude::*;

use super::cube_colors::CubeColors;

#[derive(Clone)]
pub struct CubeSettings {
    pub size: usize,
    pub cube_colors: CubeColors,
    pub transform: Transform,
    pub spawn_interior_cubies: bool,
}

impl Default for CubeSettings {
    fn default() -> Self {
        Self {
            size: 3,
            cube_colors: CubeColors::default(),
            transform: Transform::default(),

            spawn_interior_cubies: true,
        }
    }
}
