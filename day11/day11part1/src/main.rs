use std::{fs, io::Error };

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Point {
        return Point { x, y };
    }
}

fn main() {
    if let Ok(mut grid) = build_input() {
        let empty_rows = get_empty_rows(&mut grid);
        let empty_columns = get_empty_columns(&mut grid);
        let points = get_galaxy_coordinates(&grid, &empty_rows, &empty_columns);
        let mut total: u64 = 0;
        for (index, point) in points.iter().enumerate() {
            for (other_index, other_point) in points.iter().enumerate() {
                if index == other_index {
                    continue;
                }
                total += calculate_distance(point, other_point);
            }
        }
        println!("{}", total / 2);
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

fn get_empty_rows(grid: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_rows: Vec<usize> = vec![];
    for (row_index, row) in grid.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            empty_rows.insert(empty_rows.len(), row_index);
        }
    }
    return empty_rows;
}

fn get_empty_columns(grid: &mut Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_columns: Vec<usize> = vec![];
    for col in 0..grid[0].len() {
        let mut has_galaxy = false;
        for row in 0..grid.len() {
            if grid[row][col] == '#' {
                has_galaxy = true;
            }
        }
        if !has_galaxy {
            empty_columns.insert(empty_columns.len(), col);
        }
    }
    return empty_columns;
}

fn get_galaxy_coordinates(
    grid: &Vec<Vec<char>>,
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, chr) in row.iter().enumerate() {
            if *chr == '#' {
                points.insert(
                    points.len(),
                    get_expanded_coordinates(column_index, row_index, empty_rows, empty_columns),
                );
            }
        }
    }

    return points;
}

fn get_expanded_coordinates(
    x: usize,
    y: usize,
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
) -> Point {
    let empty_col_count: Vec<usize> = empty_columns
        .iter()
        .filter(|&&index| index < x)
        .map(|f| *f)
        .collect();

    let empty_row_count: Vec<usize> = empty_rows
        .iter()
        .filter(|&&index| index < y)
        .map(|f| *f)
        .collect();

    Point::new(
        (x + empty_col_count.len()).try_into().unwrap(),
        (y + empty_row_count.len()).try_into().unwrap(),
    )
}

fn calculate_distance(point_a: &Point, point_b: &Point) -> u64 {
    let x_diff: i64 = point_b.x as i64 - point_a.x as i64;
    let y_diff: i64 = point_b.y as i64 - point_a.y as i64;
    x_diff.abs() as u64 + y_diff.abs() as u64
}
