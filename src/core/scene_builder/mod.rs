use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use post_processing::AmbientLight;

use crate::core::Material;
pub use boundaries::*;
pub use combobox::*;
pub use door::*;
pub use elevator::*;
pub use player::*;
pub use spawn_point::*;
pub use wall::*;

use super::BackgroundMusic;

mod boundaries;
mod combobox;
mod door;
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
    ambient_light: ResMut<'b, AmbientLight>,
    button_on: Handle<Material>,
    button_off: Handle<Material>,
}

impl<'w, 's, 'a, 'b> SceneBuilder<'w, 's, 'a, 'b> {
    pub const BACKGROUND_DEPTH: f32 = -2.9;
    pub const WALL_DEPTH: f32 = -0.8;
    pub const ELEVATOR_DEPTH: f32 = -0.7;
    pub const DOOR_DEPTH: f32 = -0.6;
    pub const PLAYER_DEPTH: f32 = -0.2;
    pub const BOX_DEPTH: f32 = -0.1;

    pub const CELL_SIZE: f32 = 50.0;

    pub fn new(
        builder: &'b mut ChildBuilder<'w, 's, 'a>,
        meshes: &'b mut Assets<Mesh>,
        materials: &'b mut Assets<Material>,
        boundaries: &'b mut SceneBoundaries,
        assets: &'b mut AssetServer,
        background_music: ResMut<'b, BackgroundMusic>,
        mut ambient_light: ResMut<'b, AmbientLight>,
    ) -> SceneBuilder<'w, 's, 'a, 'b> {
        let wall_material = materials.add(Color::rgb(0.1, 0.1, 0.1).into());

        ambient_light.color = Color::WHITE * 30.0;

        let button_off = materials.add(Material::from(assets.load("images/button.png")));
        let button_on = materials.add(Material::from(assets.load("images/button-on.png")));

        SceneBuilder {
            builder,
            meshes,
            materials,
            wall_material,
            boundaries,
            assets,
            background_music,
            ambient_light,
            button_on,
            button_off,
        }
    }

    pub fn set_ambient_light(&mut self, ambient_light: Color) {
        self.ambient_light.color = ambient_light;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.builder.spawn_bundle(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(self.meshes.add(Quad::new(Vec2::ONE * 10000.0).into())),
            material: self.materials.add(color.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, Self::BACKGROUND_DEPTH)),
            ..default()
        });
    }

    pub fn set_audio(&mut self, name: &str) {
        self.background_music.0 = Some(name.to_string());
    }
}
