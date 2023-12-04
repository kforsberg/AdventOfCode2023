use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

struct ScratchCard {
    winning_numbers: Vec<u16>,
    my_numbers: Vec<u16>
}

fn main() {
    let mut total = 0;
    if let Ok(lines) = read_lines("./inputs/input.txt") {
        for line in lines {
            if let Ok(scratch_card_line) = line {
                let card_without_id = remove_card_id(scratch_card_line);
                let scratch_card = create_scratch_card(card_without_id);
                total += process_scatch_card(scratch_card);
            }
        }
    }
    println!("{}", total);
}

fn create_scratch_card(line: String) -> ScratchCard {
    let re = Regex::new(r"\d+").unwrap();
    let card_parts = line.split('|').collect::<Vec<&str>>();
    let card_numbers: Vec<u16> = re.find_iter(card_parts[0]).filter_map(|num| num.as_str().parse::<u16>().ok()).collect();
    let my_numbers: Vec<u16> = re.find_iter(card_parts[1]).filter_map(|num| num.as_str().parse::<u16>().ok()).collect();
    ScratchCard { winning_numbers: card_numbers, my_numbers: my_numbers }
}

fn process_scatch_card(card: ScratchCard) -> u32 {
    let mut total = 0;
    for winning_number in card.winning_numbers {
        if card.my_numbers.contains(&winning_number) {
            if total == 0 {
                total = 1;
            } else {
                total = total*2;
            }
        }
    }
    return total;

}

fn remove_card_id(game: String) -> String {
    let game_id_end_pos = game.chars().position(|c| c == ':').unwrap();
    game.chars().skip(game_id_end_pos + 2).take(game.len() - game_id_end_pos).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
