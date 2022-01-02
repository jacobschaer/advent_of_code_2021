use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut counts : [u32; 12] = [0; 12];
    let mut num_lines : u32 = 0;
    for line in reader.lines() {
        for (index, character) in line.unwrap().chars().enumerate() {
            match character {
                '0' => (),
                '1' => counts[index] += 1,
                _ => panic!("Bad input")
            }
            //println!("{} {}", index, character);
        }
        num_lines += 1;
    }

    println!("{:?}", counts);

    let half = num_lines / 2;
    let mut epsilon = 0;
    //println!("Half: {}", half);
    for (index, value) in counts.iter().enumerate() {
        if *value == half {
            panic!("Bad condition");
        }
        let bit = if *value > half {1} else {0};
        println!("Bit {}: {}", index, bit);
        epsilon |= bit << (counts.len() - (index + 1));
    }
    let gamma : u32 = !epsilon & 0b111111111111;
    println!("Epsilon: {} ({:b}), Gamma : {} ({:b}) => {}", epsilon, epsilon, gamma, gamma, epsilon * gamma);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut values = Vec::new();

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        values.push(isize::from_str_radix(&unwrapped, 2).unwrap());
    }

    let bit_width = 12;
    let mut results : [u32; 2] = [0; 2];

    for result_index in [0,1] {
        let mut filtered = values.clone();

        for index in 0..bit_width {
            let mut count = 0;
            let mut length = 0;
            // Get updated frequencies
            for value in filtered.iter() {
                if (value & 1 << (bit_width - (index + 1))) > 0 {
                    count += 1;
                }
                length += 1;
            }

            let half = (length + 1) / 2;
            println!("Half: {}, Count: {}", half, count);

            let mut bit = 0;
            if result_index == 0 {
                bit = if count >= half {0} else {1};
            }
            else {
                bit = if count >= half {1} else {0};
            }

            println!("Bit criteria: {} at {}", bit, index);
            filtered = filtered.iter()
                               .filter(|val| (*val >> (bit_width - (index + 1)) & 1) == bit)
                               .cloned()
                               .collect();
            for a in filtered.iter() {
                println!("{:b}", a);
            }
            if filtered.len() == 1 {
                results[result_index] = filtered[0] as u32;
                break;
            }
        }
    }
    println!("{:?}, {}", results, results[0] * results[1]);
    // let mut filtered = values.clone();
    // for (index, value) in counts.iter().enumerate() {
    //     let bit = if *value >= half {0} else {1};
    //     filtered = filtered.iter()
    //                        .filter(|val| (*val >> (counts.len() - (index + 1)) & 1) == bit)
    //                        .cloned()
    //                        .collect();
    //     //println!("Filtered: {:?}", filtered);
    //     if filtered.len() == 1 {
    //         co2 = filtered[0];
    //         println!("CO2 Rating: {} ({:b})", co2, co2);
    //         break;
    //     }
    // }
    // println!("Total: {}", o2 * co2);
        

    Ok(())
}

fn main() -> io::Result<()>{
    //part1()?;
    return part2()
}