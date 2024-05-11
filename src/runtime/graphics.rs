mod gui;
mod texture;
mod camera;
mod mesh;
mod instances;
mod spheres;
mod engine;

pub use self::texture::*;
pub use self::camera::*;
pub use self::mesh::*;
pub use self::instances::*;
pub use self::spheres::*;
pub use self::engine::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphics() {
        let mut gfx = Engine::new(WindowBuilder::new().build(&EventLoop::new()).unwrap(), 1, &TexturedSquare::new());
        assert_eq!(gfx.size, 1);
    }
}