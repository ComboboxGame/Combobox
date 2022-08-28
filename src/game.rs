use bevy::prelude::*;

use bevy::render::RenderApp;
use post_processing::AmbientLight;
use std::env;

use crate::core::CorePlugin;
use crate::gui::GuiPlugin;
use crate::levels::LevelPlugin;
use crate::states::{AudioState, CameraState, GuiState, LevelState};

#[derive(Debug, Clone)]
pub struct ComboboxGamePlugin;

impl Plugin for ComboboxGamePlugin {
    fn build(&self, app: &mut App) {
        if env::var("LOCAL_BUILD") == Ok("2".to_string()) {
            app.insert_resource(Msaa { samples: 4 });
            app.add_state(GuiState::Level);
            app.add_state(AudioState::Level);
            app.add_state(LevelState::Level);
            app.add_state(CameraState::FollowPlayers);
        } else {
            app.insert_resource(Msaa { samples: 1 });
            app.add_state(GuiState::MainScreen);
            app.add_state(AudioState::None);
            app.add_state(LevelState::None);
            app.add_state(CameraState::None);
        }

        app.insert_resource(AmbientLight {
            color: Color::WHITE * 30.0,
        });

        app.add_startup_system(setup_camera);
        app.add_plugin(LevelPlugin);
        app.add_plugin(CorePlugin);
        app.add_plugin(GuiPlugin);

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };
        render_app.insert_resource(Msaa { samples: 1 });
    }
}

fn setup_camera(mut commands: Commands, _clear_color: ResMut<ClearColor>) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::default().with_scale(Vec3::splat(1.0)),
        ..default()
    });
}
