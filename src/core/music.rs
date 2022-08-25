use std::{path::PathBuf, time::Duration};

use bevy::{
    prelude::{
        App, AssetServer, Assets, Commands, Entity, Handle, Local, Plugin, Query, Res, ResMut,
    },
    utils::{HashMap, Instant},
};
use bevy_kira_audio::{Audio, AudioControl, AudioEasing, AudioInstance, AudioTween};

use super::Player;

pub struct BackgroundMusicHandle(Handle<AudioInstance>);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct BackgroundMusic(pub Option<String>);

#[derive(Debug)]
pub struct PlayerStatus {
    handle: Handle<AudioInstance>,
    start_time: Option<Instant>,
}

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_background_music);
        app.add_system(play_background_music);
        app.add_system(play_player_movement_sound);
    }
}

fn setup_background_music(mut commands: Commands) {
    commands.insert_resource(BackgroundMusic(Some("audio/main_menu_background.ogg".to_string())));
}

fn play_background_music(
    mut commands: Commands,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    background_music: Res<BackgroundMusic>,
    background_music_handle: Option<Res<BackgroundMusicHandle>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if background_music.is_changed() {
        if let Some(handle) = background_music_handle {
            if let Some(instance) = audio_instances.get_mut(&handle.0) {
                instance.stop(AudioTween::new(Duration::from_secs(1), AudioEasing::Linear));
            }
            commands.remove_resource::<BackgroundMusicHandle>();
        }
        if let Some(music) = &background_music.0 {
            let handle = audio
                .play(assets.load(PathBuf::from(music.clone())))
                .looped()
                .handle();

            commands.insert_resource(BackgroundMusicHandle(handle));
        }
    }
}

fn play_player_movement_sound(
    query: Query<(Entity, &Player)>,
    mut player_status: Local<HashMap<Entity, PlayerStatus>>,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    mut instances: ResMut<Assets<AudioInstance>>,
) {
    for (entity, player) in query.iter() {
        if let Some(status) = player_status.get_mut(&entity) {
            if let Some(instance) = instances.get_mut(&status.handle) {
                if player.is_moving && status.start_time.is_none() {
                    instance.resume(AudioTween::default());
                    status.start_time = Some(Instant::now());
                }

                if !player.is_moving && status.start_time.is_some()
                {
                    instance.pause(AudioTween::new(Duration::from_secs(1), AudioEasing::InPowf(2.)));
                    status.start_time = None;
                }
            }
        } else {
            if player.is_moving {
                let handle = audio
                    .play(assets.load("audio/movement.ogg"))
                    .looped()
                    .with_playback_rate(2.)
                    .handle();
                let status = PlayerStatus {
                    handle,
                    start_time: Some(Instant::now()),
                };
                player_status.insert(entity, status);
            }
        }
    }
}

fn play_box_join_sound() {
    
}
