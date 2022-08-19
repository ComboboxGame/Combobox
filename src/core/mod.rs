use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod combobox;
mod map;
mod player;

pub use combobox::*;
pub use map::*;
pub use player::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

        #[cfg(debug_assertions)]
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}
