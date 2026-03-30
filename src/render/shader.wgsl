struct Uniforms {
    proj_matrix: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    model_matrix: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = uniforms.model_matrix * vec4<f32>(vertex.position, 1.0);
    let view_pos = uniforms.view_matrix * world_pos;
    out.clip_position = uniforms.proj_matrix * view_pos;
    out.color = vertex.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
