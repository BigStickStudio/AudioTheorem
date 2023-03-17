//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::Dynamic;
use winit::event::*;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 800.0;
const SQUARE_SIZE: i32 = 16;
const GRID_SIZE: u8 = 12;
const DESIRED_FPS: u32 = 1;

#[derive(Copy, Clone, Debug)]
struct Temperament { r: u8, g: u8, b: u8 }

impl Temperament {
    fn to_slice(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0, 
            self.g as f32 / 255.0, 
            self.b as f32 / 255.0, 
            1.0
        ]
    }
}

#[derive(Copy, Clone, Debug)]
struct Position { x: i32, y: i32 }

impl Position {
    pub fn from_coords(x: u8, y: u8) -> Position {
        Position { x: x as i32, y: y as i32 }
    }

    // X, Y, Width, Height
    fn to_slice(&self) -> [i32; 4] {
        [
            self.x * SQUARE_SIZE + SQUARE_SIZE,
            self.y * SQUARE_SIZE + SQUARE_SIZE,
            SQUARE_SIZE - 1,
            SQUARE_SIZE - 1,
        ]
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Square {
    index: usize,
    intensity: Dynamic,
    color: Temperament,
    position: Position
}

#[derive(Copy, Clone, Debug)]
struct Grid {
    grid: [Square; 144]
}

impl Grid {
    pub fn new() -> Grid {
        let mut grid = [Square { 
                            index: 0, 
                            intensity: Dynamic::Off, 
                            color: Temperament{ 
                                        r: 125, 
                                        g: 125, 
                                        b: 125 
                                    }, 
                            position: Position{ x: 0, y: 0 }
                        }; 144];

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let index = (row * GRID_SIZE + col) as usize;
                grid[index].index = index;
                grid[index].position.x = col as i32;
                grid[index].position.y = row as i32;
            }
        }

        Grid { grid }
    }

}


#[derive(Debug)]
pub struct Graphics {
    grid: Grid,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: Window,
    pipeline: wgpu::RenderPipeline,
}

impl Graphics {
    async fn new(window: Window) -> Self {
        env_logger::init();

        let grid = Grid::new();

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
                        format: format,
                        width: size.width,
                        height: size.height,
                        present_mode: capabilities.present_modes[0],
                        alpha_mode: capabilities.alpha_modes[0],
                        view_formats: vec![]
                    };
        
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
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
            grid,
            surface,
            device,
            queue,
            config,
            size,
            window,
            pipeline
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub async fn run() {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let mut gfx = Graphics::new(window).await;

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == gfx.window.id() => if !gfx.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        gfx.resize(*physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        gfx.resize(**new_inner_size);
                    },
                    _ => {}
                }
            },
            Event::RedrawRequested(window_id) if window_id == gfx.window.id() => {
                gfx.update();
                match gfx.render() {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost) => gfx.resize(gfx.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::MainEventsCleared => {
                gfx.window.request_redraw();
            },
            _ => {}
        });
    } 
}