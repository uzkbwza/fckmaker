use crate::{Instruction, Value, Index};

/// This is the what will load and interpret your programs. Recommended use for tape-based languages
/// is implementing on a struct with a field that contains a [`TapeModel`] and a loaded program.
pub trait Interpreter<I: Instruction> {

    /// These are the values contained inside memory. In brainfuck, this would be
    /// [`u8`].
    type Value: Value;

    /// This is what you use to index your memory structure. In most cases, with a contiguous 1d
    /// structure such as an array or  [`Vec`], this would be a [`usize`]. If you are using a 2d
    /// tape implementation, it might be a `(usize, usize)`.
    type Index: Index;
    type InstructionPointer: Index;
    type Program: Program<I, Self::InstructionPointer>;

    /// Take an input object and parse it as a program. You'll probably want to
    /// store it in your struct so that you can implement [`Self::get_program`].
    fn load_program(&mut self, program: String);

    /// Retrieve the entire program
    fn get_program(&self) -> &Self::Program;

    /// Interprets a single instruction. Returning true will end the program. Make sure
    /// to update the instruction pointer within this function if you are using
    /// [`Self::run_program`] with `incr` set to `false`, because will not
    /// be incremented automatically and will probably enter an infinite loop.
    fn process_instruction(&mut self, instruction: I) -> bool;

    /// Set the instruction pointer (*not* the memory pointer) to the specified index.
    fn set_instr_ptr(&mut self, index: Self::InstructionPointer);

    /// Retrieve the instruction pointer's index (*not* the current instruction).
    fn get_instr_ptr(&self) -> Self::InstructionPointer;

    fn get_current_instruction(&self) -> I {
        self.get_program().get(self.get_instr_ptr())
    }
    fn incr_instr_ptr(&mut self) {
        self.set_instr_ptr(self.get_instr_ptr().incr())
    }
    fn decr_instr_ptr(&mut self) {
        self.set_instr_ptr(self.get_instr_ptr().decr())
    }

    /// Process instructions until EOF reached or program otherwise exits.
    /// If incr is set to `true`, the instruction pointer will increment
    /// after every instruction. This is usually desired in brainfuck-like
    /// languages, but some might want to manipulate the instruction pointer
    /// directly within [`Interpreter::process_instruction`], or in some other way.
    fn run_program(&mut self, incr: bool) {
        self.set_instr_ptr(Self::InstructionPointer::default());
        loop {
            if self.process_instruction(self.get_current_instruction()) {
                return;
            } else if incr {
                self.incr_instr_ptr();
            }
        }
    }
}

/// This represents your instruction tape or source code which will be interpreted on-the-fly.
/// While most people will probably just use something like `Vec<char>`, you are allowed
/// to use your own structures for source code, if need be.
pub trait Program<I: Instruction, N: Index> {
    fn get(&self, index: N) -> I;
}

impl<I: Instruction> Program<I, usize> for Vec<I> {
    fn get(&self, index: usize) -> I {
        self[index]
    }
}