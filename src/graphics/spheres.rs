//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::Dynamic;

pub enum Sphere {
    White,
    Black,
}

impl Sphere {
    pub fn dynamic_color(&self, dynamic: Dynamic) -> [f32; 3] {
        match dynamic {
            Dynamic::Off => [0.0, 0.0, 0.0],
            Dynamic::Pianissimissimo => [0.8, 0.96, 1.0],
            Dynamic::Pianissimo => [0.7, 0.941, 1.0],
            Dynamic::Piano => [0.6, 0.921, 1.0],
            Dynamic::MezzoPiano => [0.4, 0.878, 1.0],
            Dynamic::MezzoForte => [0.2, 0.839, 1.0],
            Dynamic::Forte => [0.0, 0.745, 0.98],
            Dynamic::Fortissimo => [0.0, 0.541, 0.879],
            Dynamic::Fortissimissimo => [0.0, 0.418, 0.77]
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Sphere::White => "white_sphere",
            Sphere::Black => "black_sphere"
        }        
    }

    pub fn diffuse_bytes(&self) -> &[u8] {
        match self {
            Sphere::White => include_bytes!("textures/white_sphere.png"),
            Sphere::Black => include_bytes!("textures/black_sphere.png"),
        }
    }
}