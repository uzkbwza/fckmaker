use std::fs;
use std::io::{stdout, Write};
use text_io::read;
use fckmaker::{Interpreter, tape_model::*};


#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Incr(u8),
    Decr(u8),
    Next(usize),
    Prev(usize),
    Read,
    Print,
    BeginLoop(usize),
    EndLoop(usize),
    EOF,
}

#[derive(Debug)]
enum BracketLocation {
    Open(usize),
    Closed(usize),
}

pub struct BfOptimized {
    program: Vec<Instruction>,
    inst_ptr: usize,
    model: ClassicTapeModel,
}

impl BfOptimized {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
            inst_ptr: 0,
            model: classic_tape(),
        }
    }

    fn input() -> u8 {
        print!("\nInput a character: ");
        stdout().flush().unwrap();
        let c: char = read!("{}");
        c as u8
    }
}

impl Interpreter<Instruction> for BfOptimized {
    type Value = u8;
    type Index = usize;
    type InstructionPointer = usize;
    type Program = Vec<Instruction>;

    fn load_program(&mut self, program: String) {
        use BracketLocation::*;
        use Instruction::*;
        let chars: Vec<char> = program
            .chars()
            .filter(|x| matches!(x, '>' | '<' | '+' | '-' | '.' | ',' | ']' | '['))
            .collect();
        if chars.is_empty() {
            return;
        }
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut cursor = 0;
        let mut counter;
        let mut brackets = Vec::new();
        while cursor < chars.len() {
            counter = 0;
            let ch = chars[cursor];
            let max = match ch {
                '<' | '>' => usize::MAX,
                '+' | '-' => 255,
                _ => 1,
            };

            while cursor < chars.len() && chars[cursor] == ch && counter < max {
                counter += 1;
                cursor += 1;
            }

            let instruction = match ch {
                '+' => Incr(counter as u8),
                '-' => Decr(counter as u8),
                '>' => Next(counter),
                '<' => Prev(counter),
                ',' => Read,
                '.' => Print,
                '[' => {
                    brackets.push(Open(instructions.len()));
                    BeginLoop(0)
                }
                ']' => {
                    brackets.push(Closed(instructions.len()));
                    EndLoop(0)
                }
                _ => unreachable!("Invalid bf command"),
            };

            instructions.push(instruction);
        }

        for (i, bracket) in brackets.iter().enumerate() {
            if let Open(index) = bracket {
                let start_index = *index;
                let mut end_index = 0;
                let mut depth = 1;
                let mut counter = 0;
                while depth > 0 && i + counter < brackets.len() {
                    counter += 1;
                    if i + counter >= brackets.len() {
                        break;
                    }
                    match brackets[i + counter] {
                        Open(_) => depth += 1,
                        Closed(index) => {
                            depth -= 1;
                            end_index = index
                        }
                    }
                }
                instructions[start_index] = BeginLoop(end_index);
                instructions[end_index] = EndLoop(start_index);
            }
        }
        instructions.push(EOF);
        self.program = instructions;
        self.inst_ptr = 0;
    }

    fn get_program(&self) -> &Vec<Instruction> {
        &self.program
    }

    fn process_instruction(&mut self, instruction: Instruction) -> bool {
        use Instruction::*;
        let model = &mut self.model;
        let pointer_index = model.get_pointer();
        let cell_value = model.get_cell();

        match instruction {
            Next(amount) => model.set_pointer(pointer_index.wrapping_add(amount)),
            Prev(amount) => model.set_pointer(pointer_index.wrapping_sub(amount)),
            Incr(amount) => model.set_cell(cell_value.wrapping_add(amount)),
            Decr(amount) => model.set_cell(cell_value.wrapping_sub(amount)),
            Read => model.set_cell(Self::input()),
            Print => print!("{}", cell_value as char),
            BeginLoop(index) => {
                if cell_value == 0 {
                    self.set_instr_ptr(index)
                }
            }
            EndLoop(index) => {
                if cell_value != 0 {
                    self.set_instr_ptr(index)
                }
            }
            EOF => return true,
        };
        return false;
    }
    fn set_instr_ptr(&mut self, index: usize) {
        self.inst_ptr = index;
    }
    fn get_instr_ptr(&self) -> usize {
        self.inst_ptr
    }
}

fn main() -> std::io::Result<()> {
    let program: String = fs::read_to_string("examples/mandelbrot.bf")?;
    let mut bf = BfOptimized::new();
    bf.load_program(program);
    bf.run_program(true);
    Ok(())
}
