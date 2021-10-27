use crate::types::{
    categories::ContentCategory, content_creator::ContentCreatorInfo,
    stream_platforms::ContentPlatformInfo,
};

use serde::{Deserialize, Serialize};

pub mod aoc_ref_players;
pub mod categories;
pub mod content_creator;
pub mod des_streamers;
pub mod elo;
pub mod game_platforms;
pub mod games;
pub mod info_platforms;
pub mod languages;
pub mod stream_platforms;

type ContentUrl = String;
type ImagePath = String;
pub type GameId = String;

#[derive(typed_builder::TypedBuilder, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    content_platform_infos: Vec<ContentPlatformInfo>,
    content_categories: Vec<ContentCategory>,
    content_creators: Vec<ContentCreatorInfo>,
}
