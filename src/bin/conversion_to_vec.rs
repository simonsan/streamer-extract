// Run once

use std::io::BufWriter;

use aoc_util::types::aoc_ref_players::{Player, PlayerToVec};

fn main() {
    let file = std::fs::File::open("data/players.yaml").unwrap();
    let players: Vec<Player> = serde_yaml::from_reader(&file).unwrap();

    let mut players_to_vec: Vec<PlayerToVec> = vec![];

    for player in players {
        let player_new: PlayerToVec = PlayerToVec::builder()
            .name(player.name)
            .country(player.country)
            .esportsearnings(player.esportsearnings)
            .aoeelo(player.aoeelo)
            .liquipedia(player.liquipedia)
            .platforms(player.platforms)
            .team(player.team)
            .aka(player.aka)
            .discord({
                if let Some(x) = player.discord {
                    let vec = vec![x];
                    Some(vec)
                } else {
                    None
                }
            })
            .twitch({
                if let Some(x) = player.twitch {
                    let vec = vec![x];
                    Some(vec)
                } else {
                    None
                }
            })
            .douyu({
                if let Some(x) = player.douyu {
                    let vec = vec![x];
                    Some(vec)
                } else {
                    None
                }
            })
            .mixer(None)
            .youtube({
                if let Some(x) = player.youtube {
                    let vec = vec![x];
                    Some(vec)
                } else {
                    None
                }
            })
            .facebook_gaming({
                if let Some(x) = player.facebook_gaming {
                    let vec = vec![x];
                    Some(vec)
                } else {
                    None
                }
            })
            .build();
        players_to_vec.push(player_new);
    }

    let file = std::fs::File::create("data/edited/players.yaml").expect("Couldn't create file.");
    let mut writer = BufWriter::new(file);

    // Write data to file
    serde_yaml::to_writer(&mut writer, &players_to_vec)
        .expect("Wrting data to file experienced an error.");
}
