use bevy::app::{App, Plugin};
use bevy::asset::{Assets, Handle};
use bevy::math::Vec4;

use bevy::reflect::TypeUuid;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::{
    color::Color, render_asset::RenderAssets, render_resource::*, texture::Image,
};

use bevy::sprite::{
    Material2d, Material2dKey, Material2dPlugin,
};
use crate::CUSTOM_MATERIAL;

#[derive(Default)]
pub struct ColorMaterialCustomPlugin;

impl Plugin for ColorMaterialCustomPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<ColorMaterialCustom>::default());

        app.world
            .resource_mut::<Assets<ColorMaterialCustom>>()
            .set_untracked(
                Handle::<ColorMaterialCustom>::default(),
                ColorMaterialCustom {
                    color: Color::rgb(1.0, 0.0, 1.0),
                    ..Default::default()
                },
            );
    }
}

/// A [2d material](Material2d) that renders [2d meshes](crate::Mesh2dHandle) with a texture tinted by a uniform color
#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "e228a544-e3ca-4e1e-ba9d-4d8bc1ad8c19"]
#[uniform(0, ColorMaterialCustomUniform)]
pub struct ColorMaterialCustom {
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,

    #[texture(3)]
    #[sampler(4)]
    pub emissive: Option<Handle<Image>>,
}

impl Default for ColorMaterialCustom {
    fn default() -> Self {
        ColorMaterialCustom {
            color: Color::WHITE,
            texture: None,
            emissive: None,
        }
    }
}

impl From<Color> for ColorMaterialCustom {
    fn from(color: Color) -> Self {
        ColorMaterialCustom {
            color,
            ..Default::default()
        }
    }
}

impl From<Handle<Image>> for ColorMaterialCustom {
    fn from(texture: Handle<Image>) -> Self {
        ColorMaterialCustom {
            texture: Some(texture),
            ..Default::default()
        }
    }
}

impl From<(Handle<Image>, Handle<Image>)> for ColorMaterialCustom {
    fn from(texture: (Handle<Image>, Handle<Image>)) -> Self {
        ColorMaterialCustom {
            texture: Some(texture.0),
            emissive: Some(texture.1),
            ..Default::default()
        }
    }
}

/// The GPU representation of the uniform data of a [`ColorMaterialCustom`].
#[derive(Clone, Default, ShaderType)]
pub struct ColorMaterialCustomUniform {
    pub color: Vec4,
    pub flags: u32,
}

// NOTE: These must match the bit flags in bevy_sprite/src/mesh2d/color_material.wgsl!
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ColorMaterialFlagsCustom: u32 {
        const TEXTURE           = (1 << 0);
        const EMISSIVE           = (1 << 1);
        const NONE              = 0;
        const UNINITIALIZED     = 0xFFFF;
    }
}

impl AsBindGroupShaderType<ColorMaterialCustomUniform> for ColorMaterialCustom {
    fn as_bind_group_shader_type(
        &self,
        _images: &RenderAssets<Image>,
    ) -> ColorMaterialCustomUniform {
        let mut flags = ColorMaterialFlagsCustom::NONE;
        if self.texture.is_some() {
            flags |= ColorMaterialFlagsCustom::TEXTURE;
        }
        if self.emissive.is_some() {
            flags |= ColorMaterialFlagsCustom::EMISSIVE;
        }

        ColorMaterialCustomUniform {
            color: self.color.as_linear_rgba_f32().into(),
            flags: flags.bits(),
        }
    }
}

impl Material2d for ColorMaterialCustom {
    fn fragment_shader() -> ShaderRef {
        CUSTOM_MATERIAL.typed().into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // HDR texture
        (*descriptor.fragment.as_mut().unwrap()).targets.clear();
        (*descriptor.fragment.as_mut().unwrap())
            .targets
            .push(Some(ColorTargetState {
                format: TextureFormat::Rg11b10Float,
                blend: Some(BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            }));
        descriptor.primitive.cull_mode = None;

        Ok(())
    }
}
