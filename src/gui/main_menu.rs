use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::core::{PlayerType, PlayersSettings, MAX_PLAYERS_NUM};

use crate::gui::buttons::{spawn_basic_button, MenuArrow};
use crate::gui::TRANSPARENT_COLOR;
use crate::states::GuiState;
use std::ops::DerefMut;

#[derive(Debug, Clone)]
pub struct MainScreenGUIPlugin;

pub const PLAYER_COLORS_NUM: usize = 7;

#[derive(Debug, Clone, Component)]
pub enum MainMenuButton {
    Play,
    Play2,
    Settings,
    Credits,
}

#[derive(Component)]
pub struct MainMenuArrow {
    player_index: usize,
    button_index: usize,
}

#[derive(Component)]
pub struct ChoosePlayerImage {
    player_index: usize,
}

impl Plugin for MainScreenGUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GuiState::MainScreen).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(GuiState::MainScreen)
                .with_system(interaction)
                .with_system(update_player_preview),
        );
        app.add_system_set(SystemSet::on_exit(GuiState::MainScreen).with_system(cleanup));
    }
}

fn update_player_preview(
    mut images: Query<(&mut UiImage, &ChoosePlayerImage)>,
    player_settings: ResMut<PlayersSettings>,
    asset_server: Res<AssetServer>,
    mut preloaded_images: Local<HashSet<Handle<Image>>>,
) {
    if preloaded_images.is_empty() {
        for i in 0..7 {
            preloaded_images
                .insert(asset_server.load(format!("images/robot-preview-{}.png", i).as_str()));
        }
    }

    if !player_settings.is_changed() {
        return;
    }

    for (mut image, p_image) in images.iter_mut() {
        *image = UiImage(
            asset_server.load(
                player_settings.player_type[p_image.player_index]
                    .get_preview_image()
                    .as_str(),
            ),
        );
    }
}

fn interaction(
    buttons: Query<(&Interaction, &MainMenuButton), (Changed<Interaction>, Without<MainMenuArrow>)>,
    arrows: Query<(&Interaction, &MainMenuArrow), (Changed<Interaction>, Without<MainMenuButton>)>,
    mut gui_state: ResMut<State<GuiState>>,
    mut player_settings: ResMut<PlayersSettings>,
) {
    for (interaction, button) in buttons.iter() {
        match *interaction {
            Interaction::Clicked => match *button {
                MainMenuButton::Play => {
                    gui_state.set(GuiState::LevelSelection).unwrap();
                }
                _ => {}
            },
            _ => {}
        }
    }

    for (interaction, arrow) in arrows.iter() {
        match *interaction {
            Interaction::Clicked => {
                let mut banned = vec![];
                for i in 0..MAX_PLAYERS_NUM {
                    if i != arrow.player_index {
                        banned.push(player_settings.player_type[i])
                    }
                }

                if arrow.player_index == 0 {
                    banned.push(PlayerType::None);
                }

                player_settings.player_type[arrow.player_index] = if arrow.button_index == 0 {
                    player_settings.player_type[arrow.player_index].get_prev(&banned)
                } else {
                    player_settings.player_type[arrow.player_index].get_next(&banned)
                };
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct MainMenuNode;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
    mut player_settings: ResMut<PlayersSettings>,
) {
    clear_color.0 = Color::WHITE * 0.05;
    player_settings.deref_mut();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Auto, Val::Percent(100.0)),
                aspect_ratio: Some(1.0),
                ..default()
            },
            color: TRANSPARENT_COLOR,
            ..default()
        })
        .with_children(|parent| {
            // Choose color
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        margin: UiRect::all(Val::Auto),
                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                        ..default()
                    },
                    color: TRANSPARENT_COLOR,
                    ..default()
                })
                .with_children(|parent| {
                    for i in 0..2 {
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    margin: UiRect::all(Val::Auto),
                                    size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                    ..default()
                                },
                                color: TRANSPARENT_COLOR,
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(90.0), Val::Percent(90.0)),
                                            margin: UiRect::all(Val::Auto),
                                            ..default()
                                        },
                                        color: Color::rgba(0.2, 0.2, 0.2, 0.4).into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent
                                            .spawn_bundle(ImageBundle {
                                                image: UiImage(
                                                    asset_server.load("images/robot-preview-0.png"),
                                                ),
                                                style: Style {
                                                    position_type: PositionType::Absolute,
                                                    position: UiRect::new(
                                                        Val::Percent(70.0 * 0.2 + 15.0),
                                                        Val::Percent(70.0 * 0.2 + 15.0),
                                                        Val::Percent(15.0),
                                                        Val::Percent(15.0),
                                                    ),
                                                    size: Size::new(
                                                        Val::Percent(70.0 * 0.6),
                                                        Val::Percent(70.0),
                                                    ),
                                                    min_size: Size::new(
                                                        Val::Px(10.0),
                                                        Val::Px(10.0),
                                                    ),
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .insert(ChoosePlayerImage { player_index: i });

                                        parent
                                            .spawn_bundle(ButtonBundle {
                                                image: UiImage(
                                                    asset_server.load("images/buttons/prev.png"),
                                                ),
                                                style: Style {
                                                    margin: UiRect::all(Val::Auto),
                                                    size: Size::new(
                                                        Val::Percent(15.0),
                                                        Val::Percent(15.0),
                                                    ),
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .insert(MainMenuArrow {
                                                player_index: i,
                                                button_index: 0,
                                            })
                                            .insert(MenuArrow);
                                        parent.spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(40.0),
                                                    Val::Percent(15.0),
                                                ),
                                                ..default()
                                            },
                                            color: TRANSPARENT_COLOR,
                                            ..default()
                                        });
                                        parent
                                            .spawn_bundle(ButtonBundle {
                                                image: UiImage(
                                                    asset_server.load("images/buttons/next.png"),
                                                ),
                                                style: Style {
                                                    margin: UiRect::all(Val::Auto),
                                                    size: Size::new(
                                                        Val::Percent(15.0),
                                                        Val::Percent(15.0),
                                                    ),
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .insert(MainMenuArrow {
                                                player_index: i,
                                                button_index: 1,
                                            })
                                            .insert(MenuArrow);
                                    });
                            });
                    }
                });

            // Buttons
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        margin: UiRect::all(Val::Auto),
                        size: Size::new(Val::Percent(60.0), Val::Percent(50.0)),
                        ..default()
                    },
                    color: TRANSPARENT_COLOR,
                    ..default()
                })
                .with_children(|parent| {
                    spawn_basic_button(
                        parent,
                        asset_server.load("images/buttons/play-button.png").into(),
                        100.0,
                        MainMenuButton::Play,
                    );
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                margin: UiRect::all(Val::Auto),
                                size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                                ..default()
                            },
                            color: TRANSPARENT_COLOR,
                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_basic_button(
                                parent,
                                asset_server
                                    .load("images/buttons/credits-button.png")
                                    .into(),
                                100.0,
                                MainMenuButton::Settings,
                            );
                        });
                });
        })
        .insert(MainMenuNode);
}

fn cleanup(mut commands: Commands, nodes: Query<Entity, With<MainMenuNode>>) {
    for entity in nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
