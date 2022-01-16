use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

fn part1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    const SIZE: usize = 10;
    let mut risk = [[0; SIZE]; SIZE];
    let mut total_risk = [[0 as u64; SIZE]; SIZE];

    for (row, line) in reader.lines().enumerate() {
        for (column, value) in line.unwrap().chars().enumerate() {
            risk[row][column] = value.to_digit(10).unwrap();
        }
    }

    for row in 0..SIZE {
        for column in 0..SIZE {
            let mut from_top = std::u64::MAX;
            let mut from_left = std::u64::MAX;

            if row == 0 && column == 0 {
                total_risk[row][column] = 0;
                continue;
            }
            else {
                if row > 0 {
                    from_top = total_risk[row - 1][column];
                }
                if column > 0 {
                    from_left = total_risk[row][column - 1];
                }
            }
            total_risk[row][column] = cmp::min(from_top, from_left) + risk[row][column] as u64;
        }
    }

    println!("{:?}", total_risk);
    println!("Total: {}", total_risk[SIZE -1][SIZE-1]);

    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    cost: u64
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn part2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    const INPUT_SIZE: usize = 100;
    const SCALER: usize = 5;
    const RISK_SIZE : usize = INPUT_SIZE * SCALER;
    let mut risk = vec![vec![0; RISK_SIZE]; RISK_SIZE];
    let mut total_risk = vec![vec![std::u64::MAX; RISK_SIZE]; RISK_SIZE];

    // Build out table
    for (row, line) in reader.lines().enumerate() {
        risk[row].resize(RISK_SIZE, 0);
        total_risk[row].resize(RISK_SIZE, std::u64::MAX);

        for (column, value) in line.unwrap().chars().enumerate() {
            risk[row][column] = value.to_digit(10).unwrap() as u8;

            for row_offset in 0..SCALER {
                 for column_offset in 0..SCALER {
                    let current = (row + (INPUT_SIZE * row_offset), column + (INPUT_SIZE * column_offset));
                    if row_offset == 0 && column_offset > 0 {
                        // Immediately to the left
                        let left = (row + (INPUT_SIZE * row_offset), column + (INPUT_SIZE * (column_offset - 1)));
                        let value = risk[left.0][left.1] + 1;
                        if value >= 10 {
                            risk[current.0][current.1] = 1;
                        } else {
                            risk[current.0][current.1] = value;
                        }
                    }
                    else if row_offset > 0 {
                        // Immediately above
                        let above = (row + (INPUT_SIZE * (row_offset - 1)), column + (INPUT_SIZE * (column_offset)));
                        let value = risk[above.0][above.1] + 1;
                        if value >= 10 {
                            risk[current.0][current.1] = 1;
                        } else {
                            risk[current.0][current.1] = value;
                        }
                    }
                }
            }
        }
    }


    // Modified djikstra
    let mut heap = BinaryHeap::new();

    total_risk[0][0] = 0;

    heap.push(Node { cost: 0, position: (0,0) });

    let mut result : u64 = 0;

    while let Some(Node { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == (RISK_SIZE - 1, RISK_SIZE - 1) { result = cost; break; }

        // Important as we may have already found a better way
        if cost > total_risk[position.0][position.1] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node

        // Technically should only consider nodes that are *in* the graph - this will
        // re-add nodes that are "visited", though they will always be < so there's
        // no harm other than wasted CPU time. Could have an array of bools marking
        // visited and test
        let mut adjacent = Vec::new();
        if position.0 > 0 {
            adjacent.push((position.0 - 1, position.1));
        }
        if position.1 > 0 {
            adjacent.push((position.0, position.1 - 1));
        }
        if position.0 < RISK_SIZE - 1 {
            adjacent.push((position.0 + 1, position.1));
        }
        if position.1 < RISK_SIZE - 1 {
            adjacent.push((position.0, position.1 + 1));
        }

        for candidate in adjacent {
            let next = Node { cost: risk[candidate.0][candidate.1] as u64 + cost, position: candidate };

            // If so, add it to the frontier and continue
            if next.cost < total_risk[candidate.0][candidate.1] {
                heap.push(next);
                // Relaxation, we have now found a better way
                total_risk[candidate.0][candidate.1] = next.cost;
            }
        }
    }

    println!("Result: {}", result);
    Ok(())
}

fn main() -> io::Result<()> {
    //part1()?;
    part2()
}
