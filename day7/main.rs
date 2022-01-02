use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp;

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut positions = Vec::<u32>::new();
    let mut max = u32::MIN;
    let mut min = u32::MAX;

    for line in reader.lines() {
        let unwrapped = line.unwrap();

        for position in unwrapped.split(",") {
            let position_int = position.parse().unwrap();
            max = cmp::max(max, position_int);
            min = cmp::min(min, position_int);
            positions.push(position_int);
        }
    }

    let mut min_fuel = u64::MAX;
    let mut min_position = min;
    for candidate in min..=max {
        let mut fuel : u64 = 0;
        for position in positions.iter() {
            if *position >= candidate {
                fuel += *position as u64 - candidate as u64;
            } else {
                fuel += candidate as u64 - *position as u64;
            }
        }
        if fuel <= min_fuel {
            min_fuel = fuel;
            min_position = candidate;
        }
    }

    println!("Min fuel: {} at {}", min_fuel, min_position);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut positions = Vec::<u32>::new();
    let mut max = u32::MIN;
    let mut min = u32::MAX;

    for line in reader.lines() {
        let unwrapped = line.unwrap();

        for position in unwrapped.split(",") {
            let position_int = position.parse().unwrap();
            max = cmp::max(max, position_int);
            min = cmp::min(min, position_int);
            positions.push(position_int);
        }
    }

    let mut min_fuel = u64::MAX;
    let mut min_position = min;
    for candidate in min..=max {
        let mut fuel : u64 = 0;
        for position in positions.iter() {
            // if *position >= candidate {
            //     fuel += (1..=*position - candidate).map(|x| x as u64).sum::<u64>();
            // } else {
            //     fuel += (1..=candidate - *position).map(|x| x as u64).sum::<u64>();
            // }

            // The above can be optimized by using the formula for the sum of an arithmetic sequence
            // Assuming that most points are more than 4 away, the two sums, a product, and a division
            // is much faster. 
            if *position != candidate {
                let end_pos;
                const FIRST_STEP_COST : u32 = 1;
                if *position > candidate {
                    end_pos = *position - candidate;
                } else {
                    end_pos = candidate - *position; 
                };
                fuel += (1 + end_pos as u64 - FIRST_STEP_COST as u64) * (end_pos as u64 + FIRST_STEP_COST as u64) / 2 as u64;
            }
        }
        if fuel <= min_fuel {
            min_fuel = fuel;
            min_position = candidate;
        }
        //println!("Fuel: {} at {}", fuel, candidate);
    }

    println!("Min fuel: {} at {}", min_fuel, min_position);

    Ok(())
}

fn main() -> io::Result<()>{
    part1()?;
    return part2();
}