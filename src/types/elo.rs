use serde::{Deserialize, Serialize};

use crate::types::games::GameShortCode;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PlayerSkill {
    Beginner,
    Intermediate,
    Advanced,
    Professional,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Elo(u64);

type HighestElo = Elo;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlatformElo((GameShortCode, HighestElo));
