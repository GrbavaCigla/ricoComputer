type Memsize = u16;

pub(crate) fn get_address(memory: &[Memsize; Memsize::MAX as usize + 1], argument: u8) -> Memsize {
    match (argument >> 3) & 1 {
        1 => memory[(argument & 0b111) as usize],
        0 => (argument & 0b111).into(),
        _ => unreachable!()
    }
}