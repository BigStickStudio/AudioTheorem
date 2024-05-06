// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) color_factor: vec4<f32>,
    @location(10) white_key: f32,
}

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color_factor: vec4<f32>,
    @location(2) white_key: f32,
};

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    out.color_factor = instance.color_factor;
    out.white_key = instance.white_key;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * model_matrix * model.position;

    return out;
}

// Fragment shader
@group(0) @binding(0)
var white_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var white_sampler: sampler;
@group(0) @binding(2)
var black_diffuse: texture_2d<f32>;
@group(0) @binding(3)
var black_sampler: sampler;
@group(0) @binding(4)
var blue_diffuse: texture_2d<f32>;
@group(0) @binding(5)
var blue_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var white_sample = textureSample(white_diffuse, white_sampler, in.tex_coords);
    var black_sample = textureSample(black_diffuse, black_sampler, in.tex_coords);
    var blue_sample = textureSample(blue_diffuse, blue_sampler, in.tex_coords);

    return mix(blue_sample, (mix(black_sample, white_sample, in.white_key) * 0.5) * 2.0, in.color_factor.a);
}