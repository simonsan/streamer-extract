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

pub type UserId = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum GamePlatform {
    Aoe1(Vec<MultiplayerPlatform>),
    Aoe2(Vec<MultiplayerPlatform>),
    Aoe3(Vec<MultiplayerPlatform>),
    Aoe4(Vec<UserId>),
    Aom(Vec<MultiplayerPlatform>),
    AoeO(UserId),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum MultiplayerPlatform {
    De(Vec<String>),
    Voobly(Vec<String>),
    GameRanger(Vec<String>),
    ESOC(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContentCreatorPlatformInfo {
    content_platform: ContentPlatform,
    creator_url: ContentUrl,
    content_languages: Vec<LanguageShortCode>,
    content_categories: Vec<Category>,
}

type ContentUrl = String;
type ImagePath = String;
type PlatformBaseUrl = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum ContentPlatform {
    Twitch(ContentPlatformInfo),
    Youtube(ContentPlatformInfo),
    FacebookGaming(ContentPlatformInfo),
    Douyu(ContentPlatformInfo),
    Discord(ContentPlatformInfo),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct ContentPlatformInfo {
    pub logo: ImagePath,
    pub base_url: PlatformBaseUrl,
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
pub struct ContentCreatorInfo {
    pub id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aoc_ref_id: Option<u64>,
    pub name: String,
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImagePath>,
    #[serde(skip_serializing)]
    pub following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_platforms: Option<Vec<ContentCreatorPlatformInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_platforms: Option<Vec<InfoPlatform>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gaming_profiles: Option<Vec<GamePlatform>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_elos: Option<Vec<PlatformElo>>,
}
