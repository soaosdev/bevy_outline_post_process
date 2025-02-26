#![warn(missing_docs)]

//! A plugin for the Bevy game engine which provides an outline post-process effect. The effect
//! makes use of a normal and depth prepass to generate outlines where significant differences in the values
//! occur.

use bevy::{
    asset::embedded_asset,
    core_pipeline::core_3d::graph::{Core3d, Node3d},
    prelude::*,
    render::{
        extract_component::{ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::{RenderGraphApp, ViewNodeRunner},
        RenderApp,
    },
};

use components::OutlinePostProcessSettings;
pub use nodes::OutlineRenderLabel;

/// Components used by this plugin.
pub mod components;
mod nodes;
mod resources;

/// Plugin which provides an outline post-processing effect.
pub struct OutlinePostProcessPlugin;

impl Plugin for OutlinePostProcessPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/shaders/outline_post_process.wgsl");

        app.add_plugins((
            UniformComponentPlugin::<components::OutlinePostProcessSettings>::default(),
            ExtractComponentPlugin::<components::OutlinePostProcessSettings>::default(),
        ))
        .add_systems(Update, update_shader_clip_planes);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<nodes::OutlineRenderNode>>(
                Core3d,
                nodes::OutlineRenderLabel,
            )
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    nodes::OutlineRenderLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<resources::OutlinePostProcessPipeline>();
    }
}

fn update_shader_clip_planes(
    mut settings_query: Query<(Ref<Projection>, &mut OutlinePostProcessSettings)>,
) {
    for (projection, mut settings) in settings_query.iter_mut() {
        if projection.is_changed() {
            if let Projection::Perspective(projection) = projection.into_inner() {
                settings.camera_near = projection.near;
            }
        }
    }
}
