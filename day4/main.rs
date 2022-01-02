use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut drawn = Vec::new();
    let mut num_lines : u32 = 0;
    let mut boards = Vec::new();
    let new_board : [[(u32, bool); DIMENSION]; DIMENSION] = [[(0, false); DIMENSION]; DIMENSION];
    boards.push(new_board);
    const DIMENSION : usize = 5;
    let mut row : usize = 0;
    let mut column  : usize = 0;
    let mut board_index : usize = 0;

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        //println!("{}", unwrapped);
        if num_lines == 0 {
            for character in unwrapped.split(",") {
                let value : u32 = character.parse().unwrap();
                println!("{}", value);
                drawn.push(value);
            }
        }
        else {
            for character in unwrapped.split_whitespace() {
                let value : u32 = character.parse().unwrap();
                if column >= DIMENSION {
                    row += 1;
                    column = 0;
                }
                if row >= DIMENSION {
                    row = 0;
                    column = 0;
                    board_index += 1;
                    boards.push(new_board);
                }
                boards[board_index][row][column] = (value, false);
                column += 1;
            }
        }
        num_lines += 1;
    }
    //println!("{:?}", boards);

    for called in drawn {
        println!("Drawn: {}", called);
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for cell in row {
                    if (*cell).0 == called {
                        (*cell).1 = true;
                    }
                }
            }
        }
        let mut winner = false;
        for (board_num, board) in boards.iter().enumerate() {
            for i in 0..DIMENSION {
                if board[i].iter().all(|x| x.1) {
                    println!("Winner. Board {}, Row {}. Called {}", board_num, i, called);
                    winner = true;
                }
                if board.iter().map(|x| x[i]).all(|x| x.1) {
                    println!("Winner. Board {}, Column {}. Called {}", board_num, i, called);
                    winner = true;
                }
                if winner {
                    break;
                }
            }
            if winner {
                println!("{:?}", board);
                let sum = board.iter().flatten().filter(|x| !x.1).map(|x| x.0).sum::<u32>();
                println!("Sum: {}. Product: {}", sum, sum * called);
            }
        }
        if winner {
            break;
        }
    }
    //println!("{:?}", boards[0]);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut drawn = Vec::new();
    let mut num_lines : u32 = 0;
    let mut boards = Vec::new();
    let new_board : [[(u32, bool); DIMENSION]; DIMENSION] = [[(0, false); DIMENSION]; DIMENSION];
    boards.push(new_board);
    const DIMENSION : usize = 5;
    let mut row : usize = 0;
    let mut column  : usize = 0;
    let mut board_index : usize = 0;
    let mut winners = Vec::new();

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        //println!("{}", unwrapped);
        if num_lines == 0 {
            for character in unwrapped.split(",") {
                let value : u32 = character.parse().unwrap();
                println!("{}", value);
                drawn.push(value);
            }
        }
        else {
            for character in unwrapped.split_whitespace() {
                let value : u32 = character.parse().unwrap();
                if column >= DIMENSION {
                    row += 1;
                    column = 0;
                }
                if row >= DIMENSION {
                    row = 0;
                    column = 0;
                    board_index += 1;
                    boards.push(new_board);
                }
                boards[board_index][row][column] = (value, false);
                column += 1;
            }
        }
        num_lines += 1;
    }
    //println!("{:?}", boards);

    for _ in 0..boards.len() {
        winners.push(false);
    }

    for called in drawn {
        println!("Drawn: {}", called);
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for cell in row {
                    if (*cell).0 == called {
                        (*cell).1 = true;
                    }
                }
            }
        }
        for (board_num, board) in boards.iter().enumerate() {
            if winners[board_num] {
                continue;
            }
            for i in 0..DIMENSION {
                if board[i].iter().all(|x| x.1) {
                    println!("Winner. Board {}, Row {}. Called {}", board_num, i, called);
                    winners[board_num] = true;
                }
                if board.iter().map(|x| x[i]).all(|x| x.1) {
                    println!("Winner. Board {}, Column {}. Called {}", board_num, i, called);
                    winners[board_num] = true;
                }
                if winners[board_num] {
                    break;
                }
            }
            if winners[board_num] {
                println!("{:?}", board);
                let sum = board.iter().flatten().filter(|x| !x.1).map(|x| x.0).sum::<u32>();
                println!("Sum: {}. Product: {}", sum, sum * called);
            }
        }
    }
    //println!("{:?}", boards[0]);

    Ok(())
}

fn main() -> io::Result<()>{
    //part1()?;
    return part2()
}