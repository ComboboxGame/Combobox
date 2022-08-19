use bevy::prelude::*;

use crate::game::GameState;
use crate::gui::buttons::spawn_basic_button;
use crate::gui::TRANSPARENT_COLOR;

#[derive(Debug, Clone)]
pub struct MainMenuPlugin;

#[derive(Debug, Clone, Component)]
pub enum MainMenuButton {
    Play1,
    Play2,
    Settings,
    Credits,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup));
        app.add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(interaction));
        app.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(cleanup));
    }
}

fn interaction(
    interaction_query: Query<(&Interaction, &MainMenuButton), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, button) in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match *button {
                MainMenuButton::Play1 => {
                    game_state.set(GameState::LevelMenu).unwrap();
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct MainMenuNode;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/roboto.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Percent(30.0), Val::Auto),
                ..default()
            },
            color: TRANSPARENT_COLOR,
            ..default()
        })
        .with_children(|parent| {
            spawn_basic_button(parent, font.clone(), "Credits", MainMenuButton::Credits);
            spawn_basic_button(parent, font.clone(), "Settings", MainMenuButton::Settings);
            spawn_basic_button(
                parent,
                font.clone(),
                "Play (2 players)",
                MainMenuButton::Play2,
            );
            spawn_basic_button(
                parent,
                font.clone(),
                "Play (1 player)",
                MainMenuButton::Play1,
            );
        })
        .insert(MainMenuNode);
}

fn cleanup(mut commands: Commands, nodes: Query<Entity, With<MainMenuNode>>) {
    for entity in nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
