#import bevy_sprite::{mesh2d_vertex_output::VertexOutput, mesh2d_view_bindings::globals};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> options: u32;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let index = f32(options & 7);
    let pattern = f32(options >> 3 & 3);
    let direction = f32(options >> 5 & 3);
    let animation = f32(options >> 7 & 7);

    let animated = vec2<f32>(
        f32(animation == 0) * 0
        + f32(animation == 1) * 0
        + f32(animation == 2) * 0
        + f32(animation == 3) * 0
        + f32(animation == 4) * 0
        + f32(animation == 5) * 0,
        f32(animation == 0) * 0
        + f32(animation == 1) * 0
        + f32(animation == 2) * 0
        + f32(animation == 3) * 0
        + f32(animation == 4) * 0
        + f32(animation == 5) * trunc(globals.time) % 4,
    );

    let offset = vec2(
        pattern + index % 4 * 3,
        f32(animation != 5) * direction + trunc(index / 4) * 4,
    );

    return textureSample(texture, texture_sampler, (in.uv + offset + animated) / vec2(12, 8));
}
