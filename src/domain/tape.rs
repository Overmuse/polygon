use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Tape {
    A = 1,
    B = 2,
    C = 3,
}
