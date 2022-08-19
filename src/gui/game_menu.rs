use bevy::prelude::*;

use crate::game::GameState;
use crate::gui::buttons::spawn_basic_button;
use crate::gui::TRANSPARENT_COLOR;

#[derive(Debug, Clone)]
pub struct GameMenuPlugin;

#[derive(Debug, Clone, Component)]
pub enum GameMenuButton {
    Restart,
}

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup));
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(interaction));
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
            },
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct GameMenuNode;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/roboto.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Px(400.0), Val::Px(100.0)),
                position_type: PositionType::Absolute,
                ..default()
            },
            color: TRANSPARENT_COLOR,
            ..default()
        })
        .with_children(|parent| {
            spawn_basic_button(parent, font.clone(), "Restart", GameMenuButton::Restart);
        })
        .insert(GameMenuNode);
}

fn cleanup(mut commands: Commands, nodes: Query<Entity, With<GameMenuNode>>) {
    for entity in nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
