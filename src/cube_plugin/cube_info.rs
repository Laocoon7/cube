use bevy::prelude::*;

use super::traits::CubeId;

#[derive(Component)]
pub struct CubeInfo<Id: CubeId> {
    pub cube_id: Id,
    pub cube_dimension: u32,
}
