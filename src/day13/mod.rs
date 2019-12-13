use crate::day09::{Iss, Value};

use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    PaddleNotFound,
    BallNotFound,
    UnexpectedOutput,
}

type Pos = (Value, Value);
type Grid = HashMap<Pos, Value>;

enum StopReason {
    Output(Value, Value, Value),
    OutOfInput,
    Halted,
}

fn run(iss: &mut Iss) -> crate::Result<StopReason> {
    use crate::day09::StopReason::*;
    match (iss.run()?, iss.run()?, iss.run()?) {
        (Output(x), Output(y), Output(v)) => Ok(StopReason::Output(x, y, v)),
        (Halted, _, _) => Ok(StopReason::Halted),
        (OutOfInput, _, _) => Ok(StopReason::OutOfInput),
        _ => Err(crate::Error::boxed(Error::UnexpectedOutput)),
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let mut iss = Iss::new(mem);

    let mut g = Grid::new();
    while let StopReason::Output(x, y, v) = run(&mut iss)? {
        g.insert((x, y), v);
    }

    Ok(g.values().filter(|&&v| v == 2).count())
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let mut iss = Iss::new(mem);
    *iss.access(0) = 2;

    let mut grid = Grid::new();
    let mut score = 0;
    loop {
        match run(&mut iss)? {
            StopReason::Output(-1, 0, s) => score = s,
            StopReason::Output(x, y, v) => {
                grid.insert((x, y), v);
            }
            StopReason::OutOfInput => {
                let paddle = grid
                    .iter()
                    .find(|(_, &v)| v == 3)
                    .map(|((x, _), _)| x)
                    .ok_or_else(|| {
                        crate::Error::boxed(Error::PaddleNotFound)
                    })?;
                let ball = grid
                    .iter()
                    .find(|(_, &v)| v == 4)
                    .map(|((x, _), _)| x)
                    .ok_or_else(|| crate::Error::boxed(Error::BallNotFound))?;

                iss.feed_input(if paddle < ball {
                    1
                } else if paddle > ball {
                    -1
                } else {
                    0
                });
            }
            StopReason::Halted => break,
        }
    }

    Ok(score)
}
