use bevy::{
    app::PluginGroupBuilder,
    log::{Level, LogPlugin},
    prelude::*,
    window::PresentMode,
};

use crate::globals::{WINDOW_MAX_RESOLUTION, WINDOW_MIN_RESOLUTION, WINDOW_STARTUP_RESOLUTION, WINDOW_TITLE};

pub(super) fn default_plugins() -> PluginGroupBuilder {
    let default_plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_STARTUP_RESOLUTION.into(),
                resize_constraints: WindowResizeConstraints {
                    min_width: WINDOW_MIN_RESOLUTION.0,
                    min_height: WINDOW_MIN_RESOLUTION.1,
                    max_width: WINDOW_MAX_RESOLUTION.0,
                    max_height: WINDOW_MAX_RESOLUTION.1,
                },
                position: WindowPosition::At(IVec2::ZERO),
                title: WINDOW_TITLE.to_string(),
                present_mode: PresentMode::AutoVsync,
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(log_plugin());

    default_plugins
}

fn log_plugin() -> LogPlugin {
    let log_level = log_level();
    LogPlugin {
        filter: log_filter(log_level).to_string(),
        level: log_level,
    }
}

const fn log_level() -> Level {
    if cfg!(feature = "tracing") {
        Level::TRACE
    } else if cfg!(feature = "dev") {
        Level::INFO
    } else {
        Level::ERROR
    }
}

const fn log_filter(log_level: Level) -> &'static str {
    match log_level {
        Level::INFO => {
            "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,\
             bevy_render::render_resource::pipeline_cache=warn,big_brain=debug,sequence=debug,naga=warn"
        },
        Level::TRACE | Level::DEBUG => {
            "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,\
             bevy_render::render_resource::pipeline_cache=warn,bevy_app=debug,big_brain=debug,sequence=debug,\
             naga=warn"
        },
        _ => "",
    }
}
