use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut points = Vec::<(usize, usize)>::new();
    let mut folds = Vec::<(char, usize)>::new();
    let mut width = 0;
    let mut height = 0;

    // Parse the awful file format
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let mut point : [&str; 2] = [""; 2];

        let mut type_1_count = 0;
        for value in unwrapped.split(',') {
            point[type_1_count] = value;
            type_1_count += 1;
        }

        if type_1_count == 2 {
            points.push((point[0].parse::<usize>().unwrap(), point[1].parse::<usize>().unwrap()))
        }
        else if type_1_count == 1 {
            let mut fold : [&str; 2] = [""; 2];
            let mut type_2_count = 0;

            for subvalue in unwrapped.split("=") {
                fold[type_2_count] = subvalue;
                type_2_count += 1;
            }

            if type_2_count == 2 {
                folds.push((fold[0].chars().nth(11).unwrap(), fold[1].parse::<usize>().unwrap()));
            }
        }
    }

    // Get dimensions
    for (x,y) in points.iter() {
        if x + 1 >= width {
            width = x + 1;
        }
        if y + 1 >= height {
            height = y + 1;
        }
    }

    let mut paper : Vec::<Vec::<bool>> = Vec::new() ;
    for row in 0..height {
        let mut row_vec = Vec::<bool>::new();
        row_vec.resize(width, false);
        paper.push(row_vec);
    }

    for (x,y) in points.iter() {
        if x + 1 >= width {
            width = x + 1;
        }
        if y + 1 >= height {
            height = y + 1;
        }
    }

    for (x,y) in points.iter() {
        paper[*y][*x] = true;
    }

    for (fold_axis, fold_index) in folds.iter() {
        println!("{} {}", fold_axis, fold_index);
        if *fold_axis == 'y' {
            for row_offset in 1..=*fold_index {
                for column_idx in 0..width {
                    if *fold_index >= row_offset && (fold_index + row_offset) <= height {
                        println!("Swapping ({}, {}), with ({}, {})", *fold_index - row_offset, column_idx, row_offset + *fold_index, column_idx);
                        paper[*fold_index - row_offset][column_idx] |= paper[row_offset + *fold_index][column_idx];
                    }
                }
            }
            height = *fold_index;
        }
        else {
            for column_offset in 1..=*fold_index {
                for row_index in 0..height {
                    if *fold_index >= column_offset && (fold_index + column_offset) <= width {
                        println!("Swapping ({}, {}), with ({}, {})", row_index, *fold_index - column_offset, row_index, *fold_index + column_offset);
                        paper[row_index][*fold_index - column_offset] |= paper[row_index][*fold_index + column_offset];
                    }
                }
            }
            width = *fold_index; 
        }
    }

    println!("{:?}, {:?}, {}, {}", points, folds, width, height);
    
    let mut num_visible : usize = 0;
    for row in 0..height {
        println!("");
        for column in 0..width {
            if paper[row][column] {
                print!("{}", "#");
                num_visible += 1;
            } else {
                print!(".");
            }
        }
    } 

    println!("Num visible: {}", num_visible);

    Ok(())
}

fn main() -> io::Result<()> {
    part1()
}
