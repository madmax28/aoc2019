use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::convert::{From, TryFrom};

#[derive(Debug)]
enum Error {
    InvalidInput,
    NotEnoughTargets,
}

type Point = (i32, i32);

fn euclid(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        euclid(b, a % b)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Facing {
    x: i32,
    y: i32,
}

impl From<(Point, Point)> for Facing {
    fn from((p1, p2): (Point, Point)) -> Self {
        let p = ((p2.0 - p1.0), (p2.1 - p1.1));
        let gcd = euclid(p.0.abs(), p.1.abs());
        Facing {
            x: p.0 / gcd,
            y: p.1 / gcd,
        }
    }
}

impl Ord for Facing {
    fn cmp(&self, other: &Self) -> Ordering {
        let (a1, a2) = (
            f64::from(self.x).atan2(self.y.into()),
            f64::from(other.x).atan2(other.y.into()),
        );
        a2.partial_cmp(&a1).unwrap()
    }
}

impl PartialOrd for Facing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn gen_asteroids(input: &str) -> crate::Result<Vec<Point>> {
    let mut asts = Vec::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                asts.push((i32::try_from(x)?, i32::try_from(y)?));
            }
        }
    }
    Ok(asts)
}

fn count_observable_from(p: Point, asts: &[Point]) -> usize {
    asts.iter()
        .filter(|&&p_o| p_o != p)
        .fold(HashSet::new(), |mut acc, &p_o| {
            acc.insert(Facing::from((p, p_o)));
            acc
        })
        .len()
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let asteroids = gen_asteroids(input)?;
    Ok(asteroids
        .iter()
        .map(|&p| count_observable_from(p, &asteroids))
        .max()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mut asteroids = gen_asteroids(input)?;
    let station_loc = *asteroids
        .iter()
        .max_by_key(|&p| count_observable_from(*p, &asteroids))
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;

    let mut target_iter = {
        asteroids.sort_by_key(|p| {
            (p.0 - station_loc.0).abs() + (p.1 - station_loc.1).abs()
        });

        let mut facing_target_count = HashMap::new();
        let mut targets = asteroids
            .into_iter()
            .filter(|&p| p != station_loc)
            .map(|p| {
                let f = Facing::from((station_loc, p));
                let n = *facing_target_count
                    .entry(f)
                    .and_modify(|c| *c += 1)
                    .or_insert(0);
                (n, f, p)
            })
            .collect::<Vec<_>>();
        targets.sort();
        targets.into_iter().map(|(.., p)| p)
    };

    let t = target_iter
        .nth(199)
        .ok_or_else(|| crate::Error::boxed(Error::NotEnoughTargets))?;

    Ok(t.0 * 100 + t.1)
}
