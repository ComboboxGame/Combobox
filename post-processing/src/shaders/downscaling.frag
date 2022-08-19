#version 450

layout (location = 0) in vec3 uv;
layout (location = 0) out vec4 out_color;

#ifdef MSAA
#define TEXTURE texture2DMS
#define SAMPLER sampler2DMS
#else
#define TEXTURE texture2D
#define SAMPLER sampler2D
#endif

layout(binding = 0) uniform sampler default_sampler;
layout(binding = 1) uniform TEXTURE color_texture;
layout(binding = 3) uniform int step;

vec4 quadratic_threshold(vec4 color, float threshold, vec3 curve) {
    float br = max(max(color.r, color.g), color.b);

    float rq = clamp(br - curve.x, 0.0, curve.y);
    rq = curve.z * rq * rq;

    return color * max(rq, br - threshold) / max(br, 0.0001);
}

vec4 sample_13(vec2 scale) {
    vec4 a = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2(-1.0, -1.0) * scale);
    vec4 b = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2( 0.0, -1.0) * scale);
    vec4 c = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2( 1.0, -1.0) * scale);
    vec4 d = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2(-0.5, -0.5) * scale);
    vec4 e = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2( 0.5, -0.5) * scale);
    vec4 f = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2(-1.0,  0.0) * scale);
    vec4 g = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2( 0.0,  0.0) * scale);
    vec4 h = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2( 1.0,  0.0) * scale);
    vec4 i = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2(-0.5,  0.5) * scale);
    vec4 j = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2( 0.5,  0.5) * scale);
    vec4 k = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2(-1.0,  1.0) * scale);
    vec4 l = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2( 0.0,  1.0) * scale);
    vec4 m = texture(SAMPLER(color_texture, default_sampler), uv.xy + vec2( 1.0,  1.0) * scale);
    vec2 div = (1.0 / 4.0) * vec2(0.5, 0.125);

    vec4 o = (d + e + i + j) * div.x;
    o = o + (a + b + g + f) * div.y;
    o = o + (b + c + h + g) * div.y;
    o = o + (f + g + l + k) * div.y;
    o = o + (g + h + m + l) * div.y;

    return o;
}

void main() {
    ivec2 texSize = textureSize(color_texture, 0);
    vec2 scale = 1.0 / vec2(texSize);
    out_color = sample_13(scale);

    if (step == 1) {
        float threshold = 1.8;
        float knee = 0.1;
        vec3 curve = vec3(threshold - knee, knee * 2.0, 0.25 / knee);

        out_color = quadratic_threshold(out_color, threshold, curve);
        out_color = max(out_color, vec4(0.00001));
    }
}
