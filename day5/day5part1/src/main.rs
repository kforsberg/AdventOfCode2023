use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[derive(Clone, Copy)]
struct AlmanacEntry {
    source_range_start: i64,
    destination_range_start: i64,
    range_length: i64
}

fn main() {
    let mut seeds: Vec<i64> = Vec::new();

    // seed-to-soil
    // soil-to-fertilizer
    // fertilizer-to-water
    // water-to-light
    // light-to-temperature
    // temperature-to-humidity
    // humidity-to-location
    let mut all_maps: Vec<Vec<AlmanacEntry>> = Vec::new();
    let mut current_map: Vec<AlmanacEntry> = Vec::new();
    if let Ok(lines) = read_lines("inputs/input.txt") {
        for (index, line) in lines.enumerate() {
            if let Ok(line_text) = line {
                if index == 0 {
                    seeds = parse_integers(line_text);
                } else if index == 1 || index == 2 || line_text.contains("map") {
                    continue; // we can always ignore these lines
                } else {
                    if line_text == "" || line_text == "END" {
                         all_maps.insert(all_maps.len(), current_map.clone());
                         current_map.clear();
                         continue;
                    }
                    let parsed_int = parse_integers(line_text);
                    current_map.insert(current_map.len(), AlmanacEntry { source_range_start: parsed_int[1], destination_range_start: parsed_int[0], range_length: parsed_int[2] });
                }
            }
        }
    }
    let mut totals: Vec<i64> = Vec::new();
    for seed in seeds {
        totals.insert(totals.len(), process_seed(seed, all_maps.clone()));
    }
    totals.sort();
    println!("The smallest is {}", totals[0]);
}

fn process_seed(seed: i64, maps: Vec<Vec<AlmanacEntry>>) -> i64 {
    let mut destination_id: i64 = seed;
    for map in maps {
        let filtered_entries = map.iter().filter(|s| { destination_id >= s.source_range_start && destination_id <= s.source_range_start + s.range_length }).collect::<Vec<&AlmanacEntry>>();
        if filtered_entries.len() > 0 {
            destination_id = destination_id + (filtered_entries[0].destination_range_start - filtered_entries[0].source_range_start);
        }
    }
    return destination_id;
}

fn parse_integers(input: String) -> Vec<i64> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(&input).filter_map(|num| num.as_str().parse::<i64>().ok()).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
