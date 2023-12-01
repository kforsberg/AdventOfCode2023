use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use phf::phf_map;

static WORD_NUMBER_MAP: phf::Map<&'static str, u8> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9
};

fn main() {
    let mut all_line_codes: Vec<Vec<u8>> = Vec::new();
    if let Ok(calibration_values) = read_lines("./inputs/input.txt") {
        for line in calibration_values {
            let mut line_codes: Vec<u8> = Vec::new();
            if let Ok(calibration_value) = line  {
                line_codes = parse_line_with_words(calibration_value);
            }
            all_line_codes.insert(all_line_codes.len(), line_codes);
        }
    }
    let code = get_calibration_code(all_line_codes);
    println!("Your code is {}", code);
}

fn get_calibration_code(code_list: Vec<Vec<u8>>) -> u32 {
    let mut total: u32 = 0;
    for codes in code_list {
        let first = codes[0];
        let last = codes[codes.len()-1];
        let str = format!("{}{}", first, last);
        if let Ok(first_and_last) = str.parse::<u32>() {
            total += first_and_last;
        }
    }

    return total;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line_with_words(line: String) -> Vec<u8> {
    let mut line_codes: Vec<u8> = Vec::new();

    for (index, character) in line.chars().enumerate() {
        if let Ok(calibration_number) = character.to_string().parse::<u8>() {
            line_codes.insert(line_codes.len(), calibration_number);
        } else {
            // go through the rest of the string until there's a match found in the map
            let remaining_string = line[index..line.len() ].to_string();
            for (inner_index, inner_character) in remaining_string.chars().enumerate() {
                let sub = remaining_string[0..inner_index+1].to_string();
                if let Some(str) = WORD_NUMBER_MAP.get(&sub) {
                    line_codes.insert(line_codes.len(), *str);
                }
            }
        }
    }
    return line_codes;
}