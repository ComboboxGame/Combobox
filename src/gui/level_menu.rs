use bevy::prelude::*;

use crate::game::GameState;
use crate::gui::buttons::spawn_level_button;
use crate::gui::TRANSPARENT_COLOR;
use crate::levels::CurrentLevel;

#[derive(Debug, Clone)]
pub struct LevelMenuPlugin;

#[derive(Debug, Clone, Component)]
pub enum LevelMenuButton {
    Level(usize),
    Back,
}

impl Plugin for LevelMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::LevelMenu).with_system(setup));
        app.add_system_set(SystemSet::on_update(GameState::LevelMenu).with_system(interaction));
        app.add_system_set(SystemSet::on_exit(GameState::LevelMenu).with_system(cleanup));
    }
}

fn interaction(
    interaction_query: Query<
        (&Interaction, &LevelMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<GameState>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    for (interaction, button) in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match button {
                LevelMenuButton::Level(level) => {
                    current_level.level = *level;
                    game_state.set(GameState::Game).unwrap();
                }
                LevelMenuButton::Back => {
                    game_state.set(GameState::MainMenu).unwrap();
                }
            },
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct LevelMenuNode;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::WHITE * 0.05;

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Percent(100.00), Val::Percent(16.66)),
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
                    image: asset_server.load("images/buttons/levels/back.png").into(),
                    ..default()
                })
                .insert(LevelMenuButton::Back);
        })
        .insert(LevelMenuNode);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Auto, Val::Percent(100.0)),
                min_size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                aspect_ratio: Some(4.0 / 6.0),
                ..default()
            },
            color: TRANSPARENT_COLOR,
            ..default()
        })
        .with_children(|parent| {
            let mut level = 0;

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    size: Size::new(Val::Percent(100.0), Val::Percent(16.66)),
                    ..default()
                },
                color: TRANSPARENT_COLOR,
                ..default()
            });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Percent(100.0), Val::Percent(16.66)),
                        ..default()
                    },
                    color: TRANSPARENT_COLOR,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            ..default()
                        },
                        image: asset_server.load("images/buttons/levels/levels.png").into(),
                        ..default()
                    });
                });

            for _ in 0..3 {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            margin: UiRect::all(Val::Auto),
                            size: Size::new(Val::Percent(100.0), Val::Percent(16.66)),
                            ..default()
                        },
                        color: TRANSPARENT_COLOR,
                        ..default()
                    })
                    .with_children(|parent| {
                        for _ in 0..4 {
                            level += 1;
                            let image = asset_server.load(
                                format!("images/buttons/levels/level-{}.png", level).as_str(),
                            );
                            spawn_level_button(parent, image.into(), LevelMenuButton::Level(level));
                        }
                    });
            }

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    size: Size::new(Val::Percent(100.0), Val::Percent(16.66)),
                    ..default()
                },
                color: TRANSPARENT_COLOR,
                ..default()
            });
        })
        .insert(LevelMenuNode);
}

fn cleanup(mut commands: Commands, nodes: Query<Entity, With<LevelMenuNode>>) {
    for entity in nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
