use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const SIZE : usize = 10;

fn handle_flash(
    i : usize,
    j : usize, 
    grid : &mut [[u8; SIZE]; SIZE],
    flashed : &mut [[bool; SIZE]; SIZE]) -> u64 {
    let mut num_flashed : u64 = 0;

    if grid[i][j] > 9 && !flashed[i][j] {
        num_flashed += 1;
        grid[i][j] = 0;
        flashed[i][j] = true;

        for k in 0..3 {
            for l in 0..3 {
                let row : i8 = i as i8 - 1 + k as i8;
                let column : i8 = j as i8 - 1 + l as i8;
                if 0 <= row && row < grid.len() as i8 &&
                   0 <= column && column < grid[row as usize].len() as i8 {
                    if !(row as usize == i && column as usize == j) {
                        grid[row as usize][column as usize] += 1;
                        num_flashed += handle_flash(row as usize, column as usize, grid, flashed);
                    }
                }
            }
        }
    }
    num_flashed
    //println!("{} {}", grid[i][j], flashed[i][j]);
}

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut grid: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    for (i, line) in reader.lines().enumerate() {
        let unwrapped = line.unwrap();

        for (j, character) in unwrapped.chars().enumerate() {
            grid[i][j] = character.to_digit(10).unwrap() as u8;
        }
    }

    let mut total_flashes : u64 = 0;

    for day in 1..=100 {
        // Phase 1: Increase all by 1
        for i in 0..SIZE {
            for j in 0..SIZE {
                grid[i][j] += 1;
            }
        }

        // Phase 2: Handle flashes
        let mut flashed : [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                total_flashes += handle_flash(i,j,&mut grid,&mut flashed);
            }
        }

        // Phae 3: Wipe Flashed
        for i in 0..SIZE {
            for j in 0..SIZE {
                if flashed[i][j] {
                    grid[i][j] = 0;
                }
            }
        }

        println!("After step {} ({} flashes):", day, total_flashes);
        for row in grid {
            println!("{:?}", row);
        }

    }

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut grid: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    for (i, line) in reader.lines().enumerate() {
        let unwrapped = line.unwrap();

        for (j, character) in unwrapped.chars().enumerate() {
            grid[i][j] = character.to_digit(10).unwrap() as u8;
        }
    }

    let mut day : u32 = 0;
    loop {
        let mut total_flashes : u8 = 0;

        day += 1;
        // Phase 1: Increase all by 1
        for i in 0..SIZE {
            for j in 0..SIZE {
                grid[i][j] += 1;
            }
        }

        // Phase 2: Handle flashes
        let mut flashed : [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                total_flashes += handle_flash(i,j,&mut grid,&mut flashed) as u8;
            }
        }

        // Phae 3: Wipe Flashed
        for i in 0..SIZE {
            for j in 0..SIZE {
                if flashed[i][j] {
                    grid[i][j] = 0;
                }
            }
        }

        println!("After step {} ({} flashes):", day, total_flashes);
        for row in grid {
            println!("{:?}", row);
        }

        if total_flashes == SIZE as u8 * SIZE as u8 {
            println!("Day: {}", day);
            break;
        }
    }

    Ok(())
}


fn main() -> io::Result<()> {
    part1()?;
    return part2();
}
