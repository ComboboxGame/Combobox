use crate::game::GameState;
use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ElevatorType {
    Loop { period: f32, current: f32 },
    WeightActivated { weight_to_activate: f32 },
}

#[derive(Component, Debug, Clone)]
pub struct Elevator {
    pub start: Vec2,
    pub end: Vec2,
    pub elevator_type: ElevatorType,
}

impl Elevator {
    pub const WIDTH: f32 = 100.0;
    pub const HEIGHT: f32 = 10.0;
}

pub struct ElevatorPlugin;

impl Plugin for ElevatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(update));
    }
}

fn update(
    mut elevators: Query<(Entity, &mut Transform, &mut Elevator)>,
    time: Res<Time>,
    context: Res<RapierContext>,
) {
    for (entity, mut transform, mut elevator) in elevators.iter_mut() {
        let mut anything_below = false;
        let mut anything_and_interacts = false;

        let intervals = 8;

        for i in 0..intervals {
            let origin = transform.translation.truncate()
                + Vec2::X * Elevator::WIDTH * ((i as f32 / (intervals - 1) as f32) - 0.5);
            //let query_filter = QueryFilter::new().groups(InteractionGroups::new(ELEVATOR_BIT, ELEVATOR_FILTER));

            let query_filter = QueryFilter::exclude_fixed().exclude_collider(entity);

            if let Some((_, v)) = context.cast_ray(
                origin,
                Vec2::NEG_Y,
                Elevator::HEIGHT * 0.7,
                true,
                query_filter,
            ) {
                anything_below = true;
                if v < Elevator::HEIGHT * 0.51 {
                    anything_and_interacts = true;
                }
                break;
            }
        }

        match &mut elevator.elevator_type {
            ElevatorType::Loop { period, current } => {
                *current += time.delta_seconds();
                let t = (*current / (*period * 0.5)) % 2.0;

                if t > 1.0 && anything_below {
                    // Don't move!
                    *current -= time.delta_seconds() * 1.0;

                    if anything_and_interacts {
                        *current -= time.delta_seconds() * 1.0;
                    }
                }
                let t = (*current / (*period * 0.5)) % 2.0;

                let t = ((if t < 1.0 { t } else { 2.0 - t } - 0.5) * 1.1 + 0.5).clamp(0.0, 1.0);
                let t = t * t * (3.0 - 2.0 * t);
                let p = elevator.start * (1.0 - t) + elevator.end * t;
                transform.translation.x = p.x;
                transform.translation.y = p.y;
            }
            ElevatorType::WeightActivated {
                weight_to_activate: _,
            } => {
                todo!()
            }
        }
    }
}
