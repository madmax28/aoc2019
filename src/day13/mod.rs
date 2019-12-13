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

fn run_till_output(
    iss: &mut Iss,
) -> crate::Result<Option<(Value, Value, Value)>> {
    match (
        iss.run_till_output(),
        iss.run_till_output(),
        iss.run_till_output(),
    ) {
        (Ok(Some(x)), Ok(Some(y)), Ok(Some(v))) => Ok(Some((x, y, v))),
        (_, _, Ok(None)) => Ok(None),
        (_, _, Err(e)) => Err(e),
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
    while let Ok(Some((x, y, v))) = run_till_output(&mut iss) {
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
        match run_till_output(&mut iss) {
            Ok(Some((-1, 0, s))) => score = s,
            Ok(Some((x, y, v))) => {
                grid.insert((x, y), v);
            }
            Ok(None) => break,
            Err(e) => {
                if e.is::<crate::Error<Error>>() {
                    return Err(e);
                }

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
        }
    }

    Ok(score)
}
