use strum_macros::EnumTryAs;

// TODO: Recheck type for value
#[derive(Debug, Clone, PartialEq, Eq, EnumTryAs)]
pub enum Reference {
    Direct(String),
    Indirect(String),
    Addr(String),
    Value(u16),
}
