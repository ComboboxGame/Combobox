use bevy::prelude::*;

use crate::core::{MapBoundaries, MapBuilder, Player, SpawnPoint};
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
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(spawn_players)
                .with_system(restart_on_out_of_boundaries),
        );
    }
}

#[derive(Component)]
pub struct LevelRoot;

fn restart_on_out_of_boundaries(
    players: Query<&GlobalTransform, With<Player>>,
    boundaries: Res<MapBoundaries>,
    mut game_state: ResMut<State<GameState>>,
) {
    if !players.is_empty()
        && players.iter().all(|t| {
            let p = t.translation();

            if let Some(rect) = boundaries.rect {
                p.x < rect.min.x - 100.0
                    || p.x > rect.max.x + 100.0
                    || p.y < rect.min.x - 100.0
                    || p.y > rect.max.x + 100.0
            } else {
                p.length() > 10000.0
            }
        })
    {
        game_state.restart().unwrap();
    }
}

fn spawn_players(
    mut commands: Commands,
    spawn_points: Query<(Entity, &SpawnPoint)>,
    players: Query<&Player>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
) {
    for (entity, spawn_point) in spawn_points.iter() {
        let mut player_exists = false;
        for player in players.iter() {
            if player.id == spawn_point.id {
                player_exists = true;
            }
        }

        if !player_exists {
            let player = Player::spawn(
                &mut commands,
                &asset_server,
                &mut meshes,
                &mut materials,
                spawn_point.id,
            );
            commands.entity(entity).add_child(player);
        }
    }
}

fn setup(
    level: Res<Levels>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
    mut clear_color: ResMut<ClearColor>,
    mut boundaries: ResMut<MapBoundaries>,
    mut assets: ResMut<AssetServer>,
) {
    commands
        .spawn()
        .insert(LevelRoot)
        .insert_bundle(VisibilityBundle::default())
        .insert_bundle(TransformBundle::default())
        .with_children(|parent| {
            let mut builder = MapBuilder::new(
                parent,
                &mut *meshes,
                &mut *materials,
                &mut *clear_color,
                &mut *boundaries,
                &mut *assets,
            );
            match *level {
                Levels::NoLevel => {}
                Levels::Level0 => {
                    setup_level0(&mut builder);
                }
            }
        });
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
