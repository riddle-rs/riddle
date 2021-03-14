[[location(0)]]
var<in> in_position: vec3<f32>;
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
    out_position = r_locals.transform * vec4<f32>(in_position, 1.0);
}

[[location(0)]]
var<out> out_color_fs: vec4<f32>;

[[stage(fragment)]]
fn fs_main() {
    out_color_fs = vec4<f32>(1.0, 1.0, 1.0, 1.0);
}