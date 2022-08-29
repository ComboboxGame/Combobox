use bevy::prelude::*;

use crate::gui::buttons::spawn_level_button;
use crate::gui::TRANSPARENT_COLOR;
use crate::levels::CurrentLevel;
use crate::states::{AudioState, CameraState, GuiState, LevelState};

#[derive(Debug, Clone)]
pub struct CreditsGUIPlugin;

#[derive(Debug, Clone, Component)]
pub enum CreditsButton {
    Back,
}

impl Plugin for CreditsGUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GuiState::Credits).with_system(setup));
        app.add_system_set(SystemSet::on_update(GuiState::Credits).with_system(interaction));
        app.add_system_set(SystemSet::on_exit(GuiState::Credits).with_system(cleanup));
    }
}

fn interaction(
    interaction_query: Query<
        (&Interaction, &CreditsButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut gui_state: ResMut<State<GuiState>>,
) {
    for (interaction, button) in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match button {
                CreditsButton::Back => {
                    gui_state.set(GuiState::MainScreen).unwrap();
                }
            },
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct CreditsMenuNode;

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
                    image: asset_server.load("images/buttons/levels/back.png").into(),
                    ..default()
                })
                .insert(CreditsButton::Back);
        })
        .insert(CreditsMenuNode);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Percent(100.00), Val::Percent(90.00)),
                position_type: PositionType::Absolute,
                ..default()
            },
            color: TRANSPARENT_COLOR,
            ..default()
        }).with_children(|builder| {
        builder.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Undefined, Val::Percent(100.0)),
                min_size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                aspect_ratio: Some(1.0),
                ..default()
            },
            image: UiImage(asset_server.load("images/credits.png").into()),
            ..default()
        });
    }).insert(CreditsMenuNode);

}

fn cleanup(mut commands: Commands, nodes: Query<Entity, With<CreditsMenuNode>>) {
    for entity in nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
