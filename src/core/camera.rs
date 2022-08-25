use bevy::prelude::*;
use bevy::sprite::Rect;

use crate::core::{MapBoundaries, Player};
use crate::game::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game).with_system(update_camera_position),
        );
    }
}

fn get_view_rect(camera: &Camera, camera_transform: &Transform) -> Rect {
    let matrix = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    let max = matrix.project_point3(Vec2::ONE.extend(-1.0)).truncate();
    let min = matrix.project_point3(Vec2::NEG_ONE.extend(-1.0)).truncate();

    Rect {
        min: min.min(max),
        max: max.max(min),
    }
}

fn clamp_to_rect(pos: Vec2, view_half_size: Vec2, rect: Rect) -> Vec2 {
    [0, 1]
        .map(|i| {
            if rect.min[i] + view_half_size[i] >= rect.max[i] - view_half_size[i] {
                (rect.min[i] + rect.max[i]) * 0.5
            } else {
                pos[i].clamp(
                    rect.min[i] + view_half_size[i],
                    rect.max[i] - view_half_size[i],
                )
            }
        })
        .into()
}

fn update_camera_position(
    mut players: Query<&GlobalTransform, With<Player>>,
    mut cameras: Query<(&mut Transform, &Camera), (With<Camera2d>, Without<Player>)>,
    boundaries: Res<MapBoundaries>,
) {
    for player in players.iter_mut() {
        for (mut transform, camera) in cameras.iter_mut() {
            if let Some(view_range) = boundaries.view_range {
                let view_rect = get_view_rect(&camera, &transform);
                let view_size = (view_rect.max - view_rect.min) * 0.5;
                let zoom = if view_size.x > view_size.y {
                    view_range / view_size.y
                } else {
                    view_range / view_size.x
                };

                transform.scale *= zoom.powf(0.05);
            }

            let mut pos: Vec3 = player.translation() * 0.05 + transform.translation * 0.95;

            // Clamp camera to boundaries
            if let Some(boundaries) = boundaries.rect {
                let view_rect = get_view_rect(&camera, &transform);
                let view_size = (view_rect.max - view_rect.min) * 0.5;
                pos = clamp_to_rect(pos.truncate(), view_size, boundaries).extend(pos.z);
            }

            transform.translation = pos;
        }
    }
}
