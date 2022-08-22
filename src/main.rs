#![allow(dead_code)]
// todo: do warn(dead_code) sometimes, too noisy during development

use bevy::{asset::AssetServerSettings, prelude::*};

use crate::game::GamePlugin;
#[cfg(debug_assertions)]
use crate::utils::FpsPlugin;

mod core;
mod game;
mod gui;
mod levels;
mod utils;

fn main() {
    let mut app = App::new();

    // Watch for file system changes (shaders, textures etc)
    app.insert_resource(AssetServerSettings {
        watch_for_changes: true,
        ..default()
    });

    // Default bevy plugins
    app.add_plugins(DefaultPlugins);

    // Custom hdr rendering plugin with post processing (bloom + tone mapping)
    #[cfg(not(debug_assertions))]
    app.add_plugin(post_processing::Core2dCustomPlugin);

    // Show fps in corner of the screen
    #[cfg(debug_assertions)]
    app.add_plugin(FpsPlugin);

    // Our incredible game plugin
    app.add_plugin(GamePlugin);

    app.run();
}
