use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[derive(Clone)]
struct ScratchCard {
    winning_numbers: Vec<u16>,
    my_numbers: Vec<u16>,
    copies: u32
}

fn main() {
    let mut total = 0;
    let initial_scratch_cards = build_initial_scratch_cards();
    let final_scratch_cards = build_final_scratch_cards(initial_scratch_cards);
    for card in final_scratch_cards {
        total += process_scatch_card(&card);
    }
    
    println!("{}", total);
}

fn build_initial_scratch_cards() -> Vec<ScratchCard> {
    let mut initial_scratch_cards: Vec<ScratchCard> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/test.txt") {
        for line in lines {
            if let Ok(scratch_card_line) = line {
                let card_without_id = remove_card_id(scratch_card_line);
                let scratch_card = create_scratch_card(card_without_id);
                initial_scratch_cards.insert(initial_scratch_cards.len(), scratch_card);
            }
        }
    }
    return initial_scratch_cards;
}

fn build_final_scratch_cards(initial_card_list: Vec<ScratchCard>) -> Vec<ScratchCard> {
    let mut final_scratch_cards: Vec<ScratchCard> = Vec::new();
    
    return final_scratch_cards;
}

fn get_number_of_matches(card: &ScratchCard) -> usize {
    let mut total = 0;
    for winning_number in &card.winning_numbers {
        if card.my_numbers.contains(&winning_number) {
            total += 1;
        }
    }
    return total;
}

fn process_scatch_card(card: &ScratchCard) -> u32 {
    let mut total = 0;
    for winning_number in &card.winning_numbers {
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

fn create_scratch_card(line: String) -> ScratchCard {
    let re = Regex::new(r"\d+").unwrap();
    let card_parts = line.split('|').collect::<Vec<&str>>();
    let card_numbers: Vec<u16> = re.find_iter(card_parts[0]).filter_map(|num| num.as_str().parse::<u16>().ok()).collect();
    let my_numbers: Vec<u16> = re.find_iter(card_parts[1]).filter_map(|num| num.as_str().parse::<u16>().ok()).collect();
    ScratchCard { winning_numbers: card_numbers, my_numbers: my_numbers, copies: 0 }
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
