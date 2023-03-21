//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//


use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Index(u16);

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ColoredVertex {
    pub pos: [f32; 4],
    pub col: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TexturedVertex {
    pub pos: [f32; 4],
    pub tex_coords: [f32; 2],
}

impl ColoredVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ColoredVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4
                },
            ]
        }
    }
}

impl TexturedVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TexturedVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2
                },
            ]
        }
    }
}


const INDICES: &[Index] = &[
    Index(0), Index(1), Index(3),
    Index(3), Index(2), Index(0),
];

const COLORED_VERTICES: &[ColoredVertex] = &[
    ColoredVertex { pos: [-0.5, 0.5, 0.0, 1.0], col: [0.8, 0.0, 0.0, 1.0] },
    ColoredVertex { pos: [0.5, 0.5, 0.0, 1.0], col: [0.1, 0.0, 1.0, 1.0] },
    ColoredVertex { pos: [-0.5, -0.5, 0.0, 1.0], col: [0.1, 1.0, 0.0, 1.0] },
    ColoredVertex { pos: [0.5, -0.5, 0.0, 1.0], col: [0.0, 0.4, 0.4, 1.0] },
];

pub struct ColoredSquare<'a> {
    pub vertices: &'a [ColoredVertex],
    pub indices: &'a [Index]
}

impl <'a> ColoredSquare<'a> {
    pub fn new() -> Self {
        Self {
            vertices: COLORED_VERTICES,
            indices: INDICES
        }
    }
}

const TEXTURED_VERTICES: &[TexturedVertex] = &[
    TexturedVertex { pos: [-0.5, 0.5, 0.0, 1.0], tex_coords: [0.0, 0.0] },
    TexturedVertex { pos: [0.5, 0.5, 0.0, 1.0], tex_coords: [1.0, 0.0] },
    TexturedVertex { pos: [-0.5, -0.5, 0.0, 1.0], tex_coords: [0.0, 1.0] },
    TexturedVertex { pos: [0.5, -0.5, 0.0, 1.0], tex_coords: [1.0, 1.0] },
];

pub struct TexturedSquare<'a> {
    pub vertices: &'a [TexturedVertex],
    pub indices: &'a [Index],
}

impl <'a> TexturedSquare<'a> {
    pub fn new() -> Self {
        Self {
            vertices: TEXTURED_VERTICES,
            indices: INDICES
        }
    }
}