#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
#import bevy_pbr::{ rgb9e5 };
#import bevy_pbr::view_transformations::{
    perspective_camera_near,
};
#import bevy_pbr::mesh_view_bindings as view_bindings;

struct OutlinePostProcessSettings {
	weight: f32,
	normal_threshold: f32,
	depth_threshold: f32,
    adaptive: u32,
    camera_near: f32,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;
@group(0) @binding(2) var normal_texture: texture_2d<f32>;
@group(0) @binding(3) var normal_sampler: sampler;
@group(0) @binding(4) var depth_texture: texture_depth_2d;
@group(0) @binding(5) var depth_sampler: sampler;
@group(0) @binding(6) var<uniform> settings: OutlinePostProcessSettings;

@fragment
fn fragment(
	in: FullscreenVertexOutput
) -> @location(0) vec4<f32> {
    let screen_color = textureSample(screen_texture, screen_sampler, in.uv);
    let luma = (0.2126 * screen_color.r + 0.7152 * screen_color.g + 0.0722 * screen_color.b);

    let outline_width = settings.weight / vec2f(textureDimensions(screen_texture)); 
    let uv_top = vec2f(in.uv.x, in.uv.y - outline_width.y);
    let uv_right = vec2f(in.uv.x + outline_width.x, in.uv.y);
    let uv_top_right = vec2f(in.uv.x + outline_width.x, in.uv.y - outline_width.y);

    // NORMAL FACTOR {{{
        let normal = textureSample(normal_texture, normal_sampler, in.uv);
        let normal_top = textureSample(normal_texture, normal_sampler, uv_top);
        let normal_right = textureSample(normal_texture, normal_sampler, uv_right);
        let normal_top_right = textureSample(normal_texture, normal_sampler, uv_top_right);

        let normal_delta_top = abs(normal - normal_top);
        let normal_delta_right = abs(normal - normal_right);
        let normal_delta_top_right = abs(normal - normal_top_right);

        let normal_delta_max = max(normal_delta_top, max(normal_delta_right, normal_delta_top_right));
        let normal_delta_raw = max(normal_delta_max.x, max(normal_delta_max.y, normal_delta_max.z));

        let show_outline_normal = f32(normal_delta_raw > settings.normal_threshold);
        let normal_outline = vec4f(show_outline_normal, show_outline_normal, show_outline_normal, 0.0);
    // }}}

    let frag_width = settings.weight / vec2f(textureDimensions(screen_texture)); 
    let depth_uv_top_right = vec2f(in.uv.x + frag_width.x, in.uv.y - frag_width.y);
    let depth_uv_top_left = vec2f(in.uv.x - frag_width.x, in.uv.y - frag_width.y);
    let depth_uv_bottom_right = vec2f(in.uv.x + frag_width.x, in.uv.y + frag_width.y);
    let depth_uv_bottom_left = vec2f(in.uv.x - frag_width.x, in.uv.y + frag_width.y);
    // DEPTH FACTOR {{{
        let depth_color = textureSample(depth_texture, depth_sampler, in.uv);
        let depth = linearize_depth(depth_color);
        let depth_top_right = linearize_depth(textureSample(depth_texture, depth_sampler, depth_uv_top_right));
        let depth_top_left = linearize_depth(textureSample(depth_texture, depth_sampler, depth_uv_top_left));
        let depth_bottom_right = linearize_depth(textureSample(depth_texture, depth_sampler, depth_uv_bottom_right));
        let depth_bottom_left = linearize_depth(textureSample(depth_texture, depth_sampler, depth_uv_bottom_left));

        var depth_delta = 0.0;
        depth_delta = depth_delta + depth - depth_top_right;
        depth_delta = depth_delta + depth - depth_top_left;
        depth_delta = depth_delta + depth - depth_bottom_right;
        depth_delta = depth_delta + depth - depth_bottom_left;

        var depth_outline = vec4(step(settings.depth_threshold, depth_delta));
        if depth_color == 0.0 {
            depth_outline = vec4(0.0);
        }
    // }}}
    
    var outline = (depth_outline + normal_outline);
    if settings.adaptive != 0 && luma < 0.5 {
        outline = outline * -1;
    }
	return screen_color - outline ;
}

fn linearize_depth(depth: f32) -> f32 {
    return settings.camera_near / depth;
}
