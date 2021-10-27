use serde::{
    Deserialize,
    Serialize,
};

use crate::types::ContentUrl;

#[derive(Serialize,
           Deserialize,
           PartialEq,
           Eq,
           PartialOrd,
           Ord,
           Clone,
           Debug)]
#[serde(rename_all = "camelCase")]
pub enum InfoPlatform {
    Liquipedia(ContentUrl),
    AoeElo(i64),
    EsportsErnings(u64),
}
