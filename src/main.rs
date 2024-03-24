use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod condition_evaluator;
use condition_evaluator::ConditionEvaluator;

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

    let n_players = players.len();

    // create player stores and wants
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

    // check stores and wants
    for (i, player_stores) in all_stores.iter().enumerate() {
        for (j, &store) in player_stores.iter().enumerate() {
            // change want to 1 if store >= 3
            if store >= 3 {
                all_wants[i][j] = 1;
            }

            // change want to -1 if store is 0
            if store == 0 {
                if all_wants[i][j] == 1 {
                    println!("Player {} does not have clue {}, cannot exchange with others", i + 1, j + 1);
                }
                all_wants[i][j] = -1;
            }
        }
    }

    for i in 0..NUM_CLUES {
        let mut total_players_with_clue = 0;
        for j in 0..n_players {
            if all_stores[j][i] > 0 {
                total_players_with_clue += 1;
            }
        }

        if total_players_with_clue < 2 {
            println!("線索{}的玩家數量不足", i + 1);
            for j in 0..n_players {
                all_wants[j][i] = -1;
            }
        }
    }

    println!("===================== After checking =====================");
    println!("{:?}", all_stores);
    println!("{:?}", all_wants);

    let mut evaluator = ConditionEvaluator::new(all_wants);

    Ok(())
}
