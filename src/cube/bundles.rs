use bevy::prelude::*;

use super::{cube::Cube, cubie::Cubie};

#[derive(Bundle)]
pub struct CubeBundle {
    pub cube: Cube,
    pub spatial_bundle: SpatialBundle,
}

#[derive(Bundle)]
pub struct CubieBundle {
    pub cubie: Cubie,
    pub spatial_bundle: SpatialBundle,
}

#[derive(Bundle)]
pub struct CubieFaceBundle {
    pub pbr_bundle: PbrBundle,
}
