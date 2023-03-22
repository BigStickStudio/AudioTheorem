//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use cgmath::prelude::*;

#[derive(Debug)]
pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub index: u32
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawInstance {
    pub model: [[f32; 4]; 4],
    pub white_key: f32,
}


impl Instance {
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

    pub fn raw(&self) -> RawInstance {
        RawInstance {
            model: (cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation)).into(),
            white_key: self.index_to_white_key(),
        }
    }
}

impl RawInstance {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout{
            array_stride: mem::size_of::<RawInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute{
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32
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

