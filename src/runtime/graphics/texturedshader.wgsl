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
    @location(9) velocity: f32,
    @location(10) white_key: f32,
    @location(11) color_factor: u32,
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
    @location(3) color_factor: u32,
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
    out.color_factor = instance.color_factor;

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


    if (in.color_factor == 8u)  // These are notes that have no uniformity
        { return mix(black_and_white, red_sample, in.velocity * 3.75); }    // Velocity needs to be multiplied because Midi stops at 128 

    if (in.color_factor == 4u)  // These are notes that are in some but not complete uniformity
        { return mix(black_and_white, orange_sample, in.velocity * 2.55); } // .. but beware as our table goes to 144

    if (in.color_factor == 2u) // these are notes that are in complete uniformity .. i.e. found in all other potential pitchgroups
        { return mix(black_and_white, green_sample, in.velocity * 1.25); } /// we will need to clamp and slope with a custom lerp

    if (in.color_factor == 1u) // These are the notes that we are playing
        { return mix(black_and_white, blue_sample, in.velocity * 4.25); } // TODO: We should switch all of these to a more comprehensive adding for the color factor with respect to the white key (invert and sub)


   return black_and_white;
}