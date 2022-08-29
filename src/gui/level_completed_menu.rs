use bevy::prelude::*;

use crate::gui::TRANSPARENT_COLOR;
use crate::levels::CurrentLevel;
use crate::states::{AudioState, CameraState, GuiState, LevelState};

#[derive(Debug, Clone)]
pub struct LevelCompleteGUIPlugin;

#[derive(Debug, Clone, Component)]
pub enum LevelCompleteButton {
    Restart,
    Back,
    NextLevel,
}

impl Plugin for LevelCompleteGUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GuiState::LevelCompleted).with_system(setup));
        app.add_system_to_stage(CoreStage::PreUpdate, interaction);
        app.add_system_set(SystemSet::on_exit(GuiState::LevelCompleted).with_system(cleanup));
    }
}

fn interaction(
    interaction_query: Query<
        (&Interaction, &LevelCompleteButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut level_state: ResMut<State<LevelState>>,
    mut audio_state: ResMut<State<AudioState>>,
    mut gui_state: ResMut<State<GuiState>>,
    mut camera_state: ResMut<State<CameraState>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    for (interaction, button) in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match *button {
                LevelCompleteButton::Restart => {
                    level_state.restart().unwrap();
                    gui_state.set(GuiState::Level).unwrap();
                }
                LevelCompleteButton::Back => {
                    // Go back to level selection
                    level_state.set(LevelState::None).unwrap();
                    audio_state.set(AudioState::Menu).unwrap();
                    gui_state.set(GuiState::LevelSelection).unwrap();
                    camera_state.set(CameraState::None).unwrap();
                }
                LevelCompleteButton::NextLevel => {
                    if current_level.level != 12 {
                        current_level.level = (current_level.level) % 12 + 1;
                        level_state.restart().unwrap();
                        gui_state.set(GuiState::Level).unwrap();
                    } else {
                        level_state.set(LevelState::None).unwrap();
                        audio_state.set(AudioState::Menu).unwrap();
                        camera_state.set(CameraState::None).unwrap();
                        gui_state.set(GuiState::LevelSelection).unwrap();
                    }
                }
            },
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct GameMenuNode;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Undefined, Val::Percent(25.0)),
                min_size: Size::new(Val::Px(10.0), Val::Px(10.0)),
                position: UiRect::new(
                    Val::Undefined,
                    Val::Percent(20.0),
                    Val::Undefined,
                    Val::Percent(20.0),
                ),
                aspect_ratio: Some(1.0),
                ..default()
            },
            image: asset_server.load("images/buttons/done-2.png").into(),
            ..default()
        })
        .insert(GameMenuNode);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Percent(100.00), Val::Percent(12.00)),
                position_type: PositionType::Absolute,
                ..default()
            },
            color: TRANSPARENT_COLOR,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Undefined, Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(10.0), Val::Px(10.0)),
                        aspect_ratio: Some(1.0),
                        ..default()
                    },
                    image: asset_server.load("images/buttons/back.png").into(),
                    ..default()
                })
                .insert(LevelCompleteButton::Back);
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Undefined, Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(10.0), Val::Px(10.0)),
                        aspect_ratio: Some(1.0),
                        ..default()
                    },
                    image: asset_server.load("images/buttons/restart.png").into(),
                    ..default()
                })
                .insert(LevelCompleteButton::Restart);
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Undefined, Val::Percent(100.0)),
                        min_size: Size::new(Val::Px(10.0), Val::Px(10.0)),
                        aspect_ratio: Some(800.0 / 260.0),
                        ..default()
                    },
                    image: asset_server.load("images/buttons/next-level.png").into(),
                    ..default()
                })
                .insert(LevelCompleteButton::NextLevel);
        })
        .insert(GameMenuNode);
}

fn cleanup(mut commands: Commands, mut nodes: Query<(Entity, &mut Transform), With<GameMenuNode>>) {
    for (entity, mut transform) in nodes.iter_mut() {
        // move it faaaar away
        transform.translation = Vec3::new(10000.0, 10000.0, 1.0);
        commands.entity(entity).despawn_recursive();
    }
}
