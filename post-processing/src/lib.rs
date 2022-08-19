use std::ops::Range;

use bevy::app::{App, Plugin};
use bevy::asset::load_internal_asset;
use bevy::core_pipeline::core_2d::{extract_core_2d_camera_phases, MainPass2dNode, Transparent2d};
use bevy::core_pipeline::core_2d::graph;
use bevy::ecs::prelude::*;
use bevy::prelude::{Assets, Camera2d, Handle, HandleUntyped, Image, Msaa, Shader};
use bevy::reflect::TypeUuid;
use bevy::render::{
    camera::Camera,
    Extract,
    extract_component::ExtractComponentPlugin,
    render_graph::{RenderGraph, SlotInfo, SlotType},
    render_phase::{
        batch_phase_system, BatchedPhaseItem, CachedRenderPipelinePhaseItem, DrawFunctionId,
        DrawFunctions, EntityPhaseItem, PhaseItem, RenderPhase, sort_phase_system,
    },
    render_resource::CachedRenderPipelineId, RenderApp, RenderStage,
};
use bevy::render::camera::ExtractedCamera;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::RunGraphOnViewNode;
use bevy::render::render_resource::{
    Extent3d, Operations, ShaderStage, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureView,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::{BevyDefault, TextureCache};
use bevy::render::view::{ExtractedWindows, WindowSystem};
use bevy::sprite::Material2dPlugin;
use bevy::ui::{draw_ui_graph, UiPassNode};
use bevy::utils::{FloatOrd, HashMap};
pub use color_material_custom::ColorMaterialCustom;
use wgpu::{Color, RenderPassColorAttachment};

use crate::bloom_node::*;
use crate::color_material_custom::*;
use crate::main_pass_2d_node::*;
use crate::main_pass_2d_node::MainPass2dNodeCustom;
use crate::tone_mapping_node::*;

mod bloom_node;
mod color_material_custom;
mod main_pass_2d_node;
mod tone_mapping_node;
mod utils;

pub struct Core2dCustomPlugin;

pub const CUSTOM_DEFAULT_VERT: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8454671400261990324);
pub const CUSTOM_UPSCALING: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8678671400261345394);
pub const CUSTOM_DOWNSCALING: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8214674567261990328);
pub const CUSTOM_TONEMAPPING: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8994546787261990326);
pub const CUSTOM_MATERIAL: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8213474567257890329);

impl Plugin for Core2dCustomPlugin {
    fn build(&self, app: &mut App) {
        let mut assets = app.world.resource_mut::<Assets<Shader>>();
        assets.set_untracked(
            CUSTOM_DEFAULT_VERT,
            Shader::from_glsl(include_str!("shaders/default.vert"), ShaderStage::Vertex),
        );
        assets.set_untracked(
            CUSTOM_UPSCALING,
            Shader::from_glsl(
                include_str!("shaders/upscaling.frag"),
                ShaderStage::Fragment,
            ),
        );
        assets.set_untracked(
            CUSTOM_DOWNSCALING,
            Shader::from_glsl(
                include_str!("shaders/downscaling.frag"),
                ShaderStage::Fragment,
            ),
        );
        assets.set_untracked(
            CUSTOM_TONEMAPPING,
            Shader::from_glsl(
                include_str!("shaders/tonemapping.frag"),
                ShaderStage::Fragment,
            ),
        );
        assets.set_untracked(
            CUSTOM_MATERIAL,
            Shader::from_wgsl(include_str!("shaders/color_material_custom.wgsl")),
        );

        app.add_plugin(ColorMaterialCustomPlugin);

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app.init_resource::<ToneMappingPipeline>();
        render_app.init_resource::<BloomPipeline>();

        render_app.add_system_to_stage(
            RenderStage::Prepare,
            prepare_view_targets_custom.after(WindowSystem::Prepare),
        );

        render_app.add_system_to_stage(
            RenderStage::Prepare,
            prepare_bloom_targets.after(WindowSystem::Prepare),
        );

        let pass_node_2d = MainPass2dNodeCustom::new(&mut render_app.world);
        let bloom_node = BloomNode::new(&mut render_app.world);
        let tone_mapping_node = ToneMappingNode::new(&mut render_app.world);

        let mut graph = render_app.world.resource_mut::<RenderGraph>();
        let mut draw_2d_graph = graph
            .get_sub_graph_mut(bevy::core_pipeline::core_2d::graph::NAME)
            .unwrap();

        draw_2d_graph.remove_node(graph::node::MAIN_PASS);

        draw_2d_graph.add_node(graph::node::MAIN_PASS, pass_node_2d);
        draw_2d_graph.add_node(BloomNode::NAME, bloom_node);
        draw_2d_graph.add_node(ToneMappingNode::NAME, tone_mapping_node);

        draw_2d_graph
            .add_slot_edge(
                draw_2d_graph.input_node().unwrap().id,
                graph::input::VIEW_ENTITY,
                graph::node::MAIN_PASS,
                MainPass2dNode::IN_VIEW,
            )
            .unwrap();

        draw_2d_graph
            .add_slot_edge(
                draw_2d_graph.input_node().unwrap().id,
                graph::input::VIEW_ENTITY,
                ToneMappingNode::NAME,
                ToneMappingNode::IN_VIEW,
            )
            .unwrap();

        draw_2d_graph
            .add_slot_edge(
                draw_2d_graph.input_node().unwrap().id,
                graph::input::VIEW_ENTITY,
                BloomNode::NAME,
                BloomNode::IN_VIEW,
            )
            .unwrap();

        draw_2d_graph
            .add_slot_edge(
                graph::node::MAIN_PASS,
                MainPass2dNodeCustom::OUT_TEXTURE,
                BloomNode::NAME,
                BloomNode::IN_TEXTURE,
            )
            .unwrap();

        draw_2d_graph
            .add_slot_edge(
                BloomNode::NAME,
                BloomNode::OUT_TEXTURE,
                ToneMappingNode::NAME,
                ToneMappingNode::IN_TEXTURE,
            )
            .unwrap();

        draw_2d_graph
            .add_node_edge(ToneMappingNode::NAME, draw_ui_graph::node::UI_PASS)
            .unwrap();
    }
}

fn get_ui_graph(render_app: &mut App) -> RenderGraph {
    let ui_pass_node = UiPassNode::new(&mut render_app.world);
    let mut ui_graph = RenderGraph::default();
    ui_graph.add_node(draw_ui_graph::node::UI_PASS, ui_pass_node);
    let input_node_id = ui_graph.set_input(vec![SlotInfo::new(
        draw_ui_graph::input::VIEW_ENTITY,
        SlotType::Entity,
    )]);
    ui_graph
        .add_slot_edge(
            input_node_id,
            draw_ui_graph::input::VIEW_ENTITY,
            draw_ui_graph::node::UI_PASS,
            UiPassNode::IN_VIEW,
        )
        .unwrap();
    ui_graph
}

#[derive(Component)]
pub struct ViewTargetCustom {
    pub view: TextureView,
    pub sampled_target: Option<TextureView>,
}

impl ViewTargetCustom {
    pub fn get_color_attachment(&self, ops: Operations<Color>) -> RenderPassColorAttachment {
        RenderPassColorAttachment {
            view: self.sampled_target.as_ref().unwrap_or(&self.view),
            resolve_target: if self.sampled_target.is_some() {
                Some(&self.view)
            } else {
                None
            },
            ops,
        }
    }

    pub fn get_texture_view(&self) -> TextureView {
        self.view.clone()
    }
}

fn prepare_view_targets_custom(
    mut commands: Commands,
    msaa: Res<Msaa>,
    render_device: Res<RenderDevice>,
    mut texture_cache: ResMut<TextureCache>,
    cameras: Query<(Entity, &ExtractedCamera)>,
) {
    let mut sampled_textures = HashMap::default();
    let mut textures = HashMap::default();

    for (entity, camera) in &cameras {
        if let Some(target_size) = camera.physical_target_size {
            let texture_view = textures
                .entry(camera.target.clone())
                .or_insert_with(|| {
                    texture_cache.get(
                        &render_device,
                        TextureDescriptor {
                            label: Some("color_attachment_texture_custom"),
                            size: Extent3d {
                                width: target_size.x,
                                height: target_size.y,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: TextureDimension::D2,
                            format: TextureFormat::Rgba32Float,
                            usage: TextureUsages::RENDER_ATTACHMENT
                                | TextureUsages::TEXTURE_BINDING,
                        },
                    )
                })
                .default_view
                .clone();

            let sampled_target = if msaa.samples > 1 {
                let sampled_texture = sampled_textures
                    .entry(camera.target.clone())
                    .or_insert_with(|| {
                        texture_cache.get(
                            &render_device,
                            TextureDescriptor {
                                label: Some("sampled_color_attachment_texture_custom"),
                                size: Extent3d {
                                    width: target_size.x,
                                    height: target_size.y,
                                    depth_or_array_layers: 1,
                                },
                                mip_level_count: 1,
                                sample_count: msaa.samples,
                                dimension: TextureDimension::D2,
                                format: TextureFormat::Rgba32Float,
                                usage: TextureUsages::RENDER_ATTACHMENT
                                    | TextureUsages::TEXTURE_BINDING,
                            },
                        )
                    });
                Some(sampled_texture.default_view.clone())
            } else {
                None
            };
            commands.entity(entity).insert(ViewTargetCustom {
                view: texture_view.clone(),
                sampled_target,
            });
        }
    }
}
