use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub use combobox::*;
pub use door::*;
pub use elevator::*;
pub use player::*;

use crate::core::{FinishPoint, FinishPointArrow, Hint, Material};
use crate::states::LevelState;
use crate::utils::SceneDirection;

pub mod collision_groups;
mod combobox;
mod door;
mod elevator;
mod player;

pub const GRAVITY_FORCE: f32 = 9.8 * 100.;

pub struct SceneObjectsPlugin;

impl Plugin for SceneObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin);
        app.add_plugin(ComboboxPlugin);
        app.add_plugin(ElevatorPlugin);
        app.add_plugin(DoorPlugin);

        app.add_system_to_stage(CoreStage::PreUpdate, clean_impulse);
        app.add_system_set(SystemSet::on_update(LevelState::Level).with_system(move_finish_arrow));
        app.add_system(show_hints);
    }
}

fn clean_impulse(mut impulses: Query<&mut ExternalImpulse>) {
    for mut i in impulses.iter_mut() {
        *i = ExternalImpulse::default();
    }
}

fn move_finish_arrow(
    mut query: Query<&mut Transform, (With<FinishPointArrow>, Without<FinishPoint>)>,
    mut finish: Query<&mut Transform, (With<FinishPoint>, Without<FinishPointArrow>)>,
    time: Res<Time>,
    config: Res<RapierConfiguration>,
) {
    let gravity_direction = SceneDirection::from_gravity_direction(&config);

    for mut finish in finish.iter_mut() {
        finish.rotation = Quat::from_rotation_arc_2d(Vec2::NEG_Y, gravity_direction.get_vec());
    }

    for mut arrow in query.iter_mut() {
        arrow.translation.y =
            ((((time.seconds_since_startup() as f32 * 4.0).sin() + 1.0) * 0.5).powf(1.2) * 2.0
                - 1.0)
                * 15.0;
    }
}

fn show_hints(
    mut query: Query<(&GlobalTransform, &Handle<Material>), With<Hint>>,
    players: Query<&GlobalTransform, With<Player>>,
    mut materials: ResMut<Assets<Material>>,
) {
    for (t, m) in query.iter() {
        let mut opacity: f32 = 0.0;
        for pt in players.iter() {
            let l: f32 = (t.translation() - pt.translation()).truncate().length();

            let x = ((l - 100.0) / 80.0).max(0.0);
            opacity = opacity.max((-x * x * 1.0).exp() - 0.005);
        }

        if let Some(material) = materials.get_mut(m) {
            material.color = Color::rgba(1.0, 1.0, 1.0, opacity);
        }
    }
}
