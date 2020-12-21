#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 3) in float Vertex_Voxel_Material;
layout(location = 4) out float v_Vox_Mat;
layout(location = 5) in float Vertex_AO;
layout(location = 6) out float v_AO;

layout(location = 0) out vec3 v_Position;
layout(location = 1) out vec3 v_Normal;
layout(location = 2) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 2, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Normal = (Model * vec4(Vertex_Normal, 1.0)).xyz;
    v_Normal = mat3(Model) * Vertex_Normal;
    v_Position = (Model * vec4(Vertex_Position, 1.0)).xyz;
    v_Uv = Vertex_Uv;
    gl_Position = ViewProj * vec4(v_Position, 1.0);
    v_Vox_Mat = Vertex_Voxel_Material;
    
    vec4 ao_curve = vec4(0.0, 0.65, 0.75, 0.9);
    float ao = Vertex_AO;
    if(ao == 0.0) {
        v_AO = ao_curve.x;
    }
    if(ao == 1.0) {
        v_AO = ao_curve.y;
    }
    if(ao == 2.0) {
        v_AO = ao_curve.z;
    }
    if(ao == 3.0) {
        v_AO = ao_curve.w;
    }

}