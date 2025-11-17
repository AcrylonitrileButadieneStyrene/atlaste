#import bevy_sprite::{mesh2d_vertex_output::VertexOutput, mesh2d_view_bindings::globals};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> options: u32;

// the width and height are transmited in terms of tiles, not in pixels
// so the size needs to be multiplied by this constant.
// there are 16 pixels per tile in r2k.
const TEXTURE_SCALE: f32 = 16;
const SCROLL_SPEED: f32 = 400; // random

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let width = f32((options) & 1023);
    let height = f32((options >> 9) & 1023);
    let size = vec2(width, height) * TEXTURE_SCALE;

    // todo: it seems that this is meant to move proportionally to the size of the image
    let horizontal = f32(((options >> 18) & 63)) - 15;
    let vertical = f32(((options >> 23) & 63)) - 15;
    let disable = (f32(((options >> 28) & 1)) - 1) * -1; // if true 0 else 1
    let speed = vec2(horizontal, vertical);
    let uv = fract(in.uv - vec2(horizontal, vertical) * globals.time / SCROLL_SPEED * disable);
    
    return textureSample(texture, texture_sampler, fract(uv * size / vec2<f32>(textureDimensions(texture))));
}
