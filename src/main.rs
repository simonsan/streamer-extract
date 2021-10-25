pub mod types;

use std::io::BufWriter;

use types::players::Player;
use types::streamers::{ContentCreator, MultiplayerPlatform};

use crate::types::streamers::{ContentPlatform, GamePlatform, InfoPlatform};

fn main() {
    let file = std::fs::File::open("data/players.yaml").unwrap();
    let players: Vec<Player> = serde_yaml::from_reader(&file).unwrap();

    let mut content_creators: Vec<ContentCreator> = vec![];

    for player in &players {
        let content_creator = ContentCreator::builder()
            .name(player.name.clone())
            .country(if !player.country.is_empty() {
                Some(player.country.to_string())
            } else {
                None
            })
            .content_languages(if !player.country.is_empty() {
                Some(vec![player.country.to_string()])
            } else {
                None
            })
            .content_platforms({
                let mut platforms: Vec<ContentPlatform> = vec![];

                if let Some(id) = &player.twitch {
                    platforms.push(ContentPlatform::Twitch(id.to_string()))
                }

                if let Some(id) = &player.youtube {
                    platforms.push(ContentPlatform::Youtube(id.to_string()))
                }

                if let Some(id) = &player.facebook_gaming {
                    platforms.push(ContentPlatform::FacebookGaming(id.to_string()))
                }

                if let Some(id) = &player.douyu {
                    platforms.push(ContentPlatform::Douyu(id.to_string()))
                }

                if let Some(id) = &player.discord {
                    platforms.push(ContentPlatform::Discord(id.to_string()))
                }

                if !platforms.is_empty() {
                    Some(platforms)
                } else {
                    None
                }
            })
            .info_platforms({
                let mut platforms: Vec<InfoPlatform> = vec![];

                if let Some(id) = &player.liquipedia {
                    platforms.push(InfoPlatform::Liquipedia(id.to_string()))
                }

                if let Some(id) = &player.aoeelo {
                    platforms.push(InfoPlatform::AoeElo(*id))
                }

                if !platforms.is_empty() {
                    Some(platforms)
                } else {
                    None
                }
            })
            .gaming_profiles({
                let mut game_ids: Vec<GamePlatform> = vec![];
                let mut platform_ids: Vec<MultiplayerPlatform> = vec![];

                // AoE2
                platform_ids.push(MultiplayerPlatform::De(player.platforms.de.clone()));
                platform_ids.push(MultiplayerPlatform::Voobly(player.platforms.voobly.clone()));
                game_ids.push(GamePlatform::Aoe2(platform_ids));

                // AoE4
                game_ids.push(GamePlatform::Aoe4(player.platforms.aoe4.clone()));

                if !game_ids.is_empty() {
                    Some(game_ids)
                } else {
                    None
                }
            })
            .build();

        content_creators.push(content_creator);
    }
    content_creators.sort();

    let file = std::fs::File::create("data/streamers.yaml").expect("Couldn't create file.");
    let mut writer = BufWriter::new(file);

    // Write data to file
    serde_yaml::to_writer(&mut writer, &content_creators)
        .expect("Wrting data to file experienced an error.");
}
