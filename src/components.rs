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
    /// Color of outlines.
    color: LinearRgba,
    /// A threshold for normal differences, values below this threshold will not become outlines.
    /// Higher values will result in more outlines which may look better on smooth surfaces.
    normal_threshold: f32,
    /// A threshold for depth differences (in units), values below this threshold will not become outlines.
    /// Higher values will result in more outlines which may look better on smooth surfaces.
    depth_threshold: f32,
    /// Luma threshold to invert outline color. A value of `1.0` means this feature is disabled.
    adaptive_threshold: f32,
    /// Near plane depth of camera, used for linearization of depth buffer values
    pub(crate) camera_near: f32,
}

impl OutlinePostProcessSettings {
    /// Create a new instance with the given settings
    pub fn new(
        weight: f32,
        color: LinearRgba,
        normal_threshold: f32,
        depth_threshold: f32,
        adaptive_threshold: f32,
    ) -> Self {
        Self {
            weight,
            color,
            normal_threshold,
            depth_threshold,
            adaptive_threshold,
            camera_near: 0.0,
        }
    }
}

impl Default for OutlinePostProcessSettings {
    fn default() -> Self {
        Self {
            weight: 1.0,
            color: LinearRgba::BLACK,
            normal_threshold: 0.01,
            depth_threshold: 0.05,
            adaptive_threshold: 1.0,
            camera_near: 0.0,
        }
    }
}
