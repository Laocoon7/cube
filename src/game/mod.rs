use bevy::prelude::*;

use crate::{cube::prelude::*, gamestate::Gamestate};

const CUBE_SIZE: usize = 5;
const HALF_CUBE_SIZE: f32 = CUBE_SIZE as f32 * 0.5;
const CUBE_SCALE: f32 = 1.0;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            brightness: 1.0,
            ..Default::default()
        })
        .add_system(init_cube.in_schedule(OnEnter(Gamestate::Playing)));
    }
}

fn init_cube(mut cube_builder: CubeBuilder) {
    cube_builder.spawn_cube(&CubeSettings {
        size: CUBE_SIZE,
        cube_colors: CubeColors::japanese_white(),
        transform: Transform {
            translation: Vec3 {
                x: -HALF_CUBE_SIZE,
                y: -HALF_CUBE_SIZE,
                z: -HALF_CUBE_SIZE,
            },
            scale: Vec3::splat(CUBE_SCALE),
            ..Default::default()
        },
        spawn_interior_cubies: true,
    });
}
