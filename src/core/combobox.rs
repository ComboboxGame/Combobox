use bevy::prelude::*;

#[derive(Clone, Debug)]
pub enum ComboboxType {
    Standard { group: u32 },
    Undo,
    GravitySetter { direction: Vec2 },
    GravityActivator,
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
}
