//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::Dynamic;
use super::mesh::{TexturedSquare, ColoredSquare};

const SCREEN_WIDTH: f32 = 1600.0;
const SCREEN_HEIGHT: f32 = 1200.0;
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
pub struct Key {
    index: usize,
    intensity: Dynamic,
    color: Temperament,
    position: Position
}

#[derive(Copy, Clone, Debug)]
struct Grid {
    grid: [Key; 144]
}

impl Grid {
    pub fn new() -> Grid {
        let mut grid = [Key { 
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
