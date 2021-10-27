use serde::{Deserialize, Serialize};

use crate::types::games::GameShortCode;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MultiplayerPlatform {
    Native(Vec<String>),
    Hd(Vec<String>),
    De(Vec<String>),
    Voobly(Vec<String>),
    GameRanger(Vec<String>),
    ESOC(Vec<String>),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GamePlatform {
    pub game_platform_id: GameShortCode,
    pub game_platform_user_ids: Vec<MultiplayerPlatform>,
}
