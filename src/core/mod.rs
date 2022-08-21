use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod combobox;
mod elevator;
mod map;
mod player;

pub use combobox::*;
pub use elevator::*;
pub use map::*;
pub use player::*;

pub struct CorePlugin;

pub const G: f32 = 9.8 * 50.;

pub const WALL_BIT: u32 = 1 << 0;
pub const COMBOBOX_BIT: u32 = 1 << 1;
pub const PLAYER_BIT: u32 = 1 << 2;
pub const ELEVATOR_BIT: u32 = 1 << 3;

pub const WALL_FILTER: u32 = COMBOBOX_BIT | PLAYER_BIT;
pub const COMBOBOX_FILTER: u32 = WALL_BIT | PLAYER_BIT | ELEVATOR_BIT;
pub const PLAYER_FILTER: u32 = WALL_BIT | COMBOBOX_BIT | ELEVATOR_BIT;
pub const ELEVATOR_FILTER: u32 = PLAYER_BIT | COMBOBOX_BIT;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

        //#[cfg(debug_assertions)]
        //app.add_plugin(RapierDebugRenderPlugin::default());
        app.insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., -9.8 * 50.),
            ..Default::default()
        });

        app.init_resource::<MapBoundaries>();

        app.add_plugin(PlayerPlugin);
        app.add_plugin(ComboboxPlugin);
        app.add_plugin(ElevatorPlugin);

        app.add_system_to_stage(CoreStage::PreUpdate, clean_impulse);
    }
}

fn clean_impulse(mut impulses: Query<&mut ExternalImpulse>) {
    for mut i in impulses.iter_mut() {
        *i = ExternalImpulse::default();
    }
}
