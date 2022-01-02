use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut fishies = Vec::<u8>::new();

    for line in reader.lines() {
        let unwrapped = line.unwrap();

        for life in unwrapped.split(",") {
            fishies.push(life.parse().unwrap());
        }
    }

    for day in 1..=18 {
        let mut to_add = 0;
//        println!("After {} day: {:?}", day, fishies);
        for fish in fishies.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                to_add += 1;
            }
            else {
                *fish -= 1;
            }
        }
        fishies.resize_with(fishies.len() + to_add, || { 8 });
    }
    println!("There were: {} fishies", fishies.len());

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut fish_lives : [u64; 9] = [0; 9];

    for line in reader.lines() {
        let unwrapped = line.unwrap();

        for life in unwrapped.split(",") {
            let life : usize = life.parse().unwrap();
            fish_lives[life] += 1;
        }
    }

    for day in 1..=256 {
        let new_fish = fish_lives[0];
        fish_lives[0] = 0;
        for life_count in 0..=7 {
            let temp = fish_lives[life_count];
            fish_lives[life_count] = fish_lives[life_count + 1];
            fish_lives[life_count + 1] = temp;
        }
        fish_lives[6] += new_fish;
        fish_lives[8] += new_fish;
    }
    println!("{:?}.\n Sum: {}", fish_lives, fish_lives.iter().sum::<u64>());

    Ok(())
}


fn main() -> io::Result<()>{
    part1()?;
    return part2();
}