//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

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
