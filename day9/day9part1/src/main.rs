use std::{fs, io::Error};

fn main() {
    let lines = build_input();
    let mut total = 0;
    for mut line in lines.unwrap() {
        line.reverse();
        total += extrapolate(line);
    }
    println!("{:#?}", total);
}

fn build_input() -> Result<Vec<Vec<i32>>, Error>{
    let lines = fs::read_to_string("./inputs/input.txt")?;
    let mut total_results: Vec<Vec<i32>> = vec![];
    for line in lines.split("\r\n").into_iter() {
        let line_items: Vec<i32> = line.split(" ").map(|p| { p.parse().unwrap() }).collect();
        total_results.insert(total_results.len(), line_items);
    }
    Ok(total_results)
}

fn extrapolate(row: Vec<i32>) -> i32 {
    if row.iter().all(|f| *f == 0) {
        0
    } else {
        let differences = row.windows(2).map(|f| f.get(1).unwrap() - f.get(0).unwrap()).collect();
        return row.last().unwrap() + extrapolate(differences);
    }
}