use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


struct GameResult {
    color: String,
    amount: u32
}

static MAX_GREEN: u32 = 13;
static MAX_RED: u32 = 12;
static MAX_BLUE: u32 = 14;


fn main() {
    let mut game_results: Vec<Vec<GameResult>> = Vec::new();
    if let Ok(game_lines) = read_lines("./inputs/test.txt") {
        for game_line in game_lines {
            if let Ok(mut game) = game_line {
                game = remove_game_id(game);
                let rounds = game.split(";");
                let mut round_results: Vec<GameResult> = Vec::new();
                for round in rounds {
                    for color in round.split(",") {
                        let parts: Vec<&str> = color.trim().split(" ").collect();
                        if let Ok(amount) = parts[0].parse::<u32>() {
                            round_results.insert(round_results.len(), GameResult { color: parts[1].to_string(), amount: amount })
                        }
                    }
                }
                game_results.insert(game_results.len(), round_results);
            }
        }
    }
    // part_one(game_results);
    part_two(game_results);
}

fn part_one(game_results: Vec<Vec<GameResult>>) {
    let mut game_id = 0;
    let mut total = 0;
    for game in game_results {
        let mut should_add = true;
        game_id += 1;
        for round in game {
            if round.color == "blue" && round.amount > MAX_BLUE {
                should_add = false;
                break;
            } else if round.color == "red" && round.amount > MAX_RED {
                should_add = false;
                break;
            } else if round.color == "green" && round.amount > MAX_GREEN {
                should_add = false;
                break;
            }
        }
        if should_add {
            total += game_id;
        }
    }

    println!("{}", total);
}

fn part_two(game_results: Vec<Vec<GameResult>>) {
    let mut total = 0;
    for game in game_results {
        let mut red:Vec<u32> = game.iter().filter(|g| { g.color == "red"}).map(|g| { g.amount }).collect();
        let mut green:Vec<u32> = game.iter().filter(|g| { g.color == "green"}).map(|g| { g.amount }).collect();
        let mut blue:Vec<u32> = game.iter().filter(|g| { g.color == "blue"}).map(|g| { g.amount }).collect();
        red.sort();
        green.sort();
        blue.sort();

        total += blue.get(blue.len() - 1).unwrap() * red.get(red.len() - 1).unwrap() * green.get(green.len() - 1).unwrap();

    }
    println!("{}", total)
}

fn remove_game_id(game: String) -> String {
    let game_id_end_pos = game.chars().position(|c| c == ':').unwrap();
    game.chars().skip(game_id_end_pos + 2).take(game.len() - game_id_end_pos).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
