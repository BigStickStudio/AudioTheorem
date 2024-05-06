//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use winit::event::*;

const OPENGL_2_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Debug)]
pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fov_y), self.aspect, self.z_near, self.z_far);

        return OPENGL_2_WGPU_MATRIX * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj:[[f32; 4]; 4]
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self { view_proj: cgmath::Matrix4::identity().into(), }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_proj = camera.view_projection_matrix().into();
    }
}

#[derive(Debug)]
pub struct CameraController {
    speed: f32,
    move_forward: bool,
    move_backward: bool,
    move_left: bool,
    move_right: bool,
    look_down: bool,
    look_up: bool,
    look_right: bool,
    look_left: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            move_forward: false,
            move_backward: false,
            move_left: false,
            move_right: false,
            look_up: false,
            look_down: false,
            look_right: false,
            look_left: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let pressed = *state == ElementState::Pressed;

                match keycode {
                    VirtualKeyCode::W => {
                        self.move_forward = pressed;
                        true
                    }
                    VirtualKeyCode::S => {
                        self.move_backward = pressed;
                        true
                    }
                    VirtualKeyCode::A => {
                        self.move_left = pressed;
                        true
                    }
                    VirtualKeyCode::D => {
                        self.move_right = pressed;
                        true
                    }
                    VirtualKeyCode::Up => {
                        self.look_up = pressed;
                        true
                    }
                    VirtualKeyCode::Down => {
                        self.look_down = pressed;
                        true
                    }
                    VirtualKeyCode::Left => {
                        self.look_left = pressed;
                        true
                    }
                    VirtualKeyCode::Right => {
                        self.look_right = pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;

        let forward = camera.target - camera.eye;
        let forward_normal = forward.normalize();
        let forward_magnitude = forward.magnitude();

        if self.move_forward && forward_magnitude > self.speed {
            camera.eye += forward_normal * self.speed;
        }
        
        if self.move_backward {
            camera.eye -= forward_normal * self.speed;
        }

        let right = forward_normal.cross(camera.up);
        let forward = camera.target - camera.eye;
        let forward_magnitude = forward.magnitude();

        if self.look_right {
            camera.target = camera.eye + (forward + right * self.speed).normalize() * forward_magnitude;
        }

        
        if self.look_left {
            camera.target = camera.eye + (forward - right * self.speed).normalize() * forward_magnitude;
        }
    }
}