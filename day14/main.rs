use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::{HashMap};

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut polymer = Vec::<char>::new();
    let mut rules = HashMap::<(char, char), char>::new();
    let mut counts = HashMap::<char, usize>::new();

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            for character in line.unwrap().chars() {
                polymer.push(character);
                let count = counts.entry(character).or_insert(0);
                *count += 1;
            }
        } else if i > 1 {
            let unwrapped = line.unwrap();
            let a = unwrapped.split(" -> ").nth(0).unwrap().chars().nth(0).unwrap();
            let b = unwrapped.split(" -> ").nth(0).unwrap().chars().nth(1).unwrap();
            let c = unwrapped.split(" -> ").nth(1).unwrap().chars().nth(0).unwrap();
            rules.insert((a, b), c);
            counts.entry(c).or_insert(0);
        }
    }

    for step in 1..=40 {
        let mut index = 0;
        loop {
            let a = polymer[index];
            let b = polymer[index + 1];
            match rules.get(&(a,b)) {
                Some(x) => {polymer.insert(index + 1, *x); index += 1; *counts.get_mut(x).unwrap() += 1},
                None => ()
            }
            index += 1;
            if index >= polymer.len() - 1 {
                break;
            }
        }
        println!("Step: {}", step);
//        println!("After step {}, {}", step, polymer.iter().collect::<String>());
    }

    let mut values : Vec::<usize> = counts.into_values().collect();
    values.sort();
    println!("{:?}", values.last().unwrap() - values[0]);


    Ok(())
}

fn main() -> io::Result<()> {
    part1()
}
