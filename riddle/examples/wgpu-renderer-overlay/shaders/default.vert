#version 450

layout(location = 0) in vec3 a_Pos;

layout(set = 0, binding = 0) uniform StableLocals {
    mat4 u_ProjectionView;
};

void main() {
    gl_Position = u_ProjectionView * vec4(a_Pos, 1.0);
}