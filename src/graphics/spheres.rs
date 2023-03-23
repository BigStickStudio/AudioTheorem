//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::Dynamic;

pub enum Sphere {
    White,
    Black,
}

impl Sphere {
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