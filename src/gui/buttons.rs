use bevy::prelude::*;

pub const BASE_BUTTON_COLOR: UiColor =
    UiColor(Color::rgb(27.0 / 255.0, 135.0 / 255.0, 62.0 / 255.0));
pub const HOVER_BUTTON_COLOR: UiColor =
    UiColor(Color::rgb(12.0 / 255.0, 112.0 / 255.0, 59.0 / 255.0));
pub const CLICKED_BUTTON_COLOR: UiColor =
    UiColor(Color::rgb(9.0 / 255.0, 186.0 / 255.0, 92.0 / 255.0));

#[derive(Debug, Clone)]
pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_interaction);
    }
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = CLICKED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                *color = HOVER_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = BASE_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn spawn_basic_button<T: Component>(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    button_type: T,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(65.0)),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: BASE_BUTTON_COLOR,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(button_type);
}
