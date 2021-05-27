use crate::{Value, Index};

pub trait Tape<V: Value, I: Index = usize> {
    fn set(&mut self, index: I, value: V);
    fn get(&self, index: I) -> V;
}

/// A simple, one-dimensional cell-based tape that expands infinitely rightward.
/// See [`crate::tape_model::classic_tape`] for more info on usage.
pub struct StandardTape<V: Value> {
    cells: Vec<V>,
    default: V,
}

impl<V: Value> StandardTape<V> {
    pub fn new() -> Self {
        Self::with_custom_default(V::default())
    }

    pub fn with_custom_default(val: V) -> Self {
        let mut tape = Vec::with_capacity(30000);
        tape.push(val.clone());
        Self {
            cells: tape,
            default: val,
        }
    }

    fn expand_tape(&mut self, location: usize) {
        // dynamically grows tape to fit arbitrary set calls
        let len = self.cells.len();
        if len <= location {
            for _ in len..=location {
                self.cells.push(self.default.clone());
            }
        }
    }
}

impl<V: Value> Default for StandardTape<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Value> Tape<V> for StandardTape<V> {
    fn set(&mut self, location: usize, value: V) {
        self.expand_tape(location);
        self.cells[location] = value;
    }

    fn get(&self, location: usize) -> V {
        if self.cells.len() <= location {
            self.default.clone() // if we haven't got there yet, it's 0.
        } else {
            self.cells[location].clone()
        }
    }
}

/// Like [`StandardTape`], but expands into negative indices as well. See
/// [`crate::tape_model::bidirectional_tape`] for more info.
pub struct BidirectionalTape<V> {
    tape_forward: Vec<V>,
    tape_backward: Vec<V>,
    default: V,
}

impl<V: Value> BidirectionalTape<V> {
    pub fn new() -> Self {
        Self::with_custom_default(V::default())
    }

    pub fn with_custom_default(default: V) -> Self {
        Self {
            tape_forward: Vec::new(),
            tape_backward: vec![V::default()],
            default,
        }
    }

    fn choose_tape(&mut self, location: &mut isize) -> &mut Vec<V> {
        match *location {
            x if x < 0 => {
                *location = location.abs() - 1;
                &mut self.tape_backward
            },
            _ => &mut self.tape_forward,
        }
    }

    fn expand_tape(&mut self, mut location: isize) {
        // dynamically grows tape to fit arbitrary set calls
        let default = self.default.clone();
        let cells = self.choose_tape(&mut location);

        let len = cells.len() as isize;
        if len <= location {
            for _ in len..=location {
                cells.push(default.clone());
            }
        }
    }
}

impl<V: Value> Default for BidirectionalTape<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Value> Tape<V, isize> for BidirectionalTape<V> {
    fn set(&mut self, mut location: isize, value: V) {
        self.expand_tape(location);
        let cells = self.choose_tape(&mut location);
        cells[location as usize] = value;
    }

    fn get(&self, location: isize) -> V {
        let backward_index = (location.abs() as usize) - 1;
        if location >= 0 && (location as usize) < self.tape_forward.len() {
            self.tape_forward[location as usize].clone()
        } else if location < 0 && backward_index < self.tape_backward.len() {
            self.tape_backward[backward_index].clone()
        } else {
            V::default()
        }
    }
}