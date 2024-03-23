use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NUM_CLUES: usize = 7;

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

fn create_player_store(s: &str) -> Vec<i32> {
    let mut counts = vec![0; NUM_CLUES];
    for digit in s.chars() {
        let index = digit.to_digit(10).unwrap() as usize - 1;
        counts[index] += 1;
    }
    counts
}

fn create_player_want(s: &str) -> Vec<i32> {
    let mut counts = vec![0; NUM_CLUES];
    if s == "0" {
        return counts;
    }

    for digit in s.chars() {
        let index = digit.to_digit(10).unwrap() as usize - 1;
        counts[index] = 1;
    }
    counts
}

fn main() -> io::Result<()> {
    let players = get_players()?;
    println!("{:?}", players);

    let clues = get_input_clues()?;
    println!("{:?}", clues);

    let mut all_stores = Vec::new();
    let mut all_wants = Vec::new();

    for clue in clues {
        let parts: Vec<&str> = clue.split_whitespace().collect();
        let store = parts[0];
        let want = parts[1];
        all_stores.push(create_player_store(store));
        all_wants.push(create_player_want(want));
    }

    println!("{:?}", all_stores);
    println!("{:?}", all_wants);


    Ok(())
}
