use crate::day09::{Iss, StopReason, Value};

use std::collections::HashMap;
use std::iter;

#[derive(Debug)]
enum Error {
    UnexpectedIssResult,
}

fn check(
    iss: &Iss,
    p: (Value, Value),
    cache: &mut HashMap<(Value, Value), Value>,
) -> crate::Result<Value> {
    if let Some(v) = cache.get(&p) {
        Ok(*v)
    } else {
        let mut iss = iss.clone();
        iss.feed_input(p.0);
        iss.feed_input(p.1);

        if let StopReason::Output(o) = iss.run()? {
            cache.insert(p, o);
            Ok(o)
        } else {
            Err(crate::Error::boxed(Error::UnexpectedIssResult))
        }
    }
}

pub fn part1(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let iss = Iss::new(mem);
    let mut cache = HashMap::new();

    (0..50)
        .flat_map(|x| (0..50).zip(iter::repeat(x)))
        .map(|p| check(&iss, p, &mut cache))
        .try_fold(0, |acc, r| Ok(acc + r?))
}

pub fn part2(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let iss = Iss::new(mem);
    let mut cache = HashMap::new();

    let mut check =
        |p| -> crate::Result<bool> { Ok(check(&iss, p, &mut cache)? == 1) };

    for y in 99.. {
        for x in 99..=y {
            if check((x, y))?
                && check((x - 99, y))?
                && check((x, y - 99))?
                && check((x - 99, y - 99))?
            {
                return Ok((x - 99) * 10_000 + (y - 99));
            }
        }
    }

    unreachable!();
}
