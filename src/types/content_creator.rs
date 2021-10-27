use serde::{Deserialize, Serialize};

use crate::types::{
    categories::Category, elo::PlatformElo, game_platforms::GamePlatform,
    info_platforms::InfoPlatform, stream_platforms::ContentPlatform, ContentUrl, ImagePath,
};

type LanguageShortCode = String;
type ContentPlatformRepr = ContentPlatform;

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
#[serde(rename_all = "camelCase")]
pub struct ContentCreatorPlatformInfo {
    id: usize,
    content_platform_id: ContentPlatformRepr,
    creator_url: ContentUrl,
    content_languages: Vec<LanguageShortCode>,
    content_categories: Vec<Category>,
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
#[serde(rename_all = "camelCase")]
pub struct ContentCreatorInfo {
    pub id: usize,
    pub aoc_ref_id: Option<u64>,
    pub name: String,
    pub country: Option<String>,
    pub bio: Option<String>,
    pub image: Option<ImagePath>,
    #[serde(skip)]
    pub following: Option<bool>,
    pub content_platforms: Option<Vec<ContentCreatorPlatformInfo>>,
    pub info_platforms: Option<Vec<InfoPlatform>>,
    pub gaming_profiles: Option<Vec<GamePlatform>>,
    pub platform_elos: Option<Vec<PlatformElo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentCreatorInfoWrapper {
    pub profile: ContentCreatorInfo,
}
