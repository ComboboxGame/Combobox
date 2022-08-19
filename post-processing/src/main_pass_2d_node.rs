use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::core_2d::{MainPass2dNode, Transparent2d};
use bevy::prelude::*;
use bevy::render::camera::ExtractedCamera;
use bevy::render::render_graph::{
    Node, NodeRunError, RenderGraphContext, RunSubGraphError, SlotInfo, SlotType,
};
use bevy::render::render_phase::{DrawFunctions, RenderPhase, TrackedRenderPass};
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ExtractedView, ViewTarget};
use std::borrow::Cow;
use wgpu::{Extent3d, ImageCopyTexture, LoadOp, Operations, RenderPassDescriptor};

use crate::ViewTargetCustom;

pub struct MainPass2dNodeCustom {
    query: QueryState<
        (
            &'static ExtractedCamera,
            &'static RenderPhase<Transparent2d>,
            &'static ViewTargetCustom,
            &'static Camera2d,
        ),
        With<ExtractedView>,
    >,
}

impl MainPass2dNodeCustom {
    pub const IN_VIEW: &'static str = "view";
    pub const OUT_TEXTURE: &'static str = "texture";

    pub fn new(world: &mut World) -> Self {
        Self {
            query: world.query_filtered(),
        }
    }
}

impl Node for MainPass2dNodeCustom {
    fn input(&self) -> Vec<SlotInfo> {
        vec![SlotInfo::new(
            MainPass2dNodeCustom::IN_VIEW,
            SlotType::Entity,
        )]
    }

    fn output(&self) -> Vec<SlotInfo> {
        vec![SlotInfo::new(
            MainPass2dNodeCustom::OUT_TEXTURE,
            SlotType::TextureView,
        )]
    }

    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.get_input_entity(Self::IN_VIEW)?;

        if let Ok((camera, transparent_phase, target_custom, camera_2d)) =
            self.query.get_manual(world, view_entity)
        {
            let pass_descriptor = RenderPassDescriptor {
                label: Some("main_pass_2d_custom"),
                color_attachments: &[Some(target_custom.get_color_attachment(Operations {
                    load: match camera_2d.clear_color {
                        ClearColorConfig::Default => {
                            LoadOp::Clear(world.resource::<ClearColor>().0.into())
                        }
                        ClearColorConfig::Custom(color) => LoadOp::Clear(color.into()),
                        ClearColorConfig::None => LoadOp::Load,
                    },
                    store: true,
                }))],
                depth_stencil_attachment: None,
            };

            let draw_functions = world.resource::<DrawFunctions<Transparent2d>>();

            let render_pass = render_context
                .command_encoder
                .begin_render_pass(&pass_descriptor);

            let mut draw_functions = draw_functions.write();
            let mut tracked_pass = TrackedRenderPass::new(render_pass);
            if let Some(viewport) = camera.viewport.as_ref() {
                tracked_pass.set_camera_viewport(viewport);
            }
            for item in &transparent_phase.items {
                let draw_function = draw_functions.get_mut(item.draw_function).unwrap();
                draw_function.draw(world, &mut tracked_pass, view_entity, item);
            }

            graph
                .set_output(
                    MainPass2dNodeCustom::OUT_TEXTURE,
                    target_custom.get_texture_view(),
                )
                .unwrap();
        }

        Ok(())
    }
}
