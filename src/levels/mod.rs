use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::core::{
    FinishPoint, MapBoundaries, MapBuilder, Material, Player, PlayerBundle, PlayerType,
    PlayersSettings, SpawnPoint, GRAVITY,
};

use crate::game::GameState;
use crate::levels::level1::setup_level1;
use crate::levels::level2::setup_level2;

mod level1;
mod level2;

pub struct LevelsPlugin;

pub struct CurrentLevel {
    pub level: usize,
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevel { level: 0 });
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup));
        app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(cleanup));
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(spawn_players)
                .with_system(restart_on_out_of_boundaries)
                .with_system(finish_level),
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

fn finish_level(
    finish_points: Query<&GlobalTransform, With<FinishPoint>>,
    players: Query<&GlobalTransform, With<Player>>,
    mut game_state: ResMut<State<GameState>>,
    mut timer: Local<f32>,
    time: Res<Time>,
) {
    let mut any_player_unfinished = false;

    for player in players.iter() {
        let mut finished = false;
        for finish in finish_points.iter() {
            if (player.translation().xy() - finish.translation().xy()).length() < 80.0 {
                finished = true;
            }
        }
        if !finished {
            any_player_unfinished = true;
        }
    }

    if !players.is_empty() && !any_player_unfinished {
        *timer += time.delta_seconds();
        if *timer > 1.0 {
            *timer = 0.0;
            game_state.set(GameState::LevelMenu).unwrap();
        }
    } else {
        *timer = 0.0;
    }
}

fn spawn_players(
    mut commands: Commands,
    spawn_points: Query<(Entity, &SpawnPoint)>,
    players: Query<&Player>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
    players_settings: Res<PlayersSettings>,
) {
    let num_of_players = players_settings
        .player_type
        .iter()
        .filter(|v| **v != PlayerType::None)
        .count();

    for (entity, spawn_point) in spawn_points.iter() {
        if spawn_point.index.get_number_of_players() != num_of_players {
            continue;
        }

        let mut player_exists = false;
        for player in players.iter() {
            if player.index == spawn_point.index {
                player_exists = true;
            }
        }

        if !player_exists {
            let player_bundle = PlayerBundle::new(
                spawn_point.index,
                players_settings.player_type[spawn_point.index.unwrap_index()],
                &mut *meshes,
                &mut *materials,
                &asset_server,
            );
            let player_id = commands.spawn_bundle(player_bundle).id();
            commands.entity(entity).add_child(player_id);
        }
    }
}

fn setup(
    current_level: Res<CurrentLevel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
    mut clear_color: ResMut<ClearColor>,
    mut boundaries: ResMut<MapBoundaries>,
    mut assets: ResMut<AssetServer>,
    mut config: ResMut<RapierConfiguration>,
) {
    // Set map defaults
    config.gravity = Vec2::NEG_Y * GRAVITY;
    *boundaries = MapBoundaries::default();
    *clear_color = ClearColor(Color::BLACK);

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
            if current_level.level == 1 {
                setup_level1(&mut builder);
            }
            if current_level.level == 2 {
                setup_level2(&mut builder);
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
