use bevy::prelude::*;

use crate::core::{MapBuilder, Player};
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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
    mut clear_color: ResMut<ClearColor>,
) {
    commands
        .spawn()
        .insert(LevelRoot)
        .insert_bundle(VisibilityBundle::default())
        .insert_bundle(TransformBundle::default())
        .with_children(|parent| {
            let mut builder =
                MapBuilder::new(parent, &mut *meshes, &mut *materials, &mut *clear_color);
            match *level {
                Levels::NoLevel => {}
                Levels::Level0 => {
                    setup_level0(&mut builder);
                }
            }
        });

    Player::spawn(commands, asset_server, Vec2::new(0., 0.));
}

fn cleanup(
    mut commands: Commands,
    roots: Query<Entity, With<LevelRoot>>,
    mut clear_color: ResMut<ClearColor>,
) {
    for root in roots.iter() {
        commands.entity(root).despawn_recursive();
    }

    clear_color.0 = Color::BLACK;
}
