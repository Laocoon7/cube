use bevy::prelude::*;

use super::{block_info::BlockInfo, cube_info::CubeInfo, traits::CubeId};

#[derive(Bundle)]
pub struct CubeBundle<Id: CubeId> {
    pub name: Name,
    pub cube_info: CubeInfo<Id>,
    pub spatial_bundle: SpatialBundle,
}

#[derive(Bundle)]
pub struct BlockBundle<Id: CubeId> {
    pub block_info: BlockInfo<Id>,
    pub pbr_bundle: PbrBundle,
}

#[derive(Bundle)]
pub struct ColorBundle {
    pub pbr_bundle: PbrBundle,
}
