use bevy::prelude::*;

use super::cube_colors::CubeColors;

#[derive(Clone)]
pub struct CubeMaterials {
    base: Handle<StandardMaterial>,
    up: Handle<StandardMaterial>,
    down: Handle<StandardMaterial>,
    left: Handle<StandardMaterial>,
    right: Handle<StandardMaterial>,
    front: Handle<StandardMaterial>,
    back: Handle<StandardMaterial>,
}

impl CubeMaterials {
    pub fn new(materials: &mut Assets<StandardMaterial>, colors: &CubeColors) -> Self {
        Self {
            base: materials.add(colors.base.into()),
            up: materials.add(colors.up.into()),
            down: materials.add(colors.down.into()),
            left: materials.add(colors.left.into()),
            right: materials.add(colors.right.into()),
            front: materials.add(colors.front.into()),
            back: materials.add(colors.back.into()),
        }
    }

    pub fn base(&self) -> Handle<StandardMaterial> { self.base.clone() }

    pub fn up(&self) -> Handle<StandardMaterial> { self.up.clone() }

    pub fn down(&self) -> Handle<StandardMaterial> { self.down.clone() }

    pub fn left(&self) -> Handle<StandardMaterial> { self.left.clone() }

    pub fn right(&self) -> Handle<StandardMaterial> { self.right.clone() }

    pub fn front(&self) -> Handle<StandardMaterial> { self.front.clone() }

    pub fn back(&self) -> Handle<StandardMaterial> { self.back.clone() }
}
