use serde::{
    Deserialize,
    Serialize,
};

use crate::types::GameId;

#[derive(Serialize,
           Deserialize,
           PartialEq,
           Eq,
           PartialOrd,
           Ord,
           Clone,
           Debug)]
#[serde(rename_all = "camelCase")]
pub enum GamePlatform {
    Aoe1(Vec<MultiplayerPlatform>),
    Aoe2(Vec<MultiplayerPlatform>),
    Aoe3(Vec<MultiplayerPlatform>),
    Aoe4(Vec<GameId>),
    Aom(Vec<MultiplayerPlatform>),
    AoeO(GameId),
}

#[derive(Serialize,
           Deserialize,
           PartialEq,
           Eq,
           PartialOrd,
           Ord,
           Clone,
           Debug)]
#[serde(rename_all = "camelCase")]
pub enum MultiplayerPlatform {
    De(Vec<String>),
    Voobly(Vec<String>),
    GameRanger(Vec<String>),
    ESOC(Vec<String>),
}
