use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut coords = HashMap::new();
    const X_INDEX : usize = 0;
    const Y_INDEX  : usize = 1;

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let mut coord = [[0, 0], [0, 0]];

        for (i, point) in unwrapped.split(" -> ").enumerate() {
            //println!("{}", point);
            for (j, val) in point.split(",").enumerate() {
                coord[i][j] = val.parse().unwrap();
            }
        }

        let mut start_index = 0;
        let mut end_index = 1;

        // May not necessarily be in increasing order, find which is "first"
        if coord[0][X_INDEX] == coord[1][X_INDEX] {
            if coord[start_index][Y_INDEX] >= coord[end_index][Y_INDEX] {
                start_index = 1;
                end_index = 0;
            }
        }
        else if coord[0][Y_INDEX] == coord[1][Y_INDEX] {
            if coord[start_index][X_INDEX] >= coord[end_index][X_INDEX] {
                start_index = 1;
                end_index = 0;
            }
        }
        else {
            continue;
        }
        //println!("Start {:?}, Finish: {:?}", coord[start_index], coord[end_index]);

        for x in coord[start_index][X_INDEX]..=coord[end_index][X_INDEX] {
            for y in coord[start_index][Y_INDEX]..=coord[end_index][Y_INDEX] {
                //println!("{:?}", (x,y));
                let ordinality = coords.entry((x,y)).or_insert(0);
                *ordinality += 1;
            }
        }
    }

    // Diagnostic Print
    // for y in 0..10 {
    //     for x in 0..10 {
    //         let coord = (x,y);
    //         let char = match coords.get(&coord) {
    //             None => String::from("."),
    //             Some(x) => x.to_string()
    //         };
    //         print!("{}", char);
    //     }
    //     println!("");
    // }

    // How many points where 2 overlap
    println!("{}", coords.iter().filter(|x| *x.1 > 1).count());

    Ok(())
}


fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut coords = HashMap::new();
    const X_INDEX : usize = 0;
    const Y_INDEX  : usize = 1;

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let mut coord = [[0, 0], [0, 0]];

        for (i, point) in unwrapped.split(" -> ").enumerate() {
            //println!("{}", point);
            for (j, val) in point.split(",").enumerate() {
                coord[i][j] = val.parse().unwrap();
            }
        }

        //println!("Start {:?}, Finish: {:?}", coord[0], coord[1]);

        let mut x = coord[0][X_INDEX];
        let mut y = coord[0][Y_INDEX];

        // Ideally, we'd want to pass the two for loops an iterator
        // either forward, or reverse. But you can't have negative steps,
        // and rev() is a different type so we'd need dynamic dispatch.
        // This is a clumsy solution with none of the fast bounds checking
        // of native ranges but will get-er-done
        loop {
            //println!("{}, {}", x, y);
            let ordinality = coords.entry((x,y)).or_insert(0);
            *ordinality += 1;

            if x == coord[1][X_INDEX] && y == coord[1][Y_INDEX] {
                break;
            }

            if y > coord[1][Y_INDEX] {
                y -= 1;
            } else if y < coord[1][Y_INDEX] {
                y += 1;
            }
            if x > coord[1][X_INDEX] {
                x -= 1;
            } else if x < coord[1][X_INDEX] {
                x += 1;
            }

        }
    }

    // Diagnostic Print
    // for y in 0..10 {
    //     for x in 0..10 {
    //         let coord = (x,y);
    //         let char = match coords.get(&coord) {
    //             None => String::from("."),
    //             Some(x) => x.to_string()
    //         };
    //         print!("{}", char);
    //     }
    //     println!("");
    // }

    // How many points where 2 overlap
    println!("{}", coords.iter().filter(|x| *x.1 > 1).count());

    Ok(())
}


fn main() -> io::Result<()>{
    return part2();
}