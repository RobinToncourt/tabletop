#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var texture: texture_2d<f32>;
@group(2) @binding(1) var texture_sampler: sampler;

struct OutlineMaterial {
    color: vec4<f32>,
    region: vec4<f32>,
    outline_px: f32,
    mesh_scale: f32,
};
@group(2) @binding(2) var<uniform> material: OutlineMaterial;

fn sample_alpha(uv: vec2<f32>) -> f32 {
    // Anything outside the sprite's own region counts as transparent,
    // even if the underlying texture pixel there happens to be opaque
    // (e.g. a neighboring sprite packed into the same atlas).
    if (uv.x < material.region.x || uv.y < material.region.y ||
        uv.x > material.region.z || uv.y > material.region.w) {
        return 0.0;
    }
    return textureSample(texture, texture_sampler, uv).a;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let region_size = material.region.zw - material.region.xy;
    let padding = region_size * (material.mesh_scale - 1.0) * 0.5;
    let uv = material.region.xy - padding + in.uv * (region_size + padding * 2.0);

    if (sample_alpha(uv) > 0.5) {
        discard; // interior of the sprite — let the real sprite render it
    }

    let texel = material.outline_px / vec2<f32>(textureDimensions(texture));
    var edge = 0.0;
    for (var x = -1; x <= 1; x = x + 1) {
        for (var y = -1; y <= 1; y = y + 1) {
            edge = max(edge, sample_alpha(uv + vec2<f32>(f32(x), f32(y)) * texel));
        }
    }

    if (edge > 0.5) {
        return material.color;
    }
    discard;
}