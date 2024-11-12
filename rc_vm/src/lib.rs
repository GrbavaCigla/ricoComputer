use miette::{IntoDiagnostic, Result};
use rc_common::Word;
use std::{fs::read, path::Path, u16};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VM {
    memory: [u16; 2_usize.pow(16)],
    stack_pointer: u16,
    program_counter: u16,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            memory: [0; 2_usize.pow(16)],
            program_counter: 8,
            stack_pointer: u16::MAX,
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

        println!("{:?}", self.memory);

        Ok(())
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
        self.program_counter = 8;
        self.stack_pointer = u16::MAX;
    }
}
