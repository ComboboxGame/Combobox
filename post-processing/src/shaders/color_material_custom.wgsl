#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings


let MAX_LIGHTS_NUM: i32 = 16;

struct Lights {
    lights_num: u32,
    positions: array<vec4<f32>, MAX_LIGHTS_NUM>,
    colors: array<vec4<f32>, MAX_LIGHTS_NUM>,
    ambient: vec4<f32>,
}

struct ColorMaterial {
    color: vec4<f32>,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
    lights: Lights,
};

let COLOR_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;
let COLOR_MATERIAL_FLAGS_EMISSIVE_BIT: u32 = 2u;

@group(1) @binding(0)
var<uniform> material: ColorMaterial;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;
@group(1) @binding(3)
var emissive: texture_2d<f32>;
@group(1) @binding(4)
var emissive_sampler: sampler;

@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    #import bevy_sprite::mesh2d_vertex_output
};

fn hsl2rgb(c: vec3<f32>) -> vec3<f32>
{
    var x = c.x*6.0+vec3(0.0,4.0,2.0);
    var d = vec3(x.x % 6.0,x.y % 6.0,x.z % 6.0);
    var rgb = clamp(abs(d - 3.0) - 1.0, vec3(0.0), vec3(1.0));
    return c.z + c.y * (rgb - 0.5)*(1.0 - abs(2.0*c.z - 1.0));
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = material.color;
    if ((material.flags & COLOR_MATERIAL_FLAGS_TEXTURE_BIT) != 0u) {
        var texture_color = textureSample(texture, texture_sampler, in.uv);
        if ((material.flags & COLOR_MATERIAL_FLAGS_EMISSIVE_BIT) != 0u) {
            let emissive = textureSample(emissive, emissive_sampler, in.uv);
            texture_color = texture_color + vec4(emissive.rgb * emissive.a, 0.0) * 5.0;
        }
#ifdef VERTEX_COLORS
        output_color = output_color * texture_color * in.color;
#else
        output_color = output_color * texture_color;
#endif
    }

    if (output_color.a < 0.01) {
        output_color = vec4(output_color.rgb, 0.0);
    }

    var light = material.lights.ambient.rgb;

    for (var i:u32 = 0u; i < material.lights.lights_num; i++) {
        let radius = material.lights.positions[i].w;
        let dist = length(in.world_position.xy - material.lights.positions[i].xy);
        if (dist >= radius) {
            // continue;
        }
        //let intensity = sqrt(1.0 - dist * dist / (radius * radius));
        let x = dist / radius;
        let intensity = exp(-x*x*4.0);
        light += intensity * material.lights.colors[i].rgb;
    }

    light = light / (1.0 + dot(light, vec3(0.33, 0.33, 0.33)));

    output_color = vec4(output_color.rgb * light, output_color.w);

    return output_color;
}
