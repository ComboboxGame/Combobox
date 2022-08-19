use post_processing::ColorMaterialCustom;
use bevy::asset::AssetServerSettings;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::sprite::Mesh2dHandle;
use bevy::window::PresentMode;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use post_processing::Core2dCustomPlugin;
use screen_diags_plugin::ScreenDiagsPlugin;

pub mod screen_diags_plugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        present_mode: PresentMode::Immediate,
        ..default()
    })
        .add_plugins(DefaultPlugins)
        .add_plugin(Core2dCustomPlugin)
        .add_plugin(ScreenDiagsPlugin)
        .add_startup_system(setup)
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        //.add_system(rotate)
        .add_system(change_brightness);

    app.run();
}

fn rotate(mut objects: Query<&mut Transform, With<Mesh2dHandle>>) {
    for mut transform in objects.iter_mut() {
        transform.rotate(Quat::from_axis_angle(Vec3::Z, 0.01));
    }
}

fn change_brightness(
    mut objects: Query<&Handle<ColorMaterialCustom>>,
    mut materials: ResMut<Assets<ColorMaterialCustom>>,
    time: Res<Time>,
) {
    for handle in objects.iter_mut() {
        let material = materials.get_mut(handle).unwrap();
        let v = (time.seconds_since_startup() as f32 * 0.28) % 2.0;
        let u = (time.seconds_since_startup() as f32 * 0.15) % 1.0;
        let z = ((time.seconds_since_startup() + 1.0) as f32 * 0.3) % 2.0;
        material.color = Color::rgb_linear(
            if v > 1.0 { 2.0 - v } else { v },
            u,
            if z > 1.0 { 2.0 - z } else { z },
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterialCustom>>,
    mut clear_color: ResMut<ClearColor>,
    mut asset_server: ResMut<AssetServer>,
) {
    //let texture_handle = asset_server.load("textures/stone_color.png");
    //let normal_handle = asset_server.load("textures/stone_normal.png");

    /*commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default()
            .with_scale(Vec3::splat(1500.))
            .with_rotation(Quat::from_axis_angle(Vec3::Z, 0.2)),
        material: materials.add((texture_handle.clone(), normal_handle.clone()).into()),
        ..default()
    });*/

    clear_color.0 = Color::rgb(0.02, 0.02, 0.02);

    /*
    let colors = [
        Color::rgb(0.0, 1.0, 0.0),
        Color::rgb(0.0, 0.8, 0.8),
        Color::rgb(1.0, 1.0, 1.0),
        Color::rgb(0.0, 0.0, 1.0),
        Color::rgb(1.0, 0.0, 0.0),
        Color::rgb(1.0, 0.5, 0.0),
        Color::rgb(0.8, 0.8, 0.0),
    ];

    let num = 15;
    for (j, color) in colors.iter().enumerate() {
        for i in 0..num {
            let x = (i as f32 - (num - 1) as f32 / 2.0) * 80.0;
            let y = (j as f32 - (colors.len() - 1) as f32 / 2.0) * 80.0;
            let v = (i + 1) as f32 * 0.3;
            commands.spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform::from_xyz(x, y, 0.0)
                    .with_scale(Vec3::splat(40.))
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, 0.2)),
                material: materials.add((*color * v).into()),
                ..default()
            });
        }
    }*/
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(600.)),
        material: materials.add((Color::WHITE * 2.0).into()),
        ..default()
    });

    asset_server.watch_for_changes().unwrap();

    commands.spawn_bundle(Camera2dBundle::default());
}
