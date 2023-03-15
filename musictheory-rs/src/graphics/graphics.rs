//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use bevy::app::App;
use bevy::ecs::component::Component;

// Entity       - things that contain groups of components 
// Components   - data
// Systems      - used to process Components

#[derive(Component)]
struct Color { r: u8, g: u8, b: u8 }

#[derive(Component)]
struct Position { x: f32, y: f32, z: f32 }

#[derive(Component)]
struct Intensity(u8);

#[derive(Component)]
struct Index(u8);

struct Square(u64);


pub struct Display{
    app: App
}

fn test() { println!("Hello Bevy!") }

impl Display {
    pub fn create() -> Display {
        Display {
            app: App::new()
        }
    }

    pub fn init(&mut self) {
        self.app.add_system(test);

    }

    pub fn run(&mut self) {
        self.app.run();
    }
}