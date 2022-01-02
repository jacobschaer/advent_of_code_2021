use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut previous : Option<u32> = None;
    let mut count_increase : u32 = 0;
    for line in reader.lines() {
        let value : u32 = line?.parse().unwrap();
        count_increase += match previous {
            Some(x) => if value > x {  1 } else { 0 },
            None => 0 
        };
        previous = Some(value);
    }

    println!("Part 1 Total: {}", count_increase);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut queue : [u32; 4] = [0; 4];
    let mut running_total : u32 = 0;
    let mut read : u8 = 0;
    let mut count_increase : u32 = 0;
    for line in reader.lines() {
        // Shift the new value into the queue
        running_total -= queue[0];
        queue[0] = queue[1];
        queue[1] = queue[2];
        queue[2] = queue[3];
        queue[3] = line?.parse().unwrap();
        running_total += queue[3];

        if read >= 4 {
            if queue[0] < queue[3] {
                count_increase += 1;
            }
        } else {
            read += 1;
        }
    }

    println!("Part 2 Total: {}", count_increase);

    Ok(())
}

fn main() {
    part1();
    part2();
}