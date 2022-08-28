use crate::core::collision_groups::ELEVATOR_I;
use crate::core::Material;
use crate::states::LevelState;
use crate::utils::SceneDirection;
use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Door {
    pub height: f32,
    pub direction: SceneDirection,
    pub progress: f32,
    pub pressed_mask: u32,
    pub not_pressed_mask: u32,
}

#[derive(Component, Debug, Clone)]
pub struct DoorButton {
    pub mask: u32,
    pub direction: SceneDirection,
    pub button_off: Handle<Material>,
    pub button_on: Handle<Material>,
    pub enabled: bool,
}

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(LevelState::Level).with_system(update));
    }
}

fn update(
    mut commands: Commands,
    mut doors: Query<(Entity, &mut Transform, &mut Door, &GlobalTransform)>,
    mut buttons: Query<(Entity, &Transform, &mut DoorButton), Without<Door>>,
    time: Res<Time>,
    context: Res<RapierContext>,
    materials: ResMut<Assets<Material>>,
) {
    let mut pressed_buttons = 0;

    for (entity, transform, mut button) in buttons.iter_mut() {
        let enabled = context
            .cast_ray(
                transform.translation.truncate() + button.direction.get_perp().get_vec() * 15.0,
                button.direction.get_vec(),
                10.0,
                true,
                QueryFilter::new(),
            )
            .is_some()
            || context
                .cast_ray(
                    transform.translation.truncate() - button.direction.get_perp().get_vec() * 15.0,
                    button.direction.get_vec(),
                    10.0,
                    true,
                    QueryFilter::new(),
                )
                .is_some();

        if enabled != button.enabled {
            button.enabled = enabled;
            commands.entity(entity).insert(if enabled {
                button.button_on.clone()
            } else {
                button.button_off.clone()
            });
        }

        if enabled {
            pressed_buttons |= button.mask;
        }
    }

    for (_entity, mut transform, mut door, g_transform) in doors.iter_mut() {
        let mut opening = true;

        if (door.pressed_mask & pressed_buttons) != door.pressed_mask {
            opening = false;
        }

        if (door.not_pressed_mask & pressed_buttons) != 0 {
            opening = false;
        }

        if opening {
            door.progress += time.delta_seconds();
        } else {
            if context
                .cast_ray(
                    g_transform.translation().truncate()
                        + door.direction.get_perp().get_vec() * 13.0,
                    door.direction.get_opposite().get_vec(),
                    door.height * 0.52,
                    true,
                    QueryFilter::new().groups(ELEVATOR_I),
                )
                .is_none()
                && context
                    .cast_ray(
                        g_transform.translation().truncate()
                            - door.direction.get_perp().get_vec() * 13.0,
                        door.direction.get_opposite().get_vec(),
                        door.height * 0.52,
                        true,
                        QueryFilter::new().groups(ELEVATOR_I),
                    )
                    .is_none()
            {
                door.progress -= time.delta_seconds();
            }
        }
        door.progress = door.progress.clamp(0.0, 1.0);

        let offset = door.direction.get_vec() * door.progress * door.height;
        transform.translation.x = offset.x;
        transform.translation.y = offset.y;
    }
}
