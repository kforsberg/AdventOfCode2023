use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::path::Path;
use regex::Regex;

struct AlmanacEntry {
    source_range_start: u32,
    destination_range_start: u32,
    range_length: u32
}

fn main() {
    let mut seeds: Vec<u32> = Vec::new();

    // seed-to-soil
    // soil-to-fertilizer
    //fertilizer-to-water
    // water-to-light
    // light-to-temperature
    // temperature-to-humidity
    // humidity-to-location
    let mut maps: Vec<Vec<u32>> = Vec::new();

    if let Ok(lines) = read_lines("inputs/input.txt") {
        for (index, line) in lines.enumerate() {
            if let Ok(line_text) = line {
                if index == 0 {
                    seeds = parse_integers(line_text);
                } else {
                    
                }
            }
        }
    }
}

fn build_maps() {

}

fn parse_integers(input: String) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(&input).filter_map(|num| num.as_str().parse::<u32>().ok()).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
