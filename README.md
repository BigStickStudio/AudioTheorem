# Musical Theory Library

License is a Combination of Proprietary and Apache per the original source code below, as this project is a combination and evolution of 3 parts, or 2.5 projects..
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
<!---[![Build Status](https://travis-ci.org/huhlig/musictheory-rs.svg?branch=master)](https://travis-ci.org/huhlig/musictheory-rs) -->
<!---[![Coverage Status](https://coveralls.io/repos/github/huhlig/musictheory-rs/badge.svg?branch=master)](https://coveralls.io/github/huhlig/musictheory-rs?branch=master) -->

([Original API Docs])

## Objective

> Encoding music theory into a usable audio library
> Intended to be a learning tool and creative resource


### Ancillary TODO'S
 - [x] GUI Representation of 144 Grid
 - [x] Reading MIDI
 - [x] Synthesize Audio (Still In Progress)
 - [x] Parallelism in IO Loops
 - [ ] MIDI Playing in Concurrency with GUI and Analysis - Full Integration
 - [x] Pitchgroup Analysis with +/- Octave Representation in GUI
 - [ ] Scale Mode in GUI
 - [ ] Tonic Analysis and Chord / Inversions Chart
 - [ ] Combined Pitchgroup & Scale/Chord Analysis + Tonic Cursor
 - [ ] Abstracting Main Class and Decoupling AudioTheorem (NeoTec)
 - [ ] Develop GUI Framework (Big Stick)
 - [ ] Abstract out WGPU Engine (NeoTec)
 - [ ] Incorporate Live Analysis on Mobile
 - [ ] Incorporate Batch Analysis on Server
 - [ ] Incorporate File Drag and Drop
 - [ ] Smooth and Intuitive UI for GFX Analysis

### Working Samples
All Working Samples are Subject to change. If anyone wishes to go through the history you can find a history of the evolution of this project nested in the history of this readme. Results may vary. 

## Proto 
Proto has a python version of a graphical 'simulation' to help me better understand how to handle various sync and compute buffers for this problem set. Depending on the state of testing, this application can be ran using `python main.py`

![image](https://github.com/alephpt/AudioTheorem/assets/87874714/e557bba1-96d0-44fa-92f2-6ebff60e19df)
![image](https://github.com/alephpt/AudioTheorem/assets/87874714/e9b5b294-98c7-4584-90c0-1c3e7075e212)

## AudioTheorem
The AudioTheorem app can be ran from an executable, through `cargo run` and many of the examples and unit tests. The original documentation is filled with various examples as well.

The application takes midi input, and it is in the process of being able to write to an output. 
(This is on hold as I study more about handling buffers and queues)
![image](https://github.com/alephpt/AudioTheorem/assets/87874714/3b746d84-bb10-4d97-b645-005d41b7c1fa)
![image](https://github.com/alephpt/AudioTheorem/assets/87874714/2f3c81bb-82a8-4cb5-a110-c3a8962145f4)


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions. Additionally, without an explicitly stated agreement, any contributions submitted to this project, not inclusive of prior Apache-2.0 licensing, will be considered without any inclusive rights or entitlement to the lifetime of the work.

[Original API Docs]: https://huhlig.github.io/musictheory-rs/
[musictheory]: https://huhlig.github.io/musictheory-rs/

## License

The original "[musictheory]" part of this project, a collaborative effort between Hans Uhlig and Richard Christopher, is licensed under [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0).

The remaining original code of this project is held as proprietary context exclusively owned by Big Stick Studio, under private copyright, circa 2023 and throughout the pendency of this development project.
