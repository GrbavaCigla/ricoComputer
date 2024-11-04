#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Word(pub u8, pub u8);

impl From<u16> for Word {
    fn from(value: u16) -> Self {
        Word {
            0: 0b1111_1111 & value as u8,
            1: 0b1111_1111 & (value >> 8) as u8,
        }
    }
}

impl From<[u8; 4]> for Word {
    fn from(mut value: [u8; 4]) -> Self {
        for part in value.iter_mut() {
            *part &= 0b1111;
        }

        Self {
            0: (value[2] << 4) | value[3],
            1: (value[0] << 4) | value[1],
        }
    }
}
