#version 450

layout(location = 0) in vec2 a_Pos;
layout(location = 1) in vec2 a_TexCoord;
layout(location = 2) in vec4 a_Color;
layout(location = 0) out vec2 v_TexCoord;
layout(location = 1) out vec4 v_Color;

layout(set = 0, binding = 0) uniform StableLocals {
    mat4 u_ProjectionView;
};

void main() {
    v_TexCoord = a_TexCoord;
    v_Color = a_Color;
    gl_Position = u_ProjectionView * vec4(a_Pos, 0.0, 1.0);
}