#version 450

layout (location = 0) in vec3 vertex;
layout (location = 0) out vec3 uv;

void main() {
    gl_Position = vec4(vertex.xy * 2 - 1, vertex.z, 1.0);
    uv = vec3(vertex.x, 1 - vertex.y, vertex.z);
}
