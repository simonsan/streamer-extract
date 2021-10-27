// Run third

use std::io::BufWriter;

use aoc_util::types::aoc_ref_players::PlayerNoStream;

fn main() {
    let file = std::fs::File::open("data/edited/players.yaml").unwrap();
    let players: Vec<PlayerNoStream> = serde_yaml::from_reader(&file).unwrap();

    let file = std::fs::File::create("data/edited/players.yaml").expect("Couldn't create file.");
    let mut writer = BufWriter::new(file);

    // Write data to file
    serde_yaml::to_writer(&mut writer, &players)
        .expect("Wrting data to file experienced an error.");
}
