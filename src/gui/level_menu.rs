use bevy::prelude::*;

use crate::game::GameState;
use crate::gui::buttons::spawn_basic_button;
use crate::gui::TRANSPARENT_COLOR;
use crate::levels::Levels;

#[derive(Debug, Clone)]
pub struct LevelMenuPlugin;

#[derive(Debug, Clone, Component)]
pub enum LevelMenuButton {
    Level0,
    Level1,
    Level2,
    Level3,
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
    mut level: ResMut<Levels>,
) {
    for (interaction, button) in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match *button {
                LevelMenuButton::Back => {
                    game_state.set(GameState::MainMenu).unwrap();
                }
                LevelMenuButton::Level0 => {
                    *level = Levels::Level0;
                    game_state.set(GameState::Game).unwrap()
                }
                LevelMenuButton::Level1 => {
                    *level = Levels::Level1;
                    game_state.set(GameState::Game).unwrap()
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct LevelMenuNode;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/roboto.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Percent(40.0), Val::Auto),
                ..default()
            },
            color: TRANSPARENT_COLOR,
            ..default()
        })
        .with_children(|parent| {
            spawn_basic_button(parent, font.clone(), "Back", LevelMenuButton::Back);
            spawn_basic_button(parent, font.clone(), "Level 3", LevelMenuButton::Level3);
            spawn_basic_button(parent, font.clone(), "Level 2", LevelMenuButton::Level2);
            spawn_basic_button(parent, font.clone(), "Level 1", LevelMenuButton::Level1);
            spawn_basic_button(parent, font.clone(), "Level 0", LevelMenuButton::Level0);
        })
        .insert(LevelMenuNode);
}

fn cleanup(mut commands: Commands, nodes: Query<Entity, With<LevelMenuNode>>) {
    for entity in nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
