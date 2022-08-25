use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod combobox;
mod direction;
mod elevator;
mod input;
mod map_builder;
mod material;
mod player;

pub use camera::*;
pub use combobox::*;
pub use direction::*;
pub use elevator::*;
pub use input::*;
pub use map_builder::*;
pub use material::*;
pub use player::*;

pub struct CorePlugin;

pub const WALL_BIT: u32 = 1 << 0;
pub const COMBOBOX_BIT: u32 = 1 << 1;
pub const PLAYER_BIT: u32 = 1 << 2;
pub const ELEVATOR_BIT: u32 = 1 << 3;

pub const WALL_FILTER: u32 = COMBOBOX_BIT | PLAYER_BIT;
pub const COMBOBOX_FILTER: u32 = WALL_BIT | PLAYER_BIT | ELEVATOR_BIT | COMBOBOX_BIT;
pub const PLAYER_FILTER: u32 = WALL_BIT | COMBOBOX_BIT | ELEVATOR_BIT | PLAYER_BIT;
pub const ELEVATOR_FILTER: u32 = PLAYER_BIT | COMBOBOX_BIT;

pub const GRAVITY: f32 = 9.8 * 100.;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

        #[cfg(debug_assertions)]
        app.add_plugin(RapierDebugRenderPlugin::default());

        app.insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., -GRAVITY),
            ..Default::default()
        });

        app.init_resource::<MapBoundaries>();

        app.add_plugin(CameraPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(ComboboxPlugin);
        app.add_plugin(ElevatorPlugin);

        app.add_system_to_stage(CoreStage::PreUpdate, clean_impulse);
        app.add_system(move_finish_arrow);
    }
}

fn clean_impulse(mut impulses: Query<&mut ExternalImpulse>) {
    for mut i in impulses.iter_mut() {
        *i = ExternalImpulse::default();
    }
}

fn move_finish_arrow(mut query: Query<&mut Transform, With<FinishPointArrow>>, time: Res<Time>) {
    for mut arrow in query.iter_mut() {
        arrow.translation.y = ((((time.seconds_since_startup() as f32 * 4.0).sin() + 1.0) * 0.5).powf(1.2) * 2.0 - 1.0) * 15.0;
    }
}
