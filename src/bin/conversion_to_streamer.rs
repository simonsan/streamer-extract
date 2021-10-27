// Run second

use aoc_util::types::{
    aoc_ref_players::PlayerToVec,
    categories::{Category, ContentCategory},
    content_creator::{ContentCreatorInfo, ContentCreatorPlatformInfo},
    elo::PlayerSkill,
    game_platforms::{GamePlatform, MultiplayerPlatform},
    info_platforms::InfoPlatform,
    stream_platforms::{ContentPlatform, ContentPlatformInfo},
    Api,
};
use std::io::BufWriter;

fn main() {
    let file = std::fs::File::open("data/edited/players.yaml").unwrap();
    let mut players: Vec<PlayerToVec> = serde_yaml::from_reader(&file).unwrap();

    players.retain(|orig| {
        orig.twitch.is_some()
            | orig.douyu.is_some()
            | orig.youtube.is_some()
            | orig.facebook_gaming.is_some()
            | orig.discord.is_some()
    });

    let content_platforms: Vec<ContentPlatformInfo> = vec![
        ContentPlatformInfo {
            id: 1,
            platform_name: "Twitch".to_string(),
            logo_path: Some("media/logos/twitch.png".to_string()),
            base_url: "https://www.twitch.tv".to_string(),
        },
        ContentPlatformInfo {
            id: 2,
            platform_name: "Youtube".to_string(),
            logo_path: Some("media/logos/youtube.png".to_string()),
            base_url: "https://www.youtube.com".to_string(),
        },
        ContentPlatformInfo {
            id: 3,
            platform_name: "FacebookGaming".to_string(),
            logo_path: Some("media/logos/facebook-gaming.png".to_string()),
            base_url: "https://www.facebook.com/gaming".to_string(),
        },
        ContentPlatformInfo {
            id: 4,
            platform_name: "Douyu".to_string(),
            logo_path: Some("media/logos/douyu.png".to_string()),
            base_url: "https://www.douyu.com".to_string(),
        },
        ContentPlatformInfo {
            id: 5,
            platform_name: "Discord".to_string(),
            logo_path: Some("media/logos/discord.png".to_string()),
            base_url: "https://discord.com/invite".to_string(),
        },
    ];

    let content_categories: Vec<ContentCategory> = vec![
        ContentCategory {
            id: 1,
            category: Category::CastingRanked(PlayerSkill::Beginner),
            description: "Casting players under 800 Elo".to_string(),
        },
        ContentCategory {
            id: 2,
            category: Category::CastingRanked(PlayerSkill::Intermediate),
            description: "Casting players from 800 to 1400 Elo".to_string(),
        },
        ContentCategory {
            id: 3,
            category: Category::CastingRanked(PlayerSkill::Advanced),
            description: "Casting players from 1400 to 2000 Elo".to_string(),
        },
        ContentCategory {
            id: 4,
            category: Category::CastingRanked(PlayerSkill::Professional),
            description: "Casting players from 2000 Elo onwards".to_string(),
        },
        ContentCategory {
            id: 5,
            category: Category::PovContent(PlayerSkill::Beginner),
            description: "POV of players under 800 Elo".to_string(),
        },
        ContentCategory {
            id: 6,
            category: Category::PovContent(PlayerSkill::Intermediate),
            description: "POV of players from 800 to 1400 Elo".to_string(),
        },
        ContentCategory {
            id: 7,
            category: Category::PovContent(PlayerSkill::Advanced),
            description: "POV of players from 1400 to 2000 Elo".to_string(),
        },
        ContentCategory {
            id: 8,
            category: Category::PovContent(PlayerSkill::Professional),
            description: "POV of players from 2000 Elo onwards".to_string(),
        },
        ContentCategory {
            id: 9,
            category: Category::CastingTournaments,
            description: "Casting of ongoing tournaments".to_string(),
        },
        ContentCategory {
            id: 10,
            category: Category::OrganizingTournaments,
            description: "Organizing own tournaments".to_string(),
        },
        ContentCategory {
            id: 11,
            category: Category::CommunityGames,
            description: "Organizing and/or playing community games".to_string(),
        },
        ContentCategory {
            id: 12,
            category: Category::LearningResources,
            description: "Creating content to learn about the game".to_string(),
        },
    ];

    let mut content_creators: Vec<ContentCreatorInfo> = vec![];
    let mut uid: usize = 1;

    for player in &players {
        let content_creator = ContentCreatorInfo::builder()
            .id(uid)
            .name(player.name.clone())
            .aoc_ref_id(None)
            .country(if !player.country.is_empty() {
                Some(player.country.to_string())
            } else {
                None
            })
            .bio(None)
            .image(None)
            .following(None)
            .content_platforms({
                let mut platforms: Vec<ContentCreatorPlatformInfo> = vec![];

                if let Some(streams) = &player.twitch {
                    let mut id: usize = 1;
                    for stream in streams {
                        platforms.push(
                            ContentCreatorPlatformInfo::builder()
                                .id(id)
                                .content_platform_id(ContentPlatform::Twitch)
                                .creator_url(
                                    stream
                                        .split('/')
                                        .collect::<Vec<&str>>()
                                        .last()
                                        .unwrap()
                                        .to_string(),
                                )
                                .content_languages(vec![player.country.to_string()])
                                .content_categories(vec![])
                                .build(),
                        );
                        id += 1;
                    }
                }

                if let Some(streams) = &player.youtube {
                    let mut id: usize = 1;

                    for stream in streams {
                        platforms.push(
                            ContentCreatorPlatformInfo::builder()
                                .id(id)
                                .content_platform_id(ContentPlatform::Youtube)
                                .creator_url(
                                    stream
                                        .split('/')
                                        .collect::<Vec<&str>>()
                                        .last()
                                        .unwrap()
                                        .to_string(),
                                )
                                .content_languages(vec![player.country.to_string()])
                                .content_categories(vec![])
                                .build(),
                        );
                        id += 1;
                    }
                }

                if let Some(streams) = &player.facebook_gaming {
                    let mut id: usize = 1;

                    for stream in streams {
                        platforms.push(
                            ContentCreatorPlatformInfo::builder()
                                .id(id)
                                .content_platform_id(ContentPlatform::FacebookGaming)
                                .creator_url(
                                    stream
                                        .split('/')
                                        .collect::<Vec<&str>>()
                                        .last()
                                        .unwrap()
                                        .to_string(),
                                )
                                .content_languages(vec![player.country.to_string()])
                                .content_categories(vec![])
                                .build(),
                        );
                        id += 1;
                    }
                }

                if let Some(streams) = &player.douyu {
                    let mut id: usize = 1;

                    for stream in streams {
                        platforms.push(
                            ContentCreatorPlatformInfo::builder()
                                .id(id)
                                .content_platform_id(ContentPlatform::Douyu)
                                .creator_url(
                                    stream
                                        .split('/')
                                        .collect::<Vec<&str>>()
                                        .last()
                                        .unwrap()
                                        .to_string(),
                                )
                                .content_languages(vec![player.country.to_string()])
                                .content_categories(vec![])
                                .build(),
                        );
                        id += 1;
                    }
                }

                if let Some(streams) = &player.discord {
                    let mut id: usize = 1;

                    for stream in streams {
                        platforms.push(
                            ContentCreatorPlatformInfo::builder()
                                .id(id)
                                .content_platform_id(ContentPlatform::Discord)
                                .creator_url(
                                    stream
                                        .split('/')
                                        .collect::<Vec<&str>>()
                                        .last()
                                        .unwrap()
                                        .to_string(),
                                )
                                .content_languages(vec![player.country.to_string()])
                                .content_categories(vec![])
                                .build(),
                        );
                        id += 1;
                    }
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
                if !player.platforms.de.is_empty() {
                    platform_ids.push(MultiplayerPlatform::De(player.platforms.de.clone()));
                }
                if !player.platforms.voobly.is_empty() {
                    platform_ids.push(MultiplayerPlatform::Voobly(player.platforms.voobly.clone()));
                }
                game_ids.push(GamePlatform::Aoe2(platform_ids));

                // AoE4
                if !player.platforms.aoe4.is_empty() {
                    game_ids.push(GamePlatform::Aoe4(player.platforms.aoe4.clone()));
                }

                if !game_ids.is_empty() {
                    Some(game_ids)
                } else {
                    None
                }
            })
            .platform_elos(None)
            .build();

        content_creators.push(content_creator);

        // Increment user id
        uid += 1;
    }
    content_creators.sort();

    let api = Api::builder()
        .content_platform_infos(content_platforms)
        .content_categories(content_categories)
        .content_creators(content_creators)
        .build();

    let file = std::fs::File::create("data/edited/streamers.json").expect("Couldn't create file.");
    let mut writer = BufWriter::new(file);

    // Write data to file
    serde_json::to_writer_pretty(&mut writer, &api)
        .expect("Wrting data to file experienced an error.");
}
