use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
enum Error {
    InvalidInput,
    NoIntersection,
}

type Point = (i32, i32);
type Wire = Vec<Point>;

fn create_wire(line: &str) -> crate::Result<Wire> {
    Ok(line
        .split(',')
        .try_fold(
            ((0, 0), Wire::new()),
            |(mut p, mut w), s| -> crate::Result<(Point, Wire)> {
                let dir = s
                    .chars()
                    .nth(0)
                    .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
                let count = s
                    .get(1..)
                    .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                    .parse()?;

                for _ in 0..count {
                    match dir {
                        'U' => p.1 += 1,
                        'D' => p.1 -= 1,
                        'L' => p.0 -= 1,
                        'R' => p.0 += 1,
                        _ => {
                            return Err(crate::Error::boxed(
                                Error::InvalidInput,
                            ))
                        }
                    }

                    w.push(p);
                }

                Ok((p, w))
            },
        )?
        .1)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mut lines = input.lines();

    let wire1 = create_wire(
        lines
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?,
    )?;
    let wire2 = create_wire(
        lines
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?,
    )?;

    let points1 = HashSet::<Point>::from_iter(wire1.into_iter());
    let points2 = HashSet::<Point>::from_iter(wire2.into_iter());

    Ok(points1
        .intersection(&points2)
        .map(|p| p.0.abs() + p.1.abs())
        .min()
        .ok_or_else(|| crate::Error::boxed(Error::NoIntersection))?)
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let mut lines = input.lines();

    let wire1 = create_wire(
        lines
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?,
    )?;
    let wire2 = create_wire(
        lines
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?,
    )?;

    let points1 = HashSet::<Point>::from_iter(wire1.iter().cloned());
    let points2 = HashSet::<Point>::from_iter(wire2.iter().cloned());

    Ok(points1
        .intersection(&points2)
        .map(|i| {
            wire1.iter().take_while(|p| *p != i).count()
                + wire2.iter().take_while(|p| *p != i).count()
                + 2
        })
        .min()
        .ok_or_else(|| crate::Error::boxed(Error::NoIntersection))?)
}
