use crate::game::GameState;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ElevatorType {
    Loop { period: f32 },
    WeightActivated { weight_to_activate: f32 },
}

#[derive(Component, Debug, Clone)]
pub struct Elevator {
    pub start: Vec2,
    pub end: Vec2,
    pub elevator_type: ElevatorType,
}

pub struct ElevatorPlugin;

impl Plugin for ElevatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(update));
    }
}

fn update(mut elevators: Query<(&mut Transform, &Elevator)>, time: Res<Time>) {
    for (mut transform, elevator) in elevators.iter_mut() {
        match elevator.elevator_type {
            ElevatorType::Loop { period } => {
                let t = (time.seconds_since_startup() as f32 / (period * 0.5)) % 2.0;
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
