use bevy::prelude::*;

pub use wall::*;

use crate::game::Material;

mod wall;

pub struct MapBuilder<'a> {
    meshes: &'a mut Assets<Mesh>,
    materials: &'a mut Assets<Material>,

    wall_material: Handle<Material>,
}

impl<'a> MapBuilder<'a> {
    pub fn new(
        meshes: &'a mut Assets<Mesh>,
        materials: &'a mut Assets<Material>,
    ) -> MapBuilder<'a> {
        let wall_material = materials.add(Color::rgb_u8(255, 212, 120).into());

        MapBuilder {
            meshes,
            materials,
            wall_material,
        }
    }
}
