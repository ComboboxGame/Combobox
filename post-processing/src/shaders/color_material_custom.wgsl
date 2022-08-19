#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings

struct ColorMaterial {
    color: vec4<f32>,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
};
let COLOR_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;

@group(1) @binding(0)
var<uniform> material: ColorMaterial;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;
@group(1) @binding(3)
var normal: texture_2d<f32>;
@group(1) @binding(4)
var normal_sampler: sampler;

@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    #import bevy_sprite::mesh2d_vertex_output
};

fn rot(n: i32, p: vec2<i32>, r: vec2<i32>) -> vec2<i32> {
    if (r.y == 0) {
        if (r.x == 1) {
            return vec2(n - 1 - p.y, n - 1 - p.x);
        }
        return vec2(p.y, p.x);
    }
    return p;
}

fn d_to_xy(n: i32, d: i32) -> vec2<i32> {
    var t = d;
    var p = vec2(0, 0);
    for (var s = 1; s < n; s = s * 2) {
        let r = vec2(1 & (t / 2), 1 & (t ^ (1 & (t / 2))));
        p = rot(s, p, r);
        p = p + s * r;
        t = t / 4;
    }
    return p;
}

fn xy_to_d(n: i32, pp: vec2<i32>) -> i32 {
    var d = 0;
    var p = pp;
    for (var s = n/2; s > 0; s = s / 2) {
        let r = vec2(i32((p.x & s) > 0), i32((p.y & s) > 0));
        d = d + s*s*((3 * r.x) ^ r.y);
        p = rot(n, p, r);
    }
    return d;
}

fn hsl2rgb(c: vec3<f32>) -> vec3<f32>
{
    var x = c.x*6.0+vec3(0.0,4.0,2.0);
    var d = vec3(x.x % 6.0,x.y % 6.0,x.z % 6.0);
    var rgb = clamp(abs(d - 3.0) - 1.0, vec3(0.0), vec3(1.0));
    return c.z + c.y * (rgb - 0.5)*(1.0 - abs(2.0*c.z - 1.0));
}

fn get_hilbert_color(uv: vec2<f32>) -> vec3<f32> {
    let N = 32;
    let p = vec2<i32>(i32(f32(N) * uv.x), i32(f32(N) * uv.y));
    let d = xy_to_d(N, p);
    let dd = (f32(d) / f32(N * N));

    let br = 1.0 - pow(clamp(abs(dd - material.color.x) * 4.0, 0.0, 1.0), 0.15);
    let br2 = 1.0 - pow(clamp(abs(dd - material.color.z) * 4.0, 0.0, 1.0), 0.15);

    if (d > 0 && d < N * N - 1) {
        let prev = d_to_xy(N, d - 1);
        let next = d_to_xy(N, d + 1);
        let uvl = (uv * f32(N) - vec2(f32(p.x), f32(p.y)));
        let a = vec2(0.5, 0.5);
        let a_prev = a + vec2<f32>(prev - p);
        let a_next = a + vec2<f32>(next - p);

        if (all(min(a, a_next) - 0.1 < uvl) && all(max(a, a_next) + 0.1 > uvl)
            ||
            all(min(a, a_prev) - 0.1 < uvl) && all(max(a, a_prev) + 0.1 > uvl)) {
            return hsl2rgb(vec3((dd + material.color.y) % 1.0, 0.8, 0.5)) * (1.4 + max(br, br2) * 28.0);
        }
    }
    discard;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = material.color;
    if ((material.flags & COLOR_MATERIAL_FLAGS_TEXTURE_BIT) != 0u) {
#ifdef VERTEX_COLORS
        output_color = output_color * textureSample(texture, texture_sampler, in.uv) * in.color;
#else
        output_color = output_color * textureSample(texture, texture_sampler, in.uv);
#endif
        /*var N = textureSample(normal, normal_sampler, in.uv).xyz;
        let gamma = 2.2;
        N = vec3(pow(N.x, 1.0 / gamma), pow(N.y, 1.0 / gamma), pow(N.z, 1.0 / gamma)) * 2.0 - 1.0;

        N = normalize((mat3x3<f32>(
                                   mesh.inverse_transpose_model[0].xyz,
                                   mesh.inverse_transpose_model[1].xyz,
                                   mesh.inverse_transpose_model[2].xyz
                               ) * N));
        let light = clamp(dot(normalize(vec3(0.1, 0.3, 0.1)), N), 0.0, 1.0) * 2.0;
        output_color = vec4(output_color.xyz * light, output_color.w);*/
    }

    return vec4(get_hilbert_color(in.uv), 1.0);
}
