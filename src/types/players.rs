//! Players datastructures to be used with `aoc-reference-data` repository

use serde::{Deserialize, Serialize};

/// A player from `aoc-reference data`
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Player {
    /// Most known Name
    pub name: String,
    /// Other known Names
    #[serde(default)]
    pub aka: Vec<String>,
    /// ELO on aoeelo.com
    pub aoeelo: Option<i64>,
    /// Origin of a Player
    pub country: String,
    /// Discord invite to a Server of a Player
    pub discord: Option<String>,
    /// Link to a streaming platform
    pub douyu: Option<String>,
    /// ID for that corresponding player on that platform
    pub esportsearnings: Option<i64>,
    /// Name on the platform
    pub liquipedia: Option<String>,
    /// Link to a streaming platform (outdated)
    pub mixer: Option<String>,
    /// Platform datastructure, contains `profile_ids`
    pub platforms: Platforms,
    /// Link to a streaming platform
    pub twitch: Option<String>,
    /// Link to a streaming platform
    pub youtube: Option<String>,
    /// Link to a streaming platform
    pub facebook_gaming: Option<String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Platforms {
    #[serde(default)]
    pub aoe4: Vec<String>,
    #[serde(default)]
    pub de: Vec<String>,
    #[serde(default)]
    pub gamepark: Vec<String>,
    #[serde(default)]
    pub gameranger: Vec<String>,
    #[serde(default)]
    pub ibp: Vec<String>,
    #[serde(default)]
    pub igz: Vec<String>,
    #[serde(default)]
    pub lan: Vec<String>,
    #[serde(default)]
    pub voobly: Vec<String>,
    #[serde(default)]
    pub vooblycn: Vec<String>,
    #[serde(default)]
    pub zone: Vec<String>,
}
