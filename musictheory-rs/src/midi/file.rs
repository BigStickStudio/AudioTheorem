//
// Copyright 2019 Hans W. Uhlig, Richard I. Christopher. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Seek};
use std::path::Path;
use std::convert::TryFrom;
use super::{MidiError, MidiSong, MidiTrack};
use term::terminfo::Error::IoError;

pub enum MidiFile {
    SingleTrack {},
    MultiTrack {},
    MultiSong {},
}

impl MidiFile {
    pub fn open<T: Read + Seek>(reader: &mut T) -> Result<MidiFile> {
        // Read Header
        if reader.read_u32::<BigEndian>()? != 0x4D_54_68_64 /* MThd as HEX(u32) */ {
            return Err(std::io::Error::new(ErrorKind::InvalidData, &"Header Magic Number Mismatch"));
        }
        if reader.read_u32::<BigEndian>()? != 6 {
            return Err(std::io::Error::new(ErrorKind::InvalidData, &"Header size declaration incorrect"));
        }
        let file_format = reader.read_u16::<BigEndian>()?;
        let track_count = reader.read_u16::<BigEndian>()? as usize;
        let time_division = reader.read_i16::<BigEndian>()? as isize; // Units per Beat if positive, SMPTE Units if Negative
        let bpm = 120.0;

        // Read Tracks
        for track in 0..track_count {
            if reader.read_u32::<BigEndian>()? != 0x4D_54_72_6B /* MTrk as HEX(u32) */ {
                return Err(std::io::Error::new(ErrorKind::InvalidData, &"Track Magic Number Mismatch"));
            }
            let chunk_size = reader.read_u32::<BigEndian>()?;
            let mut time = 0;
            let mut delta_time = 0;
            let mut running_status = None;
            let mut running_channel = None;
            let mut is_running= false;
            let mut end_of_track = false;
            while !end_of_track {
                let delta_time = {
                    // Read Variable Number
                    // http://en.wikipedia.org/wiki/Variable-length_quantity
                    // cont. bit---V
                    //             7[6 5 4 3 2 1 0]+-+
                    // more bytes: 1 b b b b b b b   | concat bits to form new number
                    //                               V
                    //                             7[6 5 4 3 2 1 0]
                    //              no more bytes: 0 b b b b b b b

                    let mut octet = reader.read_u8()?;
                    let mut value = (octet & 0b0111_1111) as usize;
                    while octet >= 0b1000_0000 {
                        octet = reader.read_u8()?;
                        value = (value << 7) as usize + (octet & 0b0111_1111) as usize;
                    }
                    value
                };
            }
        }
        Err(IoError::new(ErrorKind::Interrupted, &"Not yet Implemented"))
    }
}
