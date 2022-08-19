use bevy::prelude::*;

use crate::core::MapBuilder;
use crate::game::{GameState, Material};
use crate::levels::level0::setup_level0;

mod level0;

pub struct LevelsPlugin;

pub enum Levels {
    NoLevel,
    Level0,
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Levels::Level0); // todo: should be no level by default
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup));
        app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(cleanup));
    }
}

#[derive(Component)]
pub struct LevelRoot;

fn setup(
    level: Res<Levels>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
) {
    commands
        .spawn()
        .insert(LevelRoot)
        .insert_bundle(VisibilityBundle::default())
        .insert_bundle(TransformBundle::default())
        .with_children(|parent| {
            let mut builder = MapBuilder::new(&mut *meshes, &mut *materials);
            match *level {
                Levels::NoLevel => {}
                Levels::Level0 => {
                    setup_level0(&mut builder, parent);
                }
            }
        });
}

fn cleanup(mut commands: Commands, roots: Query<Entity, With<LevelRoot>>) {
    for root in roots.iter() {
        commands.entity(root).despawn_recursive();
    }
}
