#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

struct OutlinePostProcessSettings {
	weight: f32,
    color: vec4<f32>,
	normal_threshold: f32,
	depth_threshold: f32,
    adaptive_threshold: f32,
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
    let uv_bottom = vec2f(in.uv.x, in.uv.y + outline_width.y);
    let uv_right = vec2f(in.uv.x + outline_width.x, in.uv.y);
    let uv_left = vec2f(in.uv.x - outline_width.x, in.uv.y);
    let uv_top_right = vec2f(in.uv.x + outline_width.x, in.uv.y - outline_width.y);

    // NORMAL FACTOR {{{
        let normal = textureSample(normal_texture, normal_sampler, in.uv).xyz;
        let normal_top = textureSample(normal_texture, normal_sampler, uv_top).xyz;
        let normal_right = textureSample(normal_texture, normal_sampler, uv_right).xyz;
        let normal_top_right = textureSample(normal_texture, normal_sampler, uv_top_right).xyz;

        let normal_delta_top = abs(normal - normal_top);
        let normal_delta_right = abs(normal - normal_right);
        let normal_delta_top_right = abs(normal - normal_top_right);

        let normal_top_sum = max(normal_delta_top.x, max(normal_delta_top.y, normal_delta_top.z));
        let normal_right_sum = max(normal_delta_right.x, max(normal_delta_right.y, normal_delta_right.z));
        let normal_top_right_sum = max(normal_delta_top_right.x, max(normal_delta_top_right.y, normal_delta_top_right.z));

        var normal_difference = step(settings.normal_threshold, max(normal_top_sum, max(normal_right_sum, normal_top_right_sum)));
        let normal_outline = normal_difference;
    // }}}

    // DEPTH FACTOR {{{
        let depth_color = textureSample(depth_texture, depth_sampler, in.uv);
        let depth_color_top = textureSample(depth_texture, depth_sampler, uv_top);
        let depth_color_bottom = textureSample(depth_texture, depth_sampler, uv_bottom);
        let depth_color_right = textureSample(depth_texture, depth_sampler, uv_right);
        let depth_color_left = textureSample(depth_texture, depth_sampler, uv_left);

        let depth = linearize_depth(depth_color);
        let depth_top = linearize_depth(depth_color_top);
        let depth_bottom = linearize_depth(depth_color_bottom);
        let depth_right = linearize_depth(depth_color_right);
        let depth_left = linearize_depth(depth_color_left);

        var depth_delta = 0.0;
        depth_delta = depth_delta + depth - depth_top;
        depth_delta = depth_delta + depth - depth_bottom;
        depth_delta = depth_delta + depth - depth_right;
        depth_delta = depth_delta + depth - depth_left;

        var depth_outline = step(settings.depth_threshold, depth_delta);
        if depth_color == 0.0 {
            depth_outline = 0.0;
        }
    // }}}
    
    var outline = saturate(depth_outline + normal_outline);
    if (1.0 - luma) > settings.adaptive_threshold {
        outline = outline * -1;
    }
    if (outline == 1.0) {
        return settings.color;
    }
    if (outline == -1.0) {
        return vec4(vec3(1.0) - settings.color.xyz, 1.0);
    }
	return screen_color; 
}

fn linearize_depth(depth: f32) -> f32 {
    if depth == 0.0 {
        return 0.0;
    }
    return settings.camera_near / depth;
}
