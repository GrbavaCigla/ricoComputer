use instructions::run_instruction;
use miette::{IntoDiagnostic, Result};
use std::{fs::read, path::Path, u16};

use rc_common::Word;

pub mod instructions;
mod utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VM {
    memory: [u16; 2_usize.pow(16)],
    stack_pointer: u16,
    program_counter: u16,
    state: VMState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMState {
    UNLOADED,
    RUNNING,
    STOPPED,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            // Program params
            memory: [0; 2_usize.pow(16)],
            program_counter: 8,
            stack_pointer: u16::MAX,

            // VM Params
            state: VMState::UNLOADED,
        }
    }
}

impl VM {
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.reset();

        let words: Vec<u16> = read(path)
            .into_diagnostic()?
            .chunks(2)
            .map(|x| Word(x[0], x[1]).into())
            .collect();

        self.memory[words[0] as usize..words.len() + words[0] as usize - 1]
            .clone_from_slice(&words[1..]);

        self.program_counter = words[0];
        self.state = VMState::STOPPED;

        Ok(())
    }

    // TODO: Introduce breakpoints
    pub fn start(&mut self) {
        self.state = VMState::RUNNING;

        while self.state == VMState::RUNNING {
            self.next_instr();
        }
    }

    pub fn next_instr(&mut self) {
        if self.program_counter > u16::MAX {
            self.state = VMState::STOPPED;
            return;
        }

        let next_instr = Word::from(self.memory[self.program_counter as usize]);
        
        let instr_state = run_instruction(&self.memory, next_instr, None);

        self.state = instr_state.0;
        self.program_counter += instr_state.1;
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
        self.program_counter = 8;
        self.stack_pointer = u16::MAX;
        self.state = VMState::UNLOADED;
    }
}
