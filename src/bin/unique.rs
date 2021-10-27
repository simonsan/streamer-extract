use std::io::BufWriter;

use aoc_util::types::aoc_ref_players::Player;

// Run first

fn main() {
    let file = std::fs::File::open("data/players.yaml").unwrap();
    let players: Vec<Player> = serde_yaml::from_reader(&file).unwrap();

    let mut num: u64 = 1;
    let mut unique_players: Vec<Player> = vec![];

    for mut player in players {
        player.uid = num;
        unique_players.push(player);
        num += 1;
    }

    let file = std::fs::File::create("data/edited/players.yaml").expect("Couldn't create file.");
    let mut writer = BufWriter::new(file);

    // Write data to file
    serde_yaml::to_writer(&mut writer, &unique_players)
        .expect("Wrting data to file experienced an error.");
}
