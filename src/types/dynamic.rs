//
// Copyright 2023-2024 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Dynamic {
    Off,                    // 0
    Pianissimissimo,        // 16
    Pianissimo,             // 33
    Piano,                  // 49
    MezzoPiano,             // 64
    MezzoForte,             // 80
    Forte,                  // 96
    Fortissimo,             // 122
    Fortissimissimo         // 127
}

impl Dynamic {
    pub fn to_index(&self) -> u8 {
        match *self {
            Dynamic::Off => 0,
            Dynamic::Pianissimissimo => 1,
            Dynamic::Pianissimo => 2,
            Dynamic::Piano => 3,
            Dynamic::MezzoPiano => 4,
            Dynamic::MezzoForte => 5,
            Dynamic::Forte => 6,
            Dynamic::Fortissimo => 7,
            Dynamic::Fortissimissimo => 8
        }
    }

    pub fn to_velocity(&self) -> u8 {
        match *self {
            Dynamic::Off => 0,
            Dynamic::Pianissimissimo => 16,
            Dynamic::Pianissimo => 33,
            Dynamic::Piano => 49,
            Dynamic::MezzoPiano => 64,
            Dynamic::MezzoForte => 80,
            Dynamic::Forte => 96,
            Dynamic::Fortissimo => 122,
            Dynamic::Fortissimissimo => 127
        }
    }

    pub fn from_velocity(velocity: u8) -> Dynamic {
        match velocity {
            0 => Dynamic::Off,
            1..=16 => Dynamic::Pianissimissimo,
            17..=33 => Dynamic::Pianissimo,
            34..=49 => Dynamic::Piano,
            50..=64 => Dynamic::MezzoPiano,
            65..=80 => Dynamic::MezzoForte,
            81..=96 => Dynamic::Forte,
            97..=122 => Dynamic::Fortissimo,
            123..=127 => Dynamic::Fortissimissimo,
            _ => panic!("Velocity out of range"),
        }
    }
}

impl fmt::Display for Dynamic{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            match *self {
                Dynamic::Off => format_args!("None").fmt(f),
                Dynamic::Pianissimissimo => format_args!("Pianissimissimo").fmt(f),
                Dynamic::Pianissimo => format_args!("Pianissimo").fmt(f),
                Dynamic::Piano => format_args!("Piano").fmt(f),
                Dynamic::MezzoPiano => format_args!("Mezzo-Piano").fmt(f),
                Dynamic::MezzoForte => format_args!("Mezzo-Forte").fmt(f),
                Dynamic::Forte => format_args!("Forte").fmt(f),
                Dynamic::Fortissimo => format_args!("Fortissimo").fmt(f),
                Dynamic::Fortissimissimo => format_args!("Fortissimissimo").fmt(f)
            }
        } else {
            match *self {
                Dynamic::Off => format_args!("None").fmt(f),
                Dynamic::Pianissimissimo => format_args!("ppp").fmt(f),
                Dynamic::Pianissimo => format_args!("pp").fmt(f),
                Dynamic::Piano => format_args!("p").fmt(f),
                Dynamic::MezzoPiano => format_args!("mp").fmt(f),
                Dynamic::MezzoForte => format_args!("mf").fmt(f),
                Dynamic::Forte => format_args!("f").fmt(f),
                Dynamic::Fortissimo => format_args!("ff").fmt(f),
                Dynamic::Fortissimissimo => format_args!("fff").fmt(f)
            }
        }
    }
}
