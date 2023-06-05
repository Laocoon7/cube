use bevy::app::{NoopPluginGroup, PluginGroupBuilder};
use bevy_egui::EguiPlugin;

pub(super) fn external_plugins() -> PluginGroupBuilder {
    let external_plugins = PluginGroupBuilder::start::<NoopPluginGroup>()
        // .add(TilemapPlugin)
        .add(EguiPlugin);

    // #[cfg(feature = "dev")]
    // let external_plugins = external_plugins.add(RapierDebugRenderPlugin::default());

    external_plugins
}
