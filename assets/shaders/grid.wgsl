#import bevy_sprite::mesh2d_view_bindings::globals 
#import bevy_render::view::View
#import bevy_pbr::forward_io::VertexOutput;

@group(0) @binding(0) var<uniform> view: View;

const minCellSize: f32 = 0.0001;
const minCellPixelWidth: f32 = 2.0;
const lineWidth: f32 = 2.0;
const thinColor: vec3<f32> = vec3<f32>(0.04, 0.04, 0.04);
const thickColor: vec3<f32> = vec3<f32>(0.2, 0.2, 0.2);

fn max2(v: vec2<f32>) -> f32 {
    return max(v.x, v.y);
}

fn log10(x: f32) -> f32 {
    return log2(x) / log2(10.0);
}

fn grid(uv: vec2<f32>) -> vec4<f32> {
    let dudv: vec2<f32> = vec2<f32>(
        length(vec2<f32>(dpdx(uv.x), dpdy(uv.x))),
        length(vec2<f32>(dpdx(uv.y), dpdy(uv.y)))
    );

    let lod: f32 = max(0.0, log10((max2(dudv) * minCellPixelWidth) / minCellSize) + 1.0);
    let fade: f32 = fract(lod);

    let lod0: f32 = minCellSize * pow(10.0, floor(lod));
    let lod1: f32 = lod0 * 10.0;
    let lod2: f32 = lod1 * 10.0;

    // Simulating modulus operation
    let modLod0: vec2<f32> = uv - lod0 * floor(uv / lod0);
    let modLod1: vec2<f32> = uv - lod1 * floor(uv / lod1);
    let modLod2: vec2<f32> = uv - lod2 * floor(uv / lod2);

    let lod0a: f32 = max2(vec2<f32>(1.0) - abs(clamp(modLod0 / dudv / lineWidth, vec2<f32>(0.0), vec2<f32>(1.0)) * 2.0 - vec2<f32>(1.0)));
    let lod1a: f32 = max2(vec2<f32>(1.0) - abs(clamp(modLod1 / dudv / lineWidth, vec2<f32>(0.0), vec2<f32>(1.0)) * 2.0 - vec2<f32>(1.0)));
    let lod2a: f32 = max2(vec2<f32>(1.0) - abs(clamp(modLod2 / dudv / lineWidth, vec2<f32>(0.0), vec2<f32>(1.0)) * 2.0 - vec2<f32>(1.0)));

    let color: vec3<f32> = select(
        select(thinColor, mix(thickColor, thinColor, fade), lod1a > 0.0),
        thickColor,
        lod2a > 0.0
    );
    let alpha: f32 = select(
        // select(lod1a * (1.0 - fade), lod1a, lod1a > 0.0),
        lod1a * (1.0 - fade),
        lod2a,
        lod2a > 0.0
    );

    return vec4<f32>(color, alpha);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // let ndc_pos = in.uv * 2.0 - 1.0;
    let ndc_pos = vec2<f32>(in.uv.x * 2.0 - 1.0, 1.0 - in.uv.y * 2.0); // Invert Y here

    let world_pos = (view.inverse_view_proj * vec4<f32>(ndc_pos.x, ndc_pos.y, 0.0, 1.0)).xy;

    return grid(world_pos);
}