#version 450

layout (location = 0) in vec3 uv;
layout (location = 0) out vec4 out_color;

layout(binding = 0) uniform sampler default_sampler;
layout(binding = 1) uniform texture2D color_texture;

float gamma = 1.5;

vec3 tonemapping(vec3 color) {
    float luma = dot(color, vec3(0.33, 0.33, 0.33));
    color += smoothstep(0.6, 2.5, luma) * luma * 0.5;
    luma = dot(color, vec3(0.2126, 0.7152, 0.0722));
    color /= (1. + luma);
    color = pow(color, vec3(1. / gamma));
    return color;
}

void main() {
    ivec2 texSize = textureSize(color_texture, 0);

    vec4 color = texelFetch(sampler2D(color_texture, default_sampler), ivec2(uv.xy * texSize), int(gl_SampleID)).rgba;
    out_color.xyz = tonemapping(color.xyz);
    out_color.w = color.w;
}
