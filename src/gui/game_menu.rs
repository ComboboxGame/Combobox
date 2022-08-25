use bevy::prelude::*;

use crate::game::GameState;

use crate::gui::TRANSPARENT_COLOR;

#[derive(Debug, Clone)]
pub struct GameMenuPlugin;

#[derive(Debug, Clone, Component)]
pub enum GameMenuButton {
    Restart,
    Back,
}

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup));
        app.add_system_to_stage(CoreStage::PreUpdate, interaction);
        app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(cleanup));
    }
}

fn interaction(
    interaction_query: Query<(&Interaction, &GameMenuButton), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, button) in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match *button {
                GameMenuButton::Restart => {
                    game_state.restart().unwrap();
                }
                GameMenuButton::Back => {
                    game_state.set(GameState::LevelMenu).unwrap();
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
                    image: asset_server.load("images/buttons/back.png").into(),
                    ..default()
                })
                .insert(GameMenuButton::Back);
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
                .insert(GameMenuButton::Restart);
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
