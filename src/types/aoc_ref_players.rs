//! Players datastructures to be used with `aoc-reference-data` repository

use serde::{Deserialize, Serialize};

/// A player from `aoc-reference data`
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Player {
    /// Unique ID
    #[serde(default)]
    pub uid: u64,
    /// Most known Name
    pub name: String,
    /// Teams
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    /// Other known Names
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aka: Vec<String>,
    /// ELO on aoeelo.com
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aoeelo: Option<i64>,
    /// Origin of a Player
    pub country: String,
    /// Discord invite to a Server of a Player
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    /// Link to a streaming platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub douyu: Option<String>,
    /// ID for that corresponding player on that platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esportsearnings: Option<i64>,
    /// Name on the platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquipedia: Option<String>,
    /// Link to a streaming platform (outdated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixer: Option<String>,
    /// Platform datastructure, contains `profile_ids`
    pub platforms: Platforms,
    /// Link to a streaming platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitch: Option<String>,
    /// Link to a streaming platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube: Option<String>,
    /// Link to a streaming platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_gaming: Option<String>,
}

#[derive(
    typed_builder::TypedBuilder,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]

pub struct PlayerToVec {
    /// Most known Name
    pub name: String,
    /// Other known Names
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aka: Vec<String>,
    /// Origin of a Player
    pub country: String,
    /// ELO on aoeelo.com
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aoeelo: Option<i64>,
    /// ID for that corresponding player on that platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esportsearnings: Option<i64>,
    /// Name on the platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquipedia: Option<String>,
    /// Link to a streaming platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub douyu: Option<Vec<String>>,
    /// Platform datastructure, contains `profile_ids`
    pub platforms: Platforms,
    /// Link to a streaming platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube: Option<Vec<String>>,
    /// Link to a streaming platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_gaming: Option<Vec<String>>,
    /// Discord invite to a Server of a Player
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<Vec<String>>,
    /// Link to a streaming platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitch: Option<Vec<String>>,
    /// Teams
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    /// Link to a streaming platform (outdated)
    #[serde(skip_serializing)]
    pub mixer: Option<Vec<String>>,
}

/// A player from `aoc-reference data` without streaming
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerNoStream {
    /// Unique ID
    #[serde(default)]
    pub uid: u64,
    /// Most known Name
    pub name: String,
    /// Teams
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    /// Other known Names
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aka: Vec<String>,
    /// ELO on aoeelo.com
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aoeelo: Option<i64>,
    /// Origin of a Player
    pub country: String,
    /// Discord invite to a Server of a Player
    #[serde(skip_serializing)]
    pub discord: Option<Vec<String>>,
    /// Link to a streaming platform
    #[serde(skip_serializing)]
    pub douyu: Option<Vec<String>>,
    /// ID for that corresponding player on that platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esportsearnings: Option<i64>,
    /// Name on the platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquipedia: Option<String>,
    /// Link to a streaming platform (outdated)
    #[serde(skip_serializing)]
    pub mixer: Option<Vec<String>>,
    /// Platform datastructure, contains `profile_ids`
    pub platforms: Platforms,
    /// Link to a streaming platform
    #[serde(skip_serializing)]
    pub twitch: Option<Vec<String>>,
    /// Link to a streaming platform
    #[serde(skip_serializing)]
    pub youtube: Option<Vec<String>>,
    /// Link to a streaming platform
    #[serde(skip_serializing)]
    pub facebook_gaming: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Platforms {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aoe4: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub de: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gamepark: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gameranger: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ibp: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub igz: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lan: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub voobly: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub vooblycn: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub zone: Vec<String>,
}
