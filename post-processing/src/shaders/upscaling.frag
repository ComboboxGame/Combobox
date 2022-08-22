#version 450

layout (location = 0) in vec3 uv;
layout (location = 0) out vec4 out_color;

layout(binding = 0) uniform sampler default_sampler;
layout(binding = 1) uniform texture2D first_texture; // smaller
layout(binding = 2) uniform texture2D second_texture; // bigger
layout(binding = 3) uniform uvec4 step;

vec4 sample_tent(vec2 scale) {
    vec4 d = vec4(1.0, 1.0, -1.0, 0.0);

    vec4 s =  texture(sampler2D(first_texture, default_sampler), uv.xy - d.xy * scale)
            + texture(sampler2D(first_texture, default_sampler), uv.xy - d.wy * scale) * 2.0
            + texture(sampler2D(first_texture, default_sampler), uv.xy - d.zy * scale)
            + texture(sampler2D(first_texture, default_sampler), uv.xy + d.zw * scale) * 2.0
            + texture(sampler2D(first_texture, default_sampler), uv.xy       		   ) * 4.0
            + texture(sampler2D(first_texture, default_sampler), uv.xy + d.xw * scale) * 2.0
            + texture(sampler2D(first_texture, default_sampler), uv.xy + d.zy * scale)
            + texture(sampler2D(first_texture, default_sampler), uv.xy + d.wy * scale) * 2.0
            + texture(sampler2D(first_texture, default_sampler), uv.xy + d.xy * scale);

    return s / 16.0;
}

void main() {
    ivec2 texSize = textureSize(first_texture, 0);
    vec2 scale = 1.0 / vec2(texSize);

    vec4 up_sample = sample_tent(scale * 1.0);
    vec4 color = texture(sampler2D(second_texture, default_sampler), uv.xy);
    
    if (step.x == 1) {
        out_color = vec4(color.rgb * 1.0 + up_sample.rgb * 0.5, 1.0);
    } else {
        out_color = vec4(color.rgb + up_sample.rgb * 0.6, 1.0);
    }
}
