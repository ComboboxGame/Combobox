use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;

use crate::ColorMaterialCustom;

#[derive(Component, Debug, Default)]
pub struct PointLight2d {
    pub radius: f32,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct AmbientLight {
    pub color: Color,
}

#[derive(ShaderType, Clone, Debug)]
pub struct PointLightsUniform {
    pub lights_num: u32,
    pub positions: [Vec4; 16],
    pub colors: [Vec4; 16],
    pub ambient: Vec4,
}

impl Default for PointLightsUniform {
    fn default() -> Self {
        Self {
            lights_num: 0,
            positions: [Vec4::ZERO; 16],
            colors: [Vec4::ZERO; 16],
            ambient: Vec4::new(30.0, 30.0, 30.0, 1.0),
        }
    }
}

pub fn update_lights(
    lights_query: Query<(&PointLight2d, &GlobalTransform)>,
    handles: Query<&Handle<ColorMaterialCustom>>,
    mut materials: ResMut<Assets<ColorMaterialCustom>>,
    ambient: Res<AmbientLight>,
) {
    let v: Vec4 = ambient.color.into();
    if v.truncate().length() > 10.0 && !ambient.is_changed() {
        return;
    }


    let mut lights = PointLightsUniform::default();

    lights.ambient = ambient.color.as_linear_rgba_f32().into();

    for (point_light, transform) in lights_query.iter() {
        if point_light.radius < 0.01 {
            continue;
        }

        let i = lights.lights_num as usize;
        lights.positions[i] = Vec4::new(transform.translation().x, transform.translation().y, transform.translation().z, point_light.radius);
        lights.colors[i] = point_light.color.as_linear_rgba_f32().into();
        lights.lights_num += 1;
    }

    for handle in handles.iter() {
        if let Some(material) = materials.get_mut(handle) {
            material.lights = lights.clone();
        }
    }
}
