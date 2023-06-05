use bevy::{
    core_pipeline::{bloom::BloomSettings, clear_color::ClearColorConfig},
    prelude::*,
};

use self::{
    fly::{fly_camera, FlyCamera},
    game_camera::GameCamera,
};
use crate::gamestate::Gamestate;

pub mod fly;
pub mod game_camera;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_camera.on_startup()).add_system(fly_camera.in_set(OnUpdate(Gamestate::Playing)));
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                // viewport: ,
                // order: ,
                // is_active: ,
                // computed: ,
                // target: ,
                hdr: true,
                // output_mode: ,
                // msaa_writeback: ,
                ..Default::default()
            },
            // camera_render_graph,
            // projection,
            // visible_entities,
            // frustum,
            transform: Transform::from_xyz(-2.0, 2.5, 5.0),
            // global_transform,
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            // tonemapping: Tonemapping::TonyMcMapface,
            // dither,
            // color_grading,
            ..Default::default()
        },
        BloomSettings::default(),
        GameCamera,
        FlyCamera::default(),
    ));
}
