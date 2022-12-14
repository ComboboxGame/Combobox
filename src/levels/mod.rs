use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::plugin::RapierConfiguration;
use post_processing::AmbientLight;

use crate::core::{
    BackgroundMusic, FinishPoint, Material, Player, PlayerBundle, PlayerType, PlayersSettings,
    SceneBoundaries, SceneBuilder, SpawnPoint, GRAVITY_FORCE,
};

use crate::states::{AudioState, CameraState, GuiState, LevelState};

mod level1;
mod level10;
mod level11;
mod level12;
mod level2;
mod level3;
mod level4;
mod level5;
mod level6;
mod level7;
mod level8;
mod level9;

pub struct LevelPlugin;

pub struct CurrentLevel {
    pub level: usize,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevel { level: 3 });
        app.add_system_set(SystemSet::on_enter(LevelState::Level).with_system(setup));
        app.add_system_set(SystemSet::on_exit(LevelState::Level).with_system(cleanup));
        app.add_system_set(
            SystemSet::on_update(LevelState::Level)
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
    boundaries: Res<SceneBoundaries>,
    mut level_state: ResMut<State<LevelState>>,
) {
    if !players.is_empty()
        && players.iter().all(|t| {
            let p = t.translation();

            if let Some(rect) = boundaries.rect {
                p.x < rect.min.x - 100.0
                    || p.x > rect.max.x + 100.0
                    || p.y < rect.min.y - 100.0
                    || p.y > rect.max.y + 100.0
            } else {
                p.length() > 10000.0
            }
        })
    {

        level_state.restart().unwrap();
    }
}

fn finish_level(
    finish_points: Query<&GlobalTransform, With<FinishPoint>>,
    players: Query<&GlobalTransform, With<Player>>,
    mut level_state: ResMut<State<LevelState>>,
    mut gui_state: ResMut<State<GuiState>>,
    mut camera_state: ResMut<State<CameraState>>,
    mut audio_state: ResMut<State<AudioState>>,
    mut timer: Local<f32>,
    time: Res<Time>,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    let mut any_player_unfinished = false;

    for player in players.iter() {
        let mut finished = false;
        for finish in finish_points.iter() {
            if (player.translation().xy() - finish.translation().xy()).length() < 120.0 {
                finished = true;
            }
        }
        if !finished {
            any_player_unfinished = true;
        }
    }

    if !players.is_empty()
        && !any_player_unfinished
        && *gui_state.current() != GuiState::LevelCompleted
    {
        *timer += time.delta_seconds();
        if *timer > 1.0 {
            *timer = 0.0;
            audio.play(assets.load("audio/finish.ogg")).with_volume(0.8);
            gui_state.set(GuiState::LevelCompleted).unwrap();
            ambient_light.color = Color::WHITE * 0.3;
            //camera_state.set(CameraState::None).unwrap();
            //audio_state.set(AudioState::None).unwrap();
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
    mut boundaries: ResMut<SceneBoundaries>,
    mut assets: ResMut<AssetServer>,
    mut config: ResMut<RapierConfiguration>,
    mut ambient_light: ResMut<AmbientLight>,
    background_music: ResMut<BackgroundMusic>,
) {
    // Set map defaults
    config.gravity = Vec2::NEG_Y * GRAVITY_FORCE;
    *boundaries = SceneBoundaries::default();

    commands
        .spawn()
        .insert(LevelRoot)
        .insert_bundle(VisibilityBundle::default())
        .insert_bundle(TransformBundle::default())
        .with_children(|parent| {
            let mut builder = SceneBuilder::new(
                parent,
                &mut *meshes,
                &mut *materials,
                &mut *boundaries,
                &mut *assets,
                background_music,
                ambient_light,
            );
            if current_level.level == 1 {
                level1::setup(&mut builder);
            }
            if current_level.level == 2 {
                level2::setup(&mut builder);
            }
            if current_level.level == 3 {
                level3::setup(&mut builder);
            }
            if current_level.level == 4 {
                level4::setup(&mut builder);
            }
            if current_level.level == 5 {
                level5::setup(&mut builder);
            }
            if current_level.level == 6 {
                level6::setup(&mut builder);
            }
            if current_level.level == 7 {
                level7::setup(&mut builder);
            }
            if current_level.level == 8 {
                level8::setup(&mut builder);
            }
            if current_level.level == 9 {
                level9::setup(&mut builder);
            }
            if current_level.level == 10 {
                level10::setup(&mut builder);
            }
            if current_level.level == 11 {
                level11::setup(&mut builder);
            }
            if current_level.level == 12 {
                level12::setup(&mut builder);
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
