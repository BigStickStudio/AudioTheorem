//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

pub enum Sphere {
    Black,
    White,
    Red,
    Orange,
    Green,
    Blue1,
    Blue2,
    Blue3,
    Blue4,
    Blue5,
    Blue6,
    Blue7,
    Blue8,
}

impl Sphere {
    pub fn from_index(index: u8) -> Sphere {
        match index {
            0 => Sphere::White,
            1 => Sphere::Black,
            2 => Sphere::White,
            3 => Sphere::Black,
            4 => Sphere::White,
            5 => Sphere::White,
            6 => Sphere::Black,
            7 => Sphere::White,
            8 => Sphere::Black,
            9 => Sphere::White,
            10 => Sphere::Black,
            11 => Sphere::White,
            _ => Sphere::White
        }
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
            Sphere::Black => include_bytes!("black_sphere.png"),
            Sphere::White => include_bytes!("white_sphere.png"),
            Sphere::Red => include_bytes!("red_sphere.png"),
            Sphere::Orange => include_bytes!("orange_sphere.png"),
            Sphere::Green => include_bytes!("black_sphere.png"),
            Sphere::Blue1 => include_bytes!("blue1_sphere.png"),
            Sphere::Blue2 => include_bytes!("blue2_sphere.png"),
            Sphere::Blue3 => include_bytes!("blue3_sphere.png"),
            Sphere::Blue4 => include_bytes!("blue4_sphere.png"),
            Sphere::Blue5 => include_bytes!("blue5_sphere.png"),
            Sphere::Blue6 => include_bytes!("blue6_sphere.png"),
            Sphere::Blue7 => include_bytes!("blue7_sphere.png"),
            Sphere::Blue8 => include_bytes!("blue8_sphere.png"),
        }
    }
}