use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[repr(u8)]
pub enum GameShortCode {
    Aoe1 = 1,
    Aoe2 = 2,
    Aoe3 = 3,
    Aoe4 = 4,
    Aom = 5,
    AoeO = 6,
}
