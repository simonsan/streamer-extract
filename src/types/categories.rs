use serde::{Deserialize, Serialize};

use crate::types::elo::PlayerSkill;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    CastingRanked(PlayerSkill),
    PovContent(PlayerSkill),
    CastingTournaments,
    OrganizingTournaments,
    CommunityGames,
    LearningResources,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentCategory {
    pub id: usize,
    pub category: Category,
    pub description: String,
}
