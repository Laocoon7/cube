use bevy::prelude::*;

use super::{side::Side, traits::CubeId};

#[derive(Component)]
pub struct BlockInfo<Id: CubeId> {
    pub cube_id: Id,
    pub sides: Side,
}
