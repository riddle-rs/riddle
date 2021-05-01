struct VertexOutput {
    [[location(0)]] tex_coord: vec2<f32>;
    [[location(1)]] color: vec4<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[block]]
struct Locals {
    transform: mat4x4<f32>;
};
[[group(0), binding(0)]]
var r_locals: Locals;

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] in_position: vec2<f32>,
    [[location(1)]] in_tex_coord: vec2<f32>,
    [[location(2)]] in_color: vec4<f32>
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coord = in_tex_coord;
    out.color = in_color;
    out.position = r_locals.transform * vec4<f32>(in_position.x, in_position.y, 0.0, 1.0);
    return out;
}

[[group(0), binding(1)]]
var r_color: texture_2d<f32>;
[[group(0), binding(2)]]
var r_sampler: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var tex: vec4<f32> = textureSample(r_color, r_sampler, in.tex_coord);
    return tex * in.color;
}