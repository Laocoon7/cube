use app_plugin::AppPlugin;
use bevy::prelude::*;
use camera_plugin::CameraPlugin;
use cube::CubePlugin;
use game::GamePlugin;
use gamestate::Gamestate;

pub mod gamestate;
pub mod globals;

pub mod app_plugin;
pub mod camera_plugin;

pub mod cube;
pub mod game;

fn main() {
    App::new()
        .add_state::<Gamestate>()
        .add_plugin(AppPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(CubePlugin)
        .add_plugin(GamePlugin)
        .run();
}
