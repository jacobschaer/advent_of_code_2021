use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const BRACES : [(char, char, u16, u8); 4] = [
    ('(', ')', 3, 1),
    ('[', ']', 57, 2),
    ('{', '}', 1197, 3),
    ('<', '>', 25137, 4),
];

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut score : u64 = 0;

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let mut characters = VecDeque::new();

        let mut line_score : u16 = 0;
        for candidate in unwrapped.chars() {
            for (opening, closing, value, _) in BRACES {
                if candidate == opening {
                    characters.push_back(candidate);
                }
                else if candidate == closing {
                    match characters.pop_back() {
                        Some(x) if x != opening => {
                            println!("Mismatch: Got: {} Expected match for {}. Score {}", candidate, x, value);
                            line_score += value;    
                            break;
                        },
                        None => break,
                        _ => ()
                    }
                }
            }
            if line_score > 0 {
                break;
            }
        }
        score += line_score as u64;
    }
    println!("Score: {}", score);
    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut scores = Vec::new();

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let mut characters = VecDeque::new();

        let mut line_score : u64 = 0;
        let mut bad : bool = false;
        for candidate in unwrapped.chars() {
            for (opening, closing, _, _) in BRACES {
                if candidate == opening {
                    characters.push_back(candidate);
                }
                else if candidate == closing {
                    match characters.pop_back() {
                        Some(x) if x != opening => {
                            bad = true;
                            break;
                        },
                        None => break,
                        _ => ()
                    }
                }
            }
            if bad {
                break;
            }
        }
        if !bad {
            while !characters.is_empty() {
                match characters.pop_back() {
                    Some(x) => {
                        for (opening, closing, _, value) in BRACES {
                            if x == opening {
                                print!("{}", closing);
                                line_score *= 5 as u64;
                                line_score += value as u64;
                            }
                        }
                    },
                    None => ()
                }
            }
            scores.push(line_score);
            print!(" - {} total points.", line_score);
            println!("");
        }
    }

    scores.sort();
    println!("Median: {}", scores[(scores.len()) / 2]);
    Ok(())
}

fn main() -> io::Result<()> {
    part1()?;
    return part2();
}
