use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::ImagePath;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[repr(u8)]
pub enum GameShortCode {
    Aoe1 = 1,
    Aoe2 = 2,
    Aoe3 = 3,
    Aom = 4,
    AoeO = 5,
    Aoe4 = 6,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentGames {
    pub id: usize,
    pub game_name: String,
    pub logo_path: Option<ImagePath>,
}
