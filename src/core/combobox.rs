use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::ComboboxBundle;
use crate::game::{GameState, Material};

#[derive(Clone, Debug)]
pub enum ComboboxType {
    Standard { group: u32 },
    Buf,
    Undo,
    Direction { direction: Vec2 },
    Gravity,
    Lamp { color: Color },
}

#[derive(Clone, Debug, PartialEq, Component)]
pub enum ComboboxState {
    Normal,
    SpawningAnimation(f32),
    DespawningAnimation(f32),
    Despawned,
}

pub const SPAWN_TIME: f32 = 0.15;
pub const DESPAWN_TIME: f32 = 0.15;

#[derive(Component, Clone, Debug)]
pub struct Combobox {
    pub size: f32,
    pub box_type: ComboboxType,
    pub combined_from: Vec<Combobox>,
}

impl Combobox {
    pub fn new(size: f32, box_type: ComboboxType) -> Combobox {
        Self {
            size,
            box_type,
            combined_from: vec![],
        }
    }

    pub fn merge(&self, other: &Combobox) -> Option<Vec<Combobox>> {
        match (&self.box_type, &other.box_type) {
            (
                ComboboxType::Standard { group: group1 },
                ComboboxType::Standard { group: group2 },
            ) => {
                if group1 != group2 {
                    return None;
                }

                let big_box = Combobox {
                    size: f32::sqrt(self.size.powf(2.) + other.size.powf(2.)),
                    box_type: self.box_type.clone(),
                    combined_from: vec![self.clone(), other.clone()],
                };

                return Some(vec![big_box]);
            }
            (ComboboxType::Standard { .. }, ComboboxType::Buf) => other.merge(self),
            (ComboboxType::Buf, ComboboxType::Standard { .. }) => {
                let buffed_box = Combobox {
                    size: other.size * 2.,
                    box_type: other.box_type.clone(),
                    combined_from: vec![self.clone(), other.clone()],
                };

                return Some(vec![buffed_box]);
            }
            (ComboboxType::Undo, _) => {
                if other.combined_from.len() == 0 {
                    return None;
                }

                return Some(other.combined_from.clone());
            }
            (_, ComboboxType::Undo) => {
                if self.combined_from.len() == 0 {
                    return None;
                }

                return Some(self.combined_from.clone());
            }
            (_, _) => None,
        }
    }
}

pub struct ComboboxPlugin;

impl Plugin for ComboboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(merge));
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(animation));
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(despawn));
    }
}

fn animation(
    mut commands: Commands,
    mut comboboxes: Query<(Entity, &mut ComboboxState, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut combobox_state, mut transform) in comboboxes.iter_mut() {
        let mut new_state = None;

        if let ComboboxState::SpawningAnimation(animation_time) = &mut *combobox_state {
            *animation_time += time.delta_seconds();
            transform.scale = Vec3::ONE * (*animation_time / SPAWN_TIME).clamp(0.01, 1.0);

            if *animation_time >= SPAWN_TIME {
                new_state = Some(ComboboxState::Normal);
                commands.entity(entity).insert(RigidBody::Dynamic);
            }
        }
        if let ComboboxState::DespawningAnimation(animation_time) = &mut *combobox_state {
            *animation_time += time.delta_seconds();
            transform.scale = Vec3::ONE * (1.0 - *animation_time / SPAWN_TIME).clamp(0.01, 1.0);

            if *animation_time >= DESPAWN_TIME {
                new_state = Some(ComboboxState::Despawned)
            }
        }

        if let Some(new_state) = new_state {
            *combobox_state = new_state;
        }
    }
}

fn despawn(mut commands: Commands, mut comboboxes: Query<(Entity, &ComboboxState)>) {
    for (entity, combobox_state) in comboboxes.iter_mut() {
        if *combobox_state == ComboboxState::Despawned {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn merge(
    mut commands: Commands,
    comboboxes: Query<(Entity, &Combobox, &Transform, &ComboboxState)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
) {
    'outer: for (i, (a, combobox_a, transform_a, state_a)) in comboboxes.iter().enumerate() {
        for (j, (b, combobox_b, transform_b, state_b)) in comboboxes.iter().enumerate() {
            if i >= j {
                continue;
            }

            if *state_a != ComboboxState::Normal || *state_b != ComboboxState::Normal {
                continue;
            }

            if (transform_a.translation - transform_b.translation)
                .abs()
                .max_element()
                < (combobox_a.size + combobox_b.size) * 0.55
            {
                if let Some(merge) = combobox_a.merge(combobox_b) {
                    for combobox_new in merge {
                        let position_new =
                            (transform_a.translation + transform_b.translation).xy() * 0.5;
                        commands.spawn_bundle(ComboboxBundle::new(
                            combobox_new,
                            position_new,
                            &mut meshes,
                            &mut materials,
                        ));
                    }
                    commands
                        .entity(a)
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(ComboboxState::DespawningAnimation(0.0));
                    commands
                        .entity(b)
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(ComboboxState::DespawningAnimation(0.0));
                    break 'outer;
                }
            }
        }
    }
}
