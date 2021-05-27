use std::io::{stdout, Write};
use text_io::read;
use fckmaker::{Interpreter, Index, tape_model::*};


pub struct BfNaive {
    program: Vec<char>,
    inst_ptr: usize,
    model: ClassicTapeModel,
}

impl BfNaive {
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

impl Interpreter<char> for BfNaive {
    type Value = u8;
    type Index = usize;
    type InstructionPointer = usize;
    type Program = Vec<char>;

    fn load_program(&mut self, program: String) {
        self.program = program
            .chars()
            .filter(|c| matches!(c, '>' | '<' | '+' | '-' | '.' | ',' | ']' | '['))
            .collect();
        self.program.push(' '); // eof
    }

    fn get_program(&self) -> &Vec<char> {
        &self.program
    }

    fn process_instruction(&mut self, instruction: char) -> bool {
        let model = &mut self.model;
        let pointer_index = model.get_pointer();
        let cell_value = model.get_cell();

        match instruction {
            '>' => model.set_pointer(pointer_index.incr()),
            '<' => model.set_pointer(pointer_index.decr()),
            '+' => model.set_cell(cell_value.wrapping_add(1)),
            '-' => model.set_cell(cell_value.wrapping_sub(1)),
            '.' => print!("{}", model.get_cell() as char),
            ',' => model.set_cell(Self::input()),
            '[' => {
                if cell_value == 0 {
                    let mut brackets = 1;
                    while brackets != 0 {
                        self.incr_instr_ptr();
                        let cmd = self.get_current_instruction();
                        if cmd == '[' {
                            brackets += 1;
                        } else if cmd == ']' {
                            brackets -= 1;
                        }
                    }
                }
            }
            ']' => {
                if cell_value != 0 {
                    let mut brackets = 1;
                    while brackets != 0 {
                        self.decr_instr_ptr();
                        let cmd = self.get_current_instruction();
                        if cmd == '[' {
                            brackets -= 1;
                        } else if cmd == ']' {
                            brackets += 1;
                        }
                    }
                }
            }
            ' ' => return true,
            _ => unreachable!("Unrecognized input: {}", instruction),
        };
        false
    }
    fn set_instr_ptr(&mut self, index: usize) {
        self.inst_ptr = index;
    }
    fn get_instr_ptr(&self) -> usize {
        self.inst_ptr
    }
}

fn main() {
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string();
    let mut bf = BfNaive::new();
    bf.load_program(program);
    bf.run_program(true);
}