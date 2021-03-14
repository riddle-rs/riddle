[[location(0)]]
var<in> in_position: vec2<f32>;
[[location(1)]]
var<in> in_tex_coord_vs: vec2<f32>;
[[location(2)]]
var<in> in_color_vs: vec4<f32>;
[[location(0)]]
var<out> out_tex_coord: vec2<f32>;
[[location(1)]]
var<out> out_color_vs: vec4<f32>;
[[builtin(position)]]
var<out> out_position: vec4<f32>;

[[block]]
struct Locals {
    transform: mat4x4<f32>;
};
[[group(0), binding(0)]]
var r_locals: Locals;

[[stage(vertex)]]
fn vs_main() {
    out_tex_coord = in_tex_coord_vs;
    out_color_vs = in_color_vs;
    out_position = r_locals.transform * vec4<f32>(in_position.x, in_position.y, 0.0, 1.0);
}

[[location(0)]]
var<in> in_tex_coord_fs: vec2<f32>;
[[location(1)]]
var<in> in_color_fs: vec4<f32>;
[[location(0)]]
var<out> out_color_fs: vec4<f32>;
[[group(0), binding(1)]]
var r_color: texture_2d<f32>;
[[group(0), binding(2)]]
var r_sampler: sampler;

[[stage(fragment)]]
fn fs_main() {
    var tex: vec4<f32> = textureSample(r_color, r_sampler, in_tex_coord_fs);
    out_color_fs = tex * in_color_fs;
}