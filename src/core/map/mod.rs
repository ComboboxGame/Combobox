use bevy::prelude::*;

pub use combobox::*;
pub use wall::*;

use crate::game::Material;

mod combobox;
mod wall;

pub struct MapBuilder<'w, 's, 'a, 'b> {
    builder: &'b mut ChildBuilder<'w, 's, 'a>,
    meshes: &'b mut Assets<Mesh>,
    materials: &'b mut Assets<Material>,

    wall_material: Handle<Material>,
}

impl<'w, 's, 'a, 'b> MapBuilder<'w, 's, 'a, 'b> {
    pub fn new(
        builder: &'b mut ChildBuilder<'w, 's, 'a>,
        meshes: &'b mut Assets<Mesh>,
        materials: &'b mut Assets<Material>,
    ) -> MapBuilder<'w, 's, 'a, 'b> {
        let wall_material = materials.add(Color::rgb_u8(255, 212, 120).into());

        MapBuilder {
            builder,
            meshes,
            materials,
            wall_material,
        }
    }
}
