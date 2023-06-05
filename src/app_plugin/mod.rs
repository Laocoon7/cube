use std::io::Cursor;

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use winit::window::Icon;

use self::{default_plugins::default_plugins, external_plugins::external_plugins};

pub mod default_plugins;
pub mod external_plugins;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(default_plugins())
            .add_plugins(external_plugins())
            .add_system(set_window_icon.on_startup());
    }
}

fn set_window_icon(windows: NonSend<WinitWindows>, primary_windows: Query<Entity, With<PrimaryWindow>>) {
    let primary_entity = primary_windows.single();
    let primary = windows.get_window(primary_entity).expect("Failed to get the primary window");
    let icon_buf = Cursor::new(include_bytes!("../../assets/app/icon_256x256.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).expect("Failed to load window icon");
        primary.set_window_icon(Some(icon));
    }
}
