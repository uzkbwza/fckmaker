mod tape;
pub use tape::*;
use std::marker::PhantomData;
use crate::{Value, Index};

/// Implement this when you need a model with an indexable collection such as a [`Vec`],
/// array, or anything else where you need to arbitrarily retrieve data from any point in
/// the structure. If you want a simple stack-based model with no need for indexing, just use a
/// [`Vec`]. In any case, you are free to use whatever structures you like for the memory layout.
/// These are just some handy implementations of common esolang structures.
pub trait TapeModel<V: Value, I: Index> {
    fn get_cell(&self) -> V;
    fn find_cell(&self, index: I) -> V;
    fn set_cell(&mut self, value: V);
    fn find_and_set_cell(&mut self, index: I, value: V);
    fn set_pointer(&mut self, index: I);
    fn get_pointer(&self) -> I;
}

pub type ClassicTapeModel<V = u8> = GenericTapeModel<V, usize, StandardTape<V>>;
pub type BidirectionalTapeModel<V = u8> = GenericTapeModel<V, isize, BidirectionalTape<V>>;

/// This model sets up easy-to-use, right-unbounded, one-dimensional tape with a  [`usize`] pointer
/// and should work fine for any bf-like that doesn't use an unconventional memory layout.
/// Decrementing the pointer behind the starting cell will panic.
pub fn classic_tape<V: Value>() -> ClassicTapeModel<V> {
    ClassicTapeModel::new(StandardTape::new())
}


/// This model is similar to [`ClassicTapeModel`], but with a tape that can expand in either direction. Note
/// that this uses [`isize`] instead of the normal [`usize`] to index the tape, so the distance your
/// pointer can travel in either direction is effectively cut in half. This shouldn't matter for
/// most languages on any 64-bit platforms as the maximum index is still very, very large.
pub fn bidirectional_tape<V: Value>() -> BidirectionalTapeModel<V> {
    BidirectionalTapeModel::new(BidirectionalTape::new())
}

pub struct GenericTapeModel<V: Value, I: Index, T: Tape<V, I>> {
    pub tape: T,
    pointer: I,
    phantom: PhantomData<V>
}

impl<V: Value, I: Index, T: Tape<V, I>> GenericTapeModel<V, I, T> {
    pub fn new(tape: T) -> Self {
        Self { tape, pointer: I::default(), phantom: PhantomData }
    }
}

impl<V: Value, I: Index, T: Tape<V, I>> TapeModel<V, I> for GenericTapeModel<V, I, T> {
    fn get_cell(&self) -> V {
        // get cell from current pointer location
        self.tape.get(self.pointer)
    }

    fn find_cell(&self, index: I) -> V {
        // get cell at arbitrary location
        self.tape.get(index)
    }

    fn set_cell(&mut self, value: V) {
        // set cell at pointer
        self.tape.set(self.pointer, value);
    }

    fn find_and_set_cell(&mut self, index: I, value: V) {
        // set cell at arbitrary location
        self.tape.set(index, value);
    }

    fn set_pointer(&mut self, index: I) {
        self.pointer = index;
    }

    fn get_pointer(&self) -> I {
        self.pointer
    }
}

