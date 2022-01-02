use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut depth : u32 = 0;
    let mut distance : u32 = 0;
    for line in reader.lines() {
        let actual_line = line.unwrap();
        let mut splitted = actual_line.split_whitespace();
        let direction = splitted.next().unwrap();
        let length : u32 = splitted.next().unwrap().parse().unwrap();
        match direction {
            "up" => depth -= length,
            "down" => depth += length,
            "forward" => distance += length,
            "backward" => distance -= length,
            _ => panic!("Bad input"),
        }
    }
    println!("Part 1: {} {} => {}", depth, distance, depth * distance);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut depth : u32 = 0;
    let mut distance : u32 = 0;
    let mut aim : u32 = 0;
    for line in reader.lines() {
        let actual_line = line.unwrap();
        let mut splitted = actual_line.split_whitespace();
        let direction = splitted.next().unwrap();
        let value : u32 = splitted.next().unwrap().parse().unwrap();
        match direction {
            "up" => aim -= value,
            "down" => aim += value,
            "forward" => {distance += value; depth += aim * value},
            _ => panic!("Bad input"),
        }
        println!("Aim: {}, Depth: {}, Distance: {}, Direction: {}, Value: {}", aim, depth, distance, direction, value);
    }
    println!("Part 2: {} {} => {}", depth, distance, depth * distance);

    Ok(())
}

fn main() -> io::Result<()>{
    part1()?;
    return part2();
}