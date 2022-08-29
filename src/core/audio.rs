use std::{path::PathBuf, time::Duration};

use bevy::{
    prelude::{
        App, AssetServer, Assets, Commands, Entity, Handle, Local, Plugin, Query, Res, ResMut, SystemSet,
    },
    utils::{HashMap, Instant},
};
use bevy_kira_audio::{Audio, AudioControl, AudioEasing, AudioInstance, AudioTween};

use crate::states::AudioState;

use super::Player;

pub struct BackgroundMusicHandle(Handle<AudioInstance>);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct BackgroundMusic(pub Option<String>);

#[derive(Debug)]
pub struct PlayerStatus {
    handle: Handle<AudioInstance>,
    start_time: Option<Instant>,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AudioState::Menu).with_system(setup_menu_music));
        app.add_system(play_background_music);
        app.add_system(play_player_movement_sound);

        app.insert_resource(BackgroundMusic(Some(
            "audio/main_menu_background.ogg".to_string(),
        )));
    }
}

fn setup_menu_music(mut background_music: ResMut<BackgroundMusic>) {
    background_music.0 = Some("audio/main_menu_background.ogg".to_string());
}

fn play_background_music(
    mut commands: Commands,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    background_music: Res<BackgroundMusic>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if background_music.is_changed() {
        for (_, instance) in audio_instances.iter_mut() {
            instance.stop(AudioTween::new(Duration::from_secs(1), AudioEasing::Linear));
        }
        if let Some(music) = &background_music.0 {
            audio
                .play(assets.load(PathBuf::from(music.clone())))
                .with_volume(0.1)
                .looped()
                .handle();
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

                if !player.is_moving && status.start_time.is_some() {
                    instance.pause(AudioTween::new(
                        Duration::from_secs(1),
                        AudioEasing::InPowf(2.),
                    ));
                    status.start_time = None;
                }
            }
        } else {
            if player.is_moving {
                let handle = audio
                    .play(assets.load("audio/movement.ogg"))
                    .looped()
                    .with_playback_rate(1.8)
                    .with_volume(2.)
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

fn play_box_join_sound() {}
