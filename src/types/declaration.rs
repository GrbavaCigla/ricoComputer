// TODO: Recheck type for value
#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub symbol: String,
    pub value: u16,
}

// TODO: Recheck type for value
#[derive(Debug)]
pub enum Reference {
    Direct(String),
    Indirect(String),
    Addr(String),
    Value(u16),
}
