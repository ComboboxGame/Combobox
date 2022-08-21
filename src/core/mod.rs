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

        //#[cfg(debug_assertions)]
        //app.add_plugin(RapierDebugRenderPlugin::default());
        app.insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., -9.8 * 50.),
            ..Default::default()
        });

        app.add_plugin(PlayerPlugin);
        app.add_plugin(ComboboxPlugin);

        app.add_system_to_stage(CoreStage::PreUpdate, clean_impulse);
    }
}

fn clean_impulse(mut impulses: Query<&mut ExternalImpulse>) {
    for mut i in impulses.iter_mut() {
        *i = ExternalImpulse::default();
    }
}
