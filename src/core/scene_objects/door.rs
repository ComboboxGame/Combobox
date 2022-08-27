
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
}

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(LevelState::Level).with_system(update));
    }
}

fn update(
    mut doors: Query<(Entity, &mut Transform, &mut Door)>,
    buttons: Query<(&Transform, &DoorButton), Without<Door>>,
    time: Res<Time>,
    context: Res<RapierContext>,
) {
    let mut pressed_buttons = 0;

    for (transform, button) in buttons.iter() {
        if context
            .cast_ray(
                transform.translation.truncate(),
                Vec2::Y,
                1.0,
                true,
                QueryFilter::new(),
            )
            .is_some()
        {
            pressed_buttons |= button.mask;
        }
    }

    for (_entity, mut transform, mut door) in doors.iter_mut() {
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
            door.progress -= time.delta_seconds();
        }
        door.progress = door.progress.clamp(0.0, 1.0);

        let offset = door.direction.get_vec() * door.progress * door.height;
        transform.translation.x = offset.x;
        transform.translation.y = offset.y;
    }
}
