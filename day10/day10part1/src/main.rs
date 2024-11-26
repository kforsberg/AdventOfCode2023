use std::{collections::VecDeque, fs, io::Error};

struct NearyCharacters {
    above: Option<char>,
    below: Option<char>,
    left: Option<char>,
    right: Option<char>,
}

impl NearyCharacters {
    fn new() -> NearyCharacters {
        NearyCharacters { 
            above: None, 
            below: None, 
            left: None, 
            right: None 
        }
    }
}

fn main() {
    if let Ok(grid) = build_input() {
        if let Ok(starting_point) = find_starting_row(&grid) {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            let mut traveled_paths: Vec<(usize,usize)> = Vec::new();
            traveled_paths.insert(0, starting_point);
            queue.push_back(starting_point);
            while !queue.is_empty() {
                let (row, column) = queue.pop_front().unwrap();
                let current_character = *grid.get(row).unwrap().get(column).unwrap();
                let nearby_characters = get_nearby_characters(&grid, row, column);

                if can_move_up(current_character, &nearby_characters) && !traveled_paths.contains(&(row - 1, column)) {
                    let element = (row - 1, column);
                    traveled_paths.insert(traveled_paths.len(), element);
                    queue.push_back(element);
                } if can_move_down(current_character, &nearby_characters) && !traveled_paths.contains(&(row + 1, column)) {
                    let element = (row + 1, column);
                    traveled_paths.insert(traveled_paths.len(), element);
                    queue.push_back(element);
                } if can_move_left(current_character, &nearby_characters) && !traveled_paths.contains(&(row, column - 1)) {
                    let element = (row, column - 1);
                    traveled_paths.insert(traveled_paths.len(), element);
                    queue.push_back(element);
                } if can_move_right(current_character, &nearby_characters) && !traveled_paths.contains(&(row, column + 1)) {
                    let element = (row, column + 1);
                    traveled_paths.insert(traveled_paths.len(), element);
                    queue.push_back(element);
                }
            }
            println!("{}", traveled_paths.len() / 2);
        } else {
            panic!("Could not find starting point");
        }
        
    } else {
        panic!("Error building grid");
    }
}

fn build_input() -> Result<Vec<Vec<char>>, Error> {
    let lines = fs::read_to_string("./inputs/input.txt")?;
    let mut total_results: Vec<Vec<char>> = vec![];
    for line in lines.split("\n").into_iter() {
        let line_items: Vec<char> = line.chars().collect();
        total_results.insert(total_results.len(), line_items);
    }
    Ok(total_results)
}

/// Finds the starting position of a grid of characters
/// Expects the starting position to be 'S'
/// 
/// ### Returns
/// (row, column)
fn find_starting_row(grid: &Vec<Vec<char>>) -> Result<(usize, usize), Error> {
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if *column == 'S' {
                return Ok((row_index, column_index));
            }
        }
    }
    return Err(Error::other("Could not find starting position"));
}

fn get_nearby_characters(grid: &Vec<Vec<char>>, row: usize, column: usize) -> NearyCharacters {
    let mut characters = NearyCharacters::new();

    if row > 0 {
        characters.above = grid.get(row - 1).unwrap().get(column).copied();
    }

    if row < grid.len() - 1 {
        characters.below = grid.get(row + 1).unwrap().get(column).copied();
    }

    if column > 0 {
        characters.left = grid.get(row).unwrap().get(column - 1).copied();
    }

    if column < grid.get(row).unwrap().len() {
        characters.right = grid.get(row).unwrap().get(column + 1).copied();
    }
    return characters;
}

fn can_move_up(current_char: char, nearby_characters: &NearyCharacters) -> bool {
    return nearby_characters.above.is_some() && "S|JL".contains(current_char) && "|7F".contains(nearby_characters.above.unwrap())
}

fn can_move_down(current_char: char, nearby_characters: &NearyCharacters) -> bool {
    return nearby_characters.below.is_some() && "S|7F".contains(current_char) && "|JL".contains(nearby_characters.below.unwrap())
}

fn can_move_left(current_char: char, nearby_characters: &NearyCharacters) -> bool {
    return nearby_characters.left.is_some() && "S-J7".contains(current_char) && "-FL".contains(nearby_characters.left.unwrap())
}


fn can_move_right(current_char: char, nearby_characters: &NearyCharacters) -> bool {
    return nearby_characters.right.is_some() && "S-LF".contains(current_char) && "-J7".contains(nearby_characters.right.unwrap())
}