//////
// Vertex shader
//////
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
    @location(9) velocity: f32,
    @location(10) white_key: f32,
    @location(11) harmony: f32,
}

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) velocity: f32,
    @location(2) white_key: f32,
    @location(3) harmony: f32,
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

    out.clip_position = camera.view_proj * model_matrix * model.position;
    out.tex_coords = model.tex_coords;
    out.velocity = instance.velocity;
    out.white_key = instance.white_key;
    out.harmony = instance.harmony;

    return out;
}

//////
// Fragment shader
//////
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
@group(0) @binding(6)
var green_diffuse: texture_2d<f32>;
@group(0) @binding(7)
var green_sampler: sampler;
@group(0) @binding(8)
var orange_diffuse: texture_2d<f32>;
@group(0) @binding(9)
var orange_sampler: sampler;
@group(0) @binding(10)
var red_diffuse: texture_2d<f32>;
@group(0) @binding(11)
var red_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var white_sample = textureSample(white_diffuse, white_sampler, in.tex_coords);
    var black_sample = textureSample(black_diffuse, black_sampler, in.tex_coords);
    var blue_sample = textureSample(blue_diffuse, blue_sampler, in.tex_coords);
    var green_sample = textureSample(green_diffuse, green_sampler, in.tex_coords);
    var orange_sample = textureSample(orange_diffuse, orange_sampler, in.tex_coords);
    var red_sample = textureSample(red_diffuse, red_sampler, in.tex_coords);

    var black_and_white = mix(black_sample, white_sample, in.white_key);

    let velocity_factor = in.velocity / 100.0;


    if (in.harmony == 0f)
        { return mix(black_and_white, blue_sample, velocity_factor); } 

    if (in.harmony >= 1f)
        { 
            //return mix(mix(green_sample, orange_sample, (in.harmony / 255.0)), black_and_white, in.velocity * 4.25); 
            let harmony_factor = in.harmony / 255.0;
            let green_orange = mix(green_sample, orange_sample, harmony_factor);
            return mix(black_and_white, green_orange, velocity_factor);
        }

   return black_and_white;
}