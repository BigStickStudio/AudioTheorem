//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

pub enum Sphere {
    White,
    Black,
    Blue1,
    Blue2,
    Blue3,
    Blue4,
    Blue5,
    Blue6,
    Blue7,
    Blue8,
    Green,
    Orange,
    Red,
}

impl Sphere {
    pub fn from_index(&self) -> &str {
        1 => Sphere::Blue1,
        2 => Sphere::Blue2,
        3 => Sphere::Blue3,
        4 => Sphere::Blue4,
        5 => Sphere::Blue5,
        6 => Sphere::Blue6,
        7 => Sphere::Blue7,
        8 => Sphere::Blue8,
    }

    pub fn to_string(&self) -> &str {
        match self {
            Sphere::Black => "black_sphere",
            Sphere::White => "white_sphere",
            Sphere::Red => "red_sphere",
            Sphere::Orange => "orange_sphere",
            Sphere::Green => "black_sphere",
            Sphere::Blue1 => "blue1_sphere",
            Sphere::Blue2 => "blue2_sphere",
            Sphere::Blue3 => "blue3_sphere",
            Sphere::Blue4 => "blue4_sphere",
            Sphere::Blue5 => "blue5_sphere",
            Sphere::Blue6 => "blue6_sphere",
            Sphere::Blue7 => "blue7_sphere",
            Sphere::Blue8 => "blue8_sphere",
        }        
    }

    pub fn diffuse_bytes(&self) -> &[u8] {
        match self {
            Sphere::White => include_bytes!("textures/white_sphere.png"),
            Sphere::Black => include_bytes!("textures/black_sphere.png"),
            Sphere::Blue1 => include_bytes!("textures/blue1_sphere.png"),
            Sphere::Blue2 => include_bytes!("textures/blue2_sphere.png"),
            Sphere::Blue3 => include_bytes!("textures/blue3_sphere.png"),
            Sphere::Blue4 => include_bytes!("textures/blue4_sphere.png"),
            Sphere::Blue5 => include_bytes!("textures/blue5_sphere.png"),
            Sphere::Blue6 => include_bytes!("textures/blue6_sphere.png"),
            Sphere::Blue7 => include_bytes!("textures/blue7_sphere.png"),
            Sphere::Blue8 => include_bytes!("textures/blue8_sphere.png"),
            Sphere::Green => include_bytes!("textures/black_sphere.png"),
            Sphere::Orange => include_bytes!("textures/orange_sphere.png"),
            Sphere::Red => include_bytes!("textures/red_sphere.png"),
        }
    }
}