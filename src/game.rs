use bevy::prelude::*;

use bevy::render::RenderApp;
use std::env;

use crate::core::CorePlugin;
use crate::gui::GuiPlugin;
use crate::levels::LevelsPlugin;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    // Menu states
    MainMenu,
    LevelMenu,

    // Game states
    Game,
}

#[derive(Debug, Clone)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        if env::var("LOCAL_BUILD") == Ok("1".to_string()) {
            app.add_state(GameState::MainMenu);
        } else {
            app.add_state(GameState::MainMenu);
        }
        app.add_startup_system(setup_camera);
        app.add_plugin(LevelsPlugin);
        app.add_plugin(CorePlugin);
        app.add_plugin(GuiPlugin);

        // Disable msaa todo: enable by default
        app.insert_resource(Msaa { samples: 4 });
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
