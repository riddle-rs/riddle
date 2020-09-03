#version 450

[[location 0]] var<in> v_TexCoord : vec2<f32>;
[[location 1]] var<in> v_Color : vec4<f32>;
[[location 0]] var<out> o_Target : vec4<f32>;

layout(location = 0) in vec2 v_TexCoord;
layout(location = 1) in vec4 v_Color;
layout(location = 0) out vec4 o_Target;
layout(set = 0, binding = 1) uniform texture2D t_Color;
layout(set = 0, binding = 2) uniform sampler s_Color;

void main() {
    vec4 tex = texture(sampler2D(t_Color, s_Color), v_TexCoord);
    o_Target = tex * v_Color;
}

entry_point vertex = main;