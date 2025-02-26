use bevy::{
    core_pipeline::prepass::{DeferredPrepass, DepthPrepass, NormalPrepass},
    prelude::*,
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
};

/// Component which, when inserted into an entity with a camera and normal prepass, enables an outline effect for that
/// camera.
#[derive(Component, ShaderType, ExtractComponent, PartialEq, Clone)]
#[require(NormalPrepass, DepthPrepass, DeferredPrepass, Msaa(|| Msaa::Off))]
pub struct OutlinePostProcessSettings {
    /// Weight of outlines in pixels.
    weight: f32,
    /// A threshold for normal differences, values below this threshold will not become outlines.
    /// Higher values will result in more outlines which may look better on smooth surfaces.
    normal_threshold: f32,
    /// A threshold for depth differences (in units), values below this threshold will not become outlines.
    /// Higher values will result in more outlines which may look better on smooth surfaces.
    depth_threshold: f32,
    /// Whether to use adaptive outlines. White outlines will be drawn around darker objects, while black ones will be drawn around lighter ones.
    adaptive: u32,
    /// Near plane depth of camera, used for linearization of depth buffer values
    pub(crate) camera_near: f32,
}

impl OutlinePostProcessSettings {
    /// Create a new instance with the given settings
    pub fn new(
        weight: f32,
        normal_threshold: f32,
        depth_threshold: f32,
        adaptive: bool,
    ) -> Self {
        Self {
            weight,
            normal_threshold,
            adaptive: adaptive as u32,
            depth_threshold,
            camera_near: 0.0,
        }
    }
}

impl Default for OutlinePostProcessSettings {
    fn default() -> Self {
        Self {
            weight: 1.0,
            normal_threshold: 0.0,
            adaptive: 0,
            depth_threshold: 0.05,
            camera_near: 0.0,
        }
    }
}
