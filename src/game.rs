use crate::core::CorePlugin;
use crate::gui::GuiPlugin;
use crate::levels::LevelsPlugin;
use bevy::prelude::*;

#[cfg(debug_assertions)]
pub type Material = ColorMaterial;

#[cfg(not(debug_assertions))]
pub type Material = post_processing::ColorMaterialCustom;

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
        //app.add_state(GameState::MainMenu);
        app.add_state(GameState::Game); // todo: start from MainMenu
        app.add_startup_system(setup_camera);
        app.add_plugin(LevelsPlugin);
        app.add_plugin(CorePlugin);
        app.add_plugin(GuiPlugin);
    }
}

fn setup_camera(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    // Set global clear color
    clear_color.0 = Color::BLACK;

    commands.spawn_bundle(Camera2dBundle { ..default() });
}
