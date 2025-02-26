use bevy::{
    core_pipeline::prepass::ViewPrepassTextures,
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::ComponentUniforms,
        render_graph::{NodeRunError, RenderGraphContext, RenderLabel, ViewNode},
        render_resource::{
            BindGroupEntries, Operations, PipelineCache, RenderPassColorAttachment,
            RenderPassDescriptor,
        },
        renderer::RenderContext,
        view::ViewTarget,
    },
};

use super::components;
use super::resources;

/// Label for outline post-process render node.
#[derive(RenderLabel, Clone, Eq, PartialEq, Hash, Debug)]
pub struct OutlineRenderLabel;

#[derive(Default)]
pub struct OutlineRenderNode;

impl ViewNode for OutlineRenderNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static ViewPrepassTextures,
        &'static components::OutlinePostProcessSettings,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, view_prepass_textures, _): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let render_pipeline = world.resource::<resources::OutlinePostProcessPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let Some(pipeline) = pipeline_cache.get_render_pipeline(render_pipeline.pipeline_id) else {
            warn!("Failed to get render pipeline from cache, skipping...");
            return Ok(());
        };

        let uniforms =
            world.resource::<ComponentUniforms<components::OutlinePostProcessSettings>>();
        let Some(uniform_binding) = uniforms.uniforms().binding() else {
            error!("Failed to get settings uniform binding");
            return Ok(());
        };

        let Some(normal_buffer_view) = view_prepass_textures.normal_view() else {
            error!("Failed to get normal buffer view");
            return Ok(());
        };

        let Some(depth_buffer_view) = view_prepass_textures.depth_view() else {
            error!("Failed to get normal buffer view");
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "outline_post_process_bind_group",
            &render_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &render_pipeline.screen_sampler,
                normal_buffer_view,
                &render_pipeline.normal_sampler,
                depth_buffer_view,
                &render_pipeline.depth_sampler,
                uniform_binding,
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("outline_post_process_render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                ops: Operations::default(),
                resolve_target: None,
            })],
            timestamp_writes: None,
            depth_stencil_attachment: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);
        Ok(())
    }
}
