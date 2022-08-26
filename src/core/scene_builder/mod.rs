use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::core::Material;
pub use boundaries::*;
pub use combobox::*;
pub use elevator::*;
pub use player::*;
pub use spawn_point::*;
pub use wall::*;

use super::BackgroundMusic;

mod boundaries;
mod combobox;
mod elevator;
mod player;
mod spawn_point;
mod wall;

pub struct SceneBuilder<'w, 's, 'a, 'b> {
    builder: &'b mut ChildBuilder<'w, 's, 'a>,
    meshes: &'b mut Assets<Mesh>,
    materials: &'b mut Assets<Material>,
    assets: &'b mut AssetServer,
    boundaries: &'b mut SceneBoundaries,
    wall_material: Handle<Material>,
    background_music: ResMut<'b, BackgroundMusic>,
}

impl<'w, 's, 'a, 'b> SceneBuilder<'w, 's, 'a, 'b> {
    pub fn new(
        builder: &'b mut ChildBuilder<'w, 's, 'a>,
        meshes: &'b mut Assets<Mesh>,
        materials: &'b mut Assets<Material>,
        boundaries: &'b mut SceneBoundaries,
        assets: &'b mut AssetServer,
        background_music: ResMut<'b, BackgroundMusic>,
    ) -> SceneBuilder<'w, 's, 'a, 'b> {
        let wall_material = materials.add(Color::rgb(0.1, 0.1, 0.1).into());

        SceneBuilder {
            builder,
            meshes,
            materials,
            wall_material,
            boundaries,
            assets,
            background_music,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.builder.spawn_bundle(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(self.meshes.add(Quad::new(Vec2::ONE * 10000.0).into())),
            material: self.materials.add(color.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -100.0)),
            ..default()
        });
    }

    pub fn set_audio(&mut self, name: &str) {
        self.background_music.0 = Some(name.to_string());
    }
}
