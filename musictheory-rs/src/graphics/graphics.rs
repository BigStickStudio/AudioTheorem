//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use ggez::{event, Context, GameResult};
use ggez::graphics::{Canvas, Rect, Quad, Color, DrawParam};
use ggez::mint::Point2;
use crate::types::Dynamic;

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

    fn to_rect(&self) -> Rect {
        Rect::new_i32(
            self.x * SQUARE_SIZE + SQUARE_SIZE,
            self.y * SQUARE_SIZE + SQUARE_SIZE,
            SQUARE_SIZE - 1,
            SQUARE_SIZE - 1,
        )
    }

    fn to_point2(&self) -> Point2<f32> {
        Point2 {
            x: (self.x * SQUARE_SIZE + SQUARE_SIZE) as f32,
            y: (self.y * SQUARE_SIZE + SQUARE_SIZE) as f32,
        }
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


#[derive(Copy, Clone, Debug)]
pub struct Graphics {
    state: Grid
}

impl Graphics {
    pub fn init() -> Graphics {
        Graphics { state: Grid::new() }
    }

    pub fn render(&self) -> GameResult {
        println!("here we are again!");

        let (ctx, events_loop) = ggez::ContextBuilder::new("Audio Theorem", "NeoTec Digital")
                                .window_setup(ggez::conf::WindowSetup::default().title("Audio Theorem"))
                                .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
                                .build()?;

        event::run(ctx, events_loop, self.state)
    }
}

impl event::EventHandler<ggez::GameError> for Grid {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.1, 0.1, 1.0]));

        for sq in self.grid.iter() {
            canvas.draw(
                &Quad,
                DrawParam::default()
                        .scale([50., 70.])
                        .dest(sq.position.to_point2())
                        .color(Color::RED)
            );
        }

        canvas.finish(ctx)?;
        ggez::timer::yield_now();

        Ok(())
    }
}