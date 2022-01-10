use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::{HashMap, HashSet};

fn walk1 (
    name: &str,
    map : &HashMap<String, HashSet<String>>,
    so_far : Vec::<&str>
) -> usize {
    let mut count : usize = 0;
    if name.to_lowercase() != name || so_far.iter().find(|&x| *x == name) == None {
        let mut result = so_far.clone();
        result.push(name);
        if name == "end" {
            println!("{:?}", result);
            count = 1;
        } else {
            for connected in map[name].iter() {
                count += walk1(&connected, map, result.clone());
            }
        }
    }
    count
}

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut pairs = Vec::<(String, String)>::new();
    let mut map = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let unwrapped = line.unwrap();
        let mut connection : [&str; 2] = [""; 2];

        for (j, name) in unwrapped.split('-').enumerate() {
            connection[j] = name;
        }

        pairs.push((String::from(connection[0]), String::from(connection[1])));

        map.entry(pairs[i].0.clone()).or_insert(HashSet::<String>::new()).insert(pairs[i].1.clone());
        map.entry(pairs[i].1.clone()).or_insert(HashSet::<String>::new()).insert(pairs[i].0.clone());
    }

    println!("{:?}", pairs);
    println!("{:?}", map);

    println!("There were {} paths" , walk1("start", &mut map, Vec::new()));

    Ok(())
}

fn walk2 (
    name: &str,
    map : &HashMap<String, HashSet<String>>,
    so_far : Vec::<&str>
) -> usize {
    let mut count : usize = 0;

    let mut can_be_added = false;
    let is_lower = name.to_lowercase() == name; 
    println!("{:?}", so_far);

    if is_lower {
        match so_far.iter().filter(|&x| *x == name).count() {
            0 => {
                //println!("{} is new. Adding", name);
                can_be_added = true;
            },
            1 => {
                if name == "start" {
                    //println!("{} can only exist once.", name);
                } else {
                   can_be_added = true;
                   for value in so_far.iter() {
                      if *value == value.to_lowercase() && so_far.iter().filter(|&x| x == value).count() > 1 {
                        //println!("{} can't be doubled because already has a double {}.", name, value);
                        can_be_added = false;
                        break;
                      }  
                    }
                }
            },
            _ => {
                //println!("{} can only exist at most twice.", name);
            },
        }
    } else {
        can_be_added = true;
    }

    if can_be_added {
        let mut result = so_far.clone();
        result.push(name);
        if name == "end" {
            println!("{:?}", result);
            count = 1;
        } else {
            for connected in map[name].iter() {
                count += walk2(&connected, map, result.clone());
            }
        }
    }
    count
}

fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut pairs = Vec::<(String, String)>::new();
    let mut map = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let unwrapped = line.unwrap();
        let mut connection : [&str; 2] = [""; 2];

        for (j, name) in unwrapped.split('-').enumerate() {
            connection[j] = name;
        }

        pairs.push((String::from(connection[0]), String::from(connection[1])));

        map.entry(pairs[i].0.clone()).or_insert(HashSet::<String>::new()).insert(pairs[i].1.clone());
        map.entry(pairs[i].1.clone()).or_insert(HashSet::<String>::new()).insert(pairs[i].0.clone());
    }

    println!("{:?}", pairs);
    println!("{:?}", map);

    println!("There were {} paths" , walk2("start", &mut map, Vec::new()));

    Ok(())
}

fn main() -> io::Result<()> {
    part1()?;
    part2()
}
