use bevy::prelude::*;

pub const BASE_ARROW_COLOR: UiColor = UiColor(Color::WHITE);
pub const HOVER_ARROW_COLOR: UiColor = UiColor(Color::rgb(0.9, 0.9, 0.9));
pub const CLICKED_ARROW_COLOR: UiColor = UiColor(Color::rgb(0.75, 0.75, 0.75));

#[derive(Debug, Clone)]
pub struct ButtonsPlugin;

#[derive(Component)]
pub struct MenuButton;

#[derive(Component)]
pub struct MenuArrow;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_interaction);
    }
}

fn button_interaction(
    mut buttons: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();

    for (interaction, mut color) in &mut buttons {
        match *interaction {
            Interaction::Clicked => {
                *color = CLICKED_ARROW_COLOR.into();
                window.set_cursor_icon(CursorIcon::Hand);
            }
            Interaction::Hovered => {
                *color = HOVER_ARROW_COLOR.into();
                window.set_cursor_icon(CursorIcon::Hand);
            }
            Interaction::None => {
                *color = BASE_ARROW_COLOR.into();
                window.set_cursor_icon(CursorIcon::Default);
            }
        }
    }
}

pub fn spawn_basic_button<T: Component>(
    parent: &mut ChildBuilder,
    image: UiImage,
    size: f32,
    button_type: T,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(size), Val::Undefined),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Px(0.0), Val::Px(0.0)),
                aspect_ratio: Some(2.6 / 8.0),
                min_size: Size::new(Val::Px(10.0), Val::Px(10.0)),
                ..default()
            },
            image,
            color: BASE_ARROW_COLOR,
            ..default()
        })
        .insert(MenuButton)
        .insert(button_type);
}

pub fn spawn_level_button<T: Component>(parent: &mut ChildBuilder, image: UiImage, button_type: T) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Undefined, Val::Percent(100.0)),
                aspect_ratio: Some(1.0),
                min_size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                ..default()
            },
            image,
            color: BASE_ARROW_COLOR,
            ..default()
        })
        .insert(MenuButton)
        .insert(button_type);
}
