[[location 0]] var<in> v_TexCoord : vec2<f32>;
[[location 1]] var<in> v_Color : vec4<f32>;
[[location 0]] var<out> o_Target : vec4<f32>;

[[binding 1, set 0]] var<uniform> t_Color : texture_sampled_2d<f32>;
[[binding 2, set 0]] var<uniform> s_Color : sampler;

fn main() -> void {
    tex = textureSample(t_Color, s_Color, v_TexCoord);
    o_Target = tex * v_Color;
}

entry_point fragment = main;