use bevy::prelude::*;

use crate::gui::buttons::ButtonsPlugin;
use crate::gui::credits_menu::CreditsGUIPlugin;
use crate::gui::game_menu::GameMenuPlugin;
use crate::gui::level_completed_menu::LevelCompleteGUIPlugin;
use crate::gui::level_menu::LevelSelectionGUIPlugin;
use crate::gui::main_menu::MainScreenGUIPlugin;

mod buttons;
mod game_menu;
mod level_completed_menu;
mod level_menu;
mod main_menu;
mod credits_menu;

#[derive(Debug, Clone)]
pub struct GuiPlugin;

pub const TRANSPARENT_COLOR: UiColor = UiColor(Color::rgba(0.0, 0.0, 0.0, 0.0));

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ButtonsPlugin);
        app.add_plugin(MainScreenGUIPlugin);
        app.add_plugin(LevelSelectionGUIPlugin);
        app.add_plugin(GameMenuPlugin);
        app.add_plugin(LevelCompleteGUIPlugin);
        app.add_plugin(CreditsGUIPlugin);
    }
}
