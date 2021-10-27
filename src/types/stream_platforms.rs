use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::ImagePath;

type PlatformBaseUrl = String;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
#[repr(u8)]
pub enum ContentPlatform {
    Twitch = 1,
    Youtube = 2,
    FacebookGaming = 3,
    Douyu = 4,
    Discord = 5,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentPlatformInfo {
    pub id: usize,
    pub platform_name: String,
    pub logo_path: Option<ImagePath>,
    pub base_url: PlatformBaseUrl,
}
