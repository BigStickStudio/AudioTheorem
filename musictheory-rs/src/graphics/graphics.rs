//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use bevy::prelude::*;
use bevy::ecs::component::Component;
use crate::types::Dynamic;


const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 800.0;
const SQUARE_SIZE: u8 = 16;


// Entity       - things that contain groups of components 
// Components   - data
// Systems      - used to process Components

#[derive(Copy, Clone, Debug)]
struct Color { r: u8, g: u8, b: u8 }

#[derive(Copy, Clone, Debug)]
struct Position { x: f32, y: f32, z: f32 }

#[derive(Copy, Clone, Debug, Component)]
pub struct Square{
    index: u8,
    intensity: Dynamic,
    color: Color,
    position: Position
}

#[derive(Component)]
struct POV;

fn setup(mut commands: Commands) {
    // Instantiate Camera
    commands.spawn((
        Camera3dBundle::default(),
        POV,
    ));

    // Create Grid of Squares
    for i in 0..144 {
        commands.spawn(Square {
            index: i,
            intensity: Dynamic::Forte,
            color: Color { 
                r: 72, 
                g: 72, 
                b: 72 
            },
            position: Position { 
                x: ((i % 12 + 1) * SQUARE_SIZE + SQUARE_SIZE) as f32, 
                y: ((i / 12 + 1) * SQUARE_SIZE + SQUARE_SIZE) as f32, 
                z: 0.0 
            }
        });
    }
}

fn update_grid(query: Query<&Square>){
    for square in &query {
        println!("square {} \t position {:?} \t intensity {:?}", square.index, square.position, square.intensity);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(update_grid);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Graphics;

impl Graphics {
    pub fn render() {
        App::new().add_plugins(DefaultPlugins)
                  .add_plugin(MidiPlugin)
                  .run()
    }
}