use serde::{Deserialize, Serialize};

type EloRange = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Category {
    CastingRanked(EloRange),
    CastingTournaments,
    OrganizingTournaments,
    CommunityGames,
    PovContent(EloRange),
    LearningResources,
}
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum LanguageShortCode {
    En,
    De,
    Fr,
    Esp,
    It,
    Por,
    Rus,
    Other(String),
}

pub type GameId = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum GamePlatform {
    Aoe1(Vec<MultiplayerPlatform>),
    Aoe2(Vec<MultiplayerPlatform>),
    Aoe3(Vec<MultiplayerPlatform>),
    Aoe4(Vec<GameId>),
    Aom(Vec<MultiplayerPlatform>),
    AoeO(GameId),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum MultiplayerPlatform {
    De(Vec<String>),
    Voobly(Vec<String>),
    GameRanger(Vec<String>),
    ESOC(Vec<String>),
}

type ContentUrl = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]

pub enum ContentPlatform {
    Twitch(ContentUrl),
    Youtube(ContentUrl),
    FacebookGaming(ContentUrl),
    Douyu(ContentUrl),
    Discord(ContentUrl),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum InfoPlatform {
    Liquipedia(ContentUrl),
    AoeElo(i64),
}

#[derive(
    typed_builder::TypedBuilder,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Debug,
)]
pub struct ContentCreator {
    uid: u64,
    player_id: u64,
    pub name: String,
    #[serde(skip_serializing)]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_platforms: Option<Vec<ContentPlatform>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing)]
    info_platforms: Option<Vec<InfoPlatform>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing)]
    gaming_profiles: Option<Vec<GamePlatform>>,
}
