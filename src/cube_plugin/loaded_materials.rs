use bevy::prelude::*;

use super::cube_colors::CubeColors;

#[derive(Resource)]
pub struct LoadedMaterials {
    base: Handle<StandardMaterial>,
    front: Handle<StandardMaterial>,
    back: Handle<StandardMaterial>,
    left: Handle<StandardMaterial>,
    right: Handle<StandardMaterial>,
    top: Handle<StandardMaterial>,
    bottom: Handle<StandardMaterial>,
}

impl FromWorld for LoadedMaterials {
    fn from_world(world: &mut World) -> Self {
        // extract the colors we want from our CubeSettings
        let settings = world.get_resource::<CubeColors>().expect("Failed to get CubeColors");
        let base_color = settings.base();
        let front_color = settings.front();
        let back_color = settings.back();
        let left_color = settings.left();
        let right_color = settings.right();
        let top_color = settings.top();
        let bottom_color = settings.bottom();

        // add the materials and keep the handles for each color side
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("Failed to get Assets<StandardMaterials>");
        let base = materials.add(base_color.into());
        let front = materials.add(front_color.into());
        let back = materials.add(back_color.into());
        let left = materials.add(left_color.into());
        let right = materials.add(right_color.into());
        let top = materials.add(top_color.into());
        let bottom = materials.add(bottom_color.into());

        Self {
            base,
            front,
            back,
            left,
            right,
            top,
            bottom,
        }
    }
}

impl LoadedMaterials {
    pub fn base(&self) -> Handle<StandardMaterial> { self.base.clone() }

    pub fn front(&self) -> Handle<StandardMaterial> { self.front.clone() }

    pub fn back(&self) -> Handle<StandardMaterial> { self.back.clone() }

    pub fn left(&self) -> Handle<StandardMaterial> { self.left.clone() }

    pub fn right(&self) -> Handle<StandardMaterial> { self.right.clone() }

    pub fn top(&self) -> Handle<StandardMaterial> { self.top.clone() }

    pub fn bottom(&self) -> Handle<StandardMaterial> { self.bottom.clone() }
}
