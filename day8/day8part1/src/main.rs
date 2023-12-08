use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Point {
    left: String,
    right: String
}

fn main() {
    let mut directions:Vec<char> = Vec::new();
    let mut point_map: HashMap<String, Point> = HashMap::new();
    if let Ok(lines) = read_lines("./inputs/input.txt") {
        for (index, line) in lines.enumerate() {
            if let Ok(line_text) = line {
                if index == 0 {
                    directions = line_text.chars().collect();
                } else if index > 1 {
                    let point = build_point_map(line_text);
                    point_map.insert(point.0, point.1);
                }
            }
        }
    }

    let mut current_key = "AAA";
    let mut steps = 0;

    let mut current_directional_index = 0;
    while current_key != "ZZZ" {
        let current_point = point_map.get(current_key).unwrap();
        let current_direction = directions[current_directional_index];
        current_key = match current_direction {
            'R' => &current_point.right,
            'L' => &current_point.left,
            _ => panic!("not found")
        };
        if current_directional_index == directions.len() - 1 {
            current_directional_index = 0;
        } else {
            current_directional_index += 1;
        }
        steps += 1;
    }
    println!("Total Steps: {}", steps);
}

fn build_point_map(line: String) -> (String, Point) {
    (line[0..3].to_string(), Point { left: line[7..10].to_string(), right: line[12..15].to_string() })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}