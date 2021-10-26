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
    Twitch(Vec<ContentUrl>),
    Youtube(Vec<ContentUrl>),
    FacebookGaming(Vec<ContentUrl>),
    Douyu(Vec<ContentUrl>),
    Discord(Vec<ContentUrl>),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum InfoPlatform {
    Liquipedia(ContentUrl),
    AoeElo(i64),
    EsportsErnings(u64),
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
    /// Player ID for looking up in players.yaml file
    // player_id: u64,
    pub name: String,
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_platforms: Option<Vec<ContentPlatform>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    info_platforms: Option<Vec<InfoPlatform>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    gaming_profiles: Option<Vec<GamePlatform>>,
}
