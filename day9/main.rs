use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut values = Vec::<Vec<u8>>::new();

    for line in reader.lines() {
        values.push(Vec::new());
        let unwrapped = line.unwrap();

        for char in unwrapped.chars() {
            values.last_mut().unwrap().push(char.to_digit(10).unwrap() as u8);
        }
    }

    let mut risk : u64 = 0;
    for (row_index, row) in values.iter().enumerate() {
        println!("Row {}: {:?}", row_index, row);
        for (column_index, height) in row.iter().enumerate() {
            if row_index > 0 && values[row_index - 1][column_index] <= *height {
                continue;
            }
            if column_index > 0 && values[row_index][column_index - 1] <= *height {
                continue;
            }
            if row_index < values.len() - 1 && values[row_index + 1][column_index] <= *height {
                continue;
            }
            if column_index < row.len() - 1 && values[row_index][column_index + 1] <= *height {
                continue;
            }
            println!("Low point: ({}, {}) => {}", row_index, column_index, *height);
            risk += *height as u64 + 1;
        }
    }

    println!("Risk: {}", risk);
    Ok(())
}

fn visit(table : & mut Vec::<Vec<(u32, bool)>>, row_index : usize, column_index : usize) -> u64 {
    if row_index >= table.len() {
        return 0;
    }
    if column_index >= table[row_index].len() {
        return 0;
    }
    let mut count = 0;
    if !table[row_index][column_index].1 {
        count = 1;
        table[row_index][column_index].1 = true;

        if row_index > 0 {
            count += visit(table, row_index - 1, column_index);
        }
        if column_index > 0 {
            count += visit(table, row_index, column_index - 1);
        }
        count += visit(table, row_index + 1, column_index);
        count += visit(table, row_index, column_index + 1);
    }
    count
}

fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut values = Vec::<Vec<(u32, bool)>>::new();
    let mut islands = Vec::<u64>::new();

    for line in reader.lines() {
        values.push(Vec::new());
        let unwrapped = line.unwrap();

        for char in unwrapped.chars() {
            let height = char.to_digit(10).unwrap();
            let flag = if height == 9 { true } else { false };
            values.last_mut().unwrap().push((height, flag));
        }
    }

    for row_index in 0..values.len() {
        for column_index in 0..values[row_index].len() {
            let island = visit(&mut values, row_index, column_index);
            if island > 0 {
                islands.push(island);
            }
        }
    }

    // Could use a max-heap or other structure to save the sorting step
    islands.sort();

    println!("{:?}", islands.iter().rev().take(3).product::<u64>());

    Ok(())
}

fn main() -> io::Result<()>{
    part1()?;
    return part2();
}