pub const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 0) out vec4 v_Position;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    gl_Position = v_Position;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) in vec4 v_Position;
layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 1) uniform texture2DArray MyArrayTexture_texture;
layout(set = 1, binding = 2) uniform sampler MyArrayTexture_texture_sampler;

void main() {
    // Screen-space coordinates determine which layer of the array texture we sample.
    vec2 ss = v_Position.xy / v_Position.w;
    float layer = 0.0;
    if (ss.x > 0.0 && ss.y > 0.0) {
        layer = 0.0;
    } else if (ss.x < 0.0 && ss.y > 0.0) {
        layer = 1.0;
    } else if (ss.x > 0.0 && ss.y < 0.0) {
        layer = 2.0;
    } else {
        layer = 3.0;
    }

    // Convert to texture coordinates.
    vec2 uv = (ss + vec2(1.0)) / 2.0;

    o_Target = texture(sampler2DArray(MyArrayTexture_texture, MyArrayTexture_texture_sampler), vec3(uv, layer));
}
"#;
