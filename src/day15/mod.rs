use crate::day09::{Iss, StopReason, Value};

use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
enum Error {
    InvalidDirection,
    NoPathFound,
    UnexpectedIssResult,
}

type Pos = (Value, Value);

fn step(p: Pos, d: Value) -> crate::Result<Pos> {
    match d {
        1 => Ok((p.0, p.1 + 1)),
        2 => Ok((p.0, p.1 - 1)),
        3 => Ok((p.0 - 1, p.1)),
        4 => Ok((p.0 + 1, p.1)),
        _ => Err(crate::Error::boxed(Error::InvalidDirection)),
    }
}

fn find_oxygen_system(iss: Iss) -> crate::Result<(Pos, Value, Iss)> {
    let mut candidates = VecDeque::new();
    candidates.push_back(((0, 0), 0, iss.clone()));
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    Ok('outer: loop {
        let (pos, dist, iss) = candidates
            .pop_front()
            .ok_or_else(|| crate::Error::boxed(Error::NoPathFound))?;

        for dir in 1..=4 {
            let mut iss = iss.clone();
            iss.feed_input(dir);

            match iss.run()? {
                StopReason::Output(0) => (),
                StopReason::Output(1) => {
                    let pos = step(pos, dir)?;

                    if visited.contains(&pos) {
                        continue;
                    }
                    visited.insert(pos);

                    candidates.push_back((pos, dist + 1, iss.clone()));
                }
                StopReason::Output(2) => break 'outer (step(pos, dir)?, dist + 1, iss),
                _ => {
                    return Err(crate::Error::boxed(Error::UnexpectedIssResult))
                }
            }
        }
    })
}

fn fill_oxygen(iss: Iss) -> crate::Result<(Pos, Value, Iss)> {
    let mut candidates = VecDeque::new();
    candidates.push_back(((0, 0), 0, iss.clone()));
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    Ok(loop {
        let (pos, dist, iss) = candidates
            .pop_front()
            .ok_or_else(|| crate::Error::boxed(Error::NoPathFound))?;

        for dir in 1..=4 {
            let mut iss = iss.clone();
            iss.feed_input(dir);

            match iss.run()? {
                StopReason::Output(0) => (),
                StopReason::Output(1) | StopReason::Output(2) => {
                    let pos = step(pos, dir)?;

                    if visited.contains(&pos) {
                        continue;
                    }
                    visited.insert(pos);

                    candidates.push_back((pos, dist + 1, iss.clone()));
                }
                _ => {
                    return Err(crate::Error::boxed(Error::UnexpectedIssResult))
                }
            }
        }

        if candidates.is_empty() {
            break (pos, dist, iss);
        }
    })
}

pub fn part1(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let iss = Iss::new(mem);

    let (_, dist, _) = find_oxygen_system(iss)?;

    Ok(dist)
}

pub fn part2(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let iss = Iss::new(mem);

    let (_, _, iss) = find_oxygen_system(iss)?;
    let (_, dist, _) = fill_oxygen(iss)?;

    Ok(dist)
}
