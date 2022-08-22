use bevy::prelude::*;
use bevy::sprite::ColorMaterial;

#[cfg(debug_assertions)]
pub type Material = ColorMaterial;

#[cfg(not(debug_assertions))]
pub type Material = post_processing::ColorMaterialCustom;

pub fn material_from_texture_and_emissive(
    texture: Handle<Image>,
    _emissive: Handle<Image>,
) -> Material {
    #[cfg(not(debug_assertions))]
    return (texture, emissive).into();

    #[cfg(debug_assertions)]
    return texture.into();
}
