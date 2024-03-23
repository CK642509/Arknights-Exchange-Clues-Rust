use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_players() -> io::Result<Vec<String>> {
    let path = Path::new("src/settings/players.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut players = Vec::new();
    for line in reader.lines() {
        let line = line?.trim().to_string();
        players.push(line);
    }

    Ok(players)
}   

fn get_input_clues() -> io::Result<Vec<String>> {
    let path = Path::new("src/settings/clues.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut clues = Vec::new();
    for line in reader.lines() {
        let line = line?.trim().to_string();
        clues.push(line);
    }

    Ok(clues)
}

fn main() -> io::Result<()> {
    let players = get_players()?;
    println!("{:?}", players);

    let clues = get_input_clues()?;
    println!("{:?}", clues);
    Ok(())
}
