//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::Dynamic;
use cgmath::prelude::*;
use super::super::Disposition;

#[derive(Debug)]
pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub index: u32,
    pub dynamic: Dynamic,
    pub disposition: Disposition,
}

impl Instance {
    // These colors are not even used properly

     // We do 1.25 Time Scale in the shader to throttle the color into the key
     pub fn dynamic_velocity(&self) -> f32 {
        match self.dynamic {
            Dynamic::Off => 0.0,
            Dynamic::Pianissimissimo => 0.1,
            Dynamic::Pianissimo => 0.125,
            Dynamic::Piano => 0.2,
            Dynamic::MezzoPiano => 0.25,
            Dynamic::MezzoForte => 0.3,
            Dynamic::Forte => 0.35,
            Dynamic::Fortissimo => 0.4,
            Dynamic::Fortissimissimo => 0.45
        }
    }

    fn index_to_white_key(&self) -> f32 {
        match self.index {
            1 => 0.0,
            3 => 0.0,
            6 => 0.0,
            8 => 0.0,
            10 => 0.0,
            _ => 1.0
        }
    }

    // Used to populate our instance buffer and send it to the GPU as defined below and in the shader
    pub fn raw(&self) -> RawInstance {
        RawInstance {
            model: (cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation)).into(),
            velocity: self.dynamic_velocity(),
            color_factor: self.disposition.as_u32(),
            white_key : self.index_to_white_key(),
        }
    }

    // This is a trigger key function that will be used to trigger the key .. that's all
    pub fn trigger_key(&mut self, velocity: u8, disposition: u8) {
        self.dynamic = Dynamic::from_velocity(velocity);
        self.disposition = Disposition::from_u8(disposition);
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawInstance {
    pub model: [[f32; 4]; 4],
    pub velocity: f32,
    pub white_key: f32,
    pub color_factor: u32,
}

// Used for Vertex Buffer Layout
impl RawInstance {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout{
            array_stride: mem::size_of::<RawInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {                         // Model Matrix
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },  
                wgpu::VertexAttribute {                         // Model Matrix
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {                         // Model Matrix
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {                         // Model Matrix
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute{                          // Velocity       
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32
                },
                wgpu::VertexAttribute{                          // White Key
                    offset: mem::size_of::<[f32; 17]>()as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32
                },
                wgpu::VertexAttribute {                         // Color Factor
                    offset: mem::size_of::<[f32; 18]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Uint32
                }
            ]
        }
    }
}

#[derive(Debug)]
pub struct Objects {
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}

