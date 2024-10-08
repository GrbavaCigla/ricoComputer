use super::Word;

const ADDR_SIZE: usize = 2_usize.pow(16);


#[derive(Debug)]
pub struct VM {
    pub memory: [u16; ADDR_SIZE],
    pub processor: (u16, u16),
}

impl VM {
    pub fn new() -> Self {
        Self {
            memory: [0; ADDR_SIZE],
            processor: (0, 0),
        }
    }
}

impl From<Vec<u8>> for VM  {
    fn from(value: Vec<u8>) -> Self {
        let words: Vec<Word> = value.chunks(2).map(|b| Word::from([b[0], b[1]])).collect();

        Self { memory: [0; ADDR_SIZE], processor: (0, 0) }
    }
}