//
// Copyright 2023-2024 Richard I. Christopher, NeoTec Digital for Big Stick Studios under Ancillary. All Rights Reserved. Nexus Project.
//

use super::super::Sequence;
use super::super::Disposition;
use super::spheres;
use super::texture::Texture;
use super::camera::{Camera, CameraUniform, CameraController};
use super::mesh::*;
use super::instances::{Instance, RawInstance};
use super::spheres::Sphere;
use crate::runtime::{disposition, SequenceData};
use crate::types::Dynamic;
use std::sync::{Arc, Mutex};
use wgpu::util::DeviceExt;
use winit::event::WindowEvent;
use winit::window::Window;
use winit::dpi::PhysicalSize;
use cgmath::prelude::*;


#[derive(Debug)]
pub struct Engine {
    device: wgpu::Device,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,
    pub window: Window,
    pipeline: wgpu::RenderPipeline,
    queue: wgpu::Queue,
    pub size: PhysicalSize<u32>,
    camera: Camera,
    camera_controller: CameraController,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_textures: Vec<Texture>,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_vertices: u32,
    num_indices: u32,
}

impl Engine {
    // Todo: Break this into init functions
    pub async fn new(window: Window, grid_size: u32, square: &TexturedSquare<'_>) -> Self {
        env_logger::init();

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.enumerate_adapters(wgpu::Backends::all())
                              .filter(|adapter| {
                                  adapter.is_surface_supported(&surface)
                              })
                              .next().unwrap();
        
        let (device, queue) = adapter.request_device(
                                                &wgpu::DeviceDescriptor {
                                                    features: wgpu::Features::empty(),
                                                    limits: wgpu::Limits::default(),
                                                    label: None
                                                }, None)
                                            .await.unwrap();

        let capabilities = surface.get_capabilities(&adapter);

        let format = capabilities.formats.iter().copied()
                                         .filter(|f| f.describe().srgb)
                                         .next()
                                         .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration{
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format,
                        width: size.width,
                        height: size.height,
                        present_mode: capabilities.present_modes[0],
                        alpha_mode: capabilities.alpha_modes[0],
                        view_formats: vec![]
                    };
        
        surface.configure(&device, &config);

        let instance_displacement: cgmath::Vector3<f32> = cgmath::Vector3::new(
            grid_size as f32 * 0.25,
            0.0,
            grid_size as f32 * 0.25,
        );

        // We are creating a grid of instances
        let instances = (0..grid_size).flat_map(|y| {
            (0..grid_size).map(move |x| {
                let index = x;
                let position = cgmath::Vector3 { x: x as f32, y: y as f32, z: 0.0 } - instance_displacement;
                let rotation = cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_y(), cgmath::Deg(0.0));
                Instance { position, rotation, index, dynamic: Dynamic::Off, disposition: Disposition::Natural }
            })
        }).collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::raw).collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(square.vertices),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(square.indices),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        let num_vertices = square.vertices.len() as u32;
        let num_indices = square.indices.len() as u32;

        let spheres: Vec<Sphere> = vec![Sphere::White, Sphere::Black, Sphere::Blue8, Sphere::Green, Sphere::Orange, Sphere::Red];
        let diffuse_textures: Vec<Texture> = spheres
                                                .iter()
                                                .map(|sphere| 
                                                    { Texture::from_bytes(&device, &queue, sphere.diffuse_bytes(), sphere.to_string())})
                                                .collect();

        // We are creating a layout for the textures that we are going to use in the shader
        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler (wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler (wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 4, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 5, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler (wgpu::SamplerBindingType::Filtering),
                        count: None,
                    }, 
                    wgpu::BindGroupLayoutEntry {
                        binding: 6, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 7, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler (wgpu::SamplerBindingType::Filtering),
                        count: None,
                    }, 
                    wgpu::BindGroupLayoutEntry {
                        binding: 8, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 9, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler (wgpu::SamplerBindingType::Filtering),
                        count: None,
                    }, 
                    wgpu::BindGroupLayoutEntry {
                        binding: 10, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 11, visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler (wgpu::SamplerBindingType::Filtering),
                        count: None,
                    }
                ],
                label: Some("texture_bind_group_layout"),
            }
        );

        // and here we are binding the textures to the layout
        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry 
                        { binding: 0, resource: wgpu::BindingResource::TextureView(&diffuse_textures[0].view), },
                    wgpu::BindGroupEntry 
                        { binding: 1, resource: wgpu::BindingResource::Sampler(&diffuse_textures[0].sampler), },
                    wgpu::BindGroupEntry 
                        { binding: 2, resource: wgpu::BindingResource::TextureView(&diffuse_textures[1].view), },
                    wgpu::BindGroupEntry 
                        { binding: 3, resource: wgpu::BindingResource::Sampler(&diffuse_textures[1].sampler), },
                    wgpu::BindGroupEntry 
                        { binding: 4, resource: wgpu::BindingResource::TextureView(&diffuse_textures[2].view), },
                    wgpu::BindGroupEntry 
                        { binding: 5, resource: wgpu::BindingResource::Sampler(&diffuse_textures[2].sampler), },
                    wgpu::BindGroupEntry 
                        { binding: 6, resource: wgpu::BindingResource::TextureView(&diffuse_textures[3].view), },
                    wgpu::BindGroupEntry 
                        { binding: 7, resource: wgpu::BindingResource::Sampler(&diffuse_textures[3].sampler), },
                    wgpu::BindGroupEntry 
                        { binding: 8, resource: wgpu::BindingResource::TextureView(&diffuse_textures[4].view), },
                    wgpu::BindGroupEntry 
                        { binding: 9, resource: wgpu::BindingResource::Sampler(&diffuse_textures[4].sampler), },
                    wgpu::BindGroupEntry 
                        { binding: 10, resource: wgpu::BindingResource::TextureView(&diffuse_textures[5].view), },
                    wgpu::BindGroupEntry 
                        { binding: 11, resource: wgpu::BindingResource::Sampler(&diffuse_textures[5].sampler), }
                ],
                label: Some("diffuse_bind_group")
            }
        );

        // Fixed Camera - TODO: allow for camera movement in specific scenarios -> this becomes part of the graphics engine for Nexus
        let camera = Camera {
            eye: (2.5, 7.0, 12.0).into(),
            target: (2.5, 6.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        };

        let camera_controller = CameraController::new(0.1);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_projection(&camera);

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some("camera_bind_group_layout"),
            }
        );

        let camera_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }
                ],
                label: Some("camera_bind_group"),
            }
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("texturedshader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &texture_bind_group_layout,
                &camera_bind_group_layout
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    TexturedVertex::desc(),
                    RawInstance::desc()
                    ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    //blend: Some(wgpu::BlendState::REPLACE),
                    blend: Some(wgpu::BlendState{
                        color: wgpu::BlendComponent{
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent::OVER
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            device,
            config,
            surface,
            window,
            pipeline,
            queue,
            size,
            camera, 
            camera_controller,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            diffuse_bind_group,
            diffuse_textures,
            instances,
            instance_buffer,
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event)
    }

    pub fn enable_tones(&mut self, iv: SequenceData) {
        // reset all the instances dynamic to off
        self.instances.iter_mut().for_each(|instance| {
            instance.dynamic = Dynamic::Off;
            instance.disposition = Disposition::Natural;
        });


        for i in iv.iv.iter()
            { self.instances[*i.index as usize].trigger_key(*i.velocity, iv.disposition); }
        
        let instance_data = self.instances.iter().map(Instance::raw).collect::<Vec<_>>();

        self.instance_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_projection(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder")
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.03,
                            g: 0.06,
                            b: 0.09,
                            a: 1.0,
                        }),
                        store: true,
                    }
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}