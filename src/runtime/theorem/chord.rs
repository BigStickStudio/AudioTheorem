
// Chord is a struct that represents a chord e.g. 'best attempt' to find a 'root' in music theory. It is a collection of notes that are played simultaneously.
use crate::types::{Note, Interval};


#[derive(Clone, Debug)]
pub struct Chord { 
    // This isn't much of a chord, but it's an interface for a "scale" 
    // to act as a cursor for all possible scales and N number of potential chords based on inversions.
    // We can use this to reduce interval sets and determine how we want to filter the scales (proprietary - all rights reserved - Ancillary, 2024)
    root: Note,
    intervals: Vec<(Note, Interval)> // We could just refactor this to just be intervals or even just have a function that returns the intervals (which we already have elsewhere)
    // we need to add a vector where we add a gradient based the maximum derivation and the minimum derivation 
        // ++ while !> /2
    // and bounds of 14ths, and splitting sequences based on scale and interval limits
        // .. we could plug machine learning here in the future to determine the best chord for a given set of notes (proprietary Nexus.)
}