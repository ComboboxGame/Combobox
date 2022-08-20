use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

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
            },
            (ComboboxType::Undo, _) => {
                if other.combined_from.len() == 0 {
                    return None;
                }

                return Some(other.combined_from.clone());
            },
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
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(combobox_system));
    }
}

fn combobox_system(
    mut commands: Commands,
    comboboxes: Query<(Entity, &Combobox, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
) {
    'outer: for (i, (a, combobox_a, transform_a)) in comboboxes.iter().enumerate() {
        for (j, (b, combobox_b, transform_b)) in comboboxes.iter().enumerate() {
            if i >= j {
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
                    commands.entity(a).despawn_recursive();
                    commands.entity(b).despawn_recursive();
                    break 'outer;
                }
            }
        }
    }
}
