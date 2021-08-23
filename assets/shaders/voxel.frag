#version 450

const int MAX_LIGHTS = 10;

struct Light {
    mat4 proj;
    vec4 pos;
    vec4 color;
};

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec2 v_Uv;
layout(location = 4) in float v_Vox_Mat;
layout(location = 6) in float v_AO;
layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Lights {
    vec3 AmbientColor;
    uvec4 NumLights;
    Light SceneLights[MAX_LIGHTS];
};

layout(set = 3, binding = 0) uniform TerrainMaterial_albedo {
    vec4 Albedo;
};

layout(set = 3, binding = 1) uniform texture2DArray TerrainMaterial_albedo_texture;
layout(set = 3, binding = 2) uniform sampler TerrainMaterial_albedo_texture_sampler;

layout(set = 3, binding = 3) uniform TerrainMaterial_custom_val {
    float custom_val;
};

void main() {

    vec4 output_color = Albedo - vec4(v_AO, v_AO, v_AO, 0.0);
    //output_color = Albedo; // Disables AO
    
    output_color *= texture(
        sampler2DArray(TerrainMaterial_albedo_texture, TerrainMaterial_albedo_texture_sampler),
        vec3(v_Uv, v_Vox_Mat));

    output_color *= vec4(vec3(v_Vox_Mat), 1.0);

    vec3 normal = normalize(v_Normal);
    // accumulate color
    vec3 color = AmbientColor;
    for (int i=0; i<int(NumLights.x) && i<MAX_LIGHTS; ++i) {
        Light light = SceneLights[i];
        // compute Lambertian diffuse term
        vec3 light_dir = normalize(light.pos.xyz - v_Position);
        float diffuse = max(0.0, dot(normal, light_dir));
        // float dist = (distance(light.pos.xyz, v_Position) / 1.0);
        // diffuse = diffuse / (dist * dist);
        // add light contribution
        color += diffuse * light.color.xyz;
    }
    output_color.xyz *= color;

    // multiply the light by material color
    o_Target = output_color;
}
