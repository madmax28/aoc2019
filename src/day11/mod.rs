use crate::day09::{Iss, Value};

use std::cmp::{max, min};
use std::collections::HashMap;
use std::convert::{From, TryFrom, TryInto};

#[derive(Debug)]
enum Error {
    InvalidColor,
    InvalidTurn,
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White,
}

impl From<Color> for Value {
    fn from(c: Color) -> Self {
        match c {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl TryFrom<Value> for Color {
    type Error = Box<crate::Error<Error>>;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(crate::Error::boxed(Error::InvalidColor)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn turn(self, v: Value) -> crate::Result<Self> {
        match (self, v) {
            (Dir::Left, 0) => Ok(Dir::Down),
            (Dir::Right, 0) => Ok(Dir::Up),
            (Dir::Up, 0) => Ok(Dir::Left),
            (Dir::Down, 0) => Ok(Dir::Right),
            (Dir::Left, 1) => Ok(Dir::Up),
            (Dir::Right, 1) => Ok(Dir::Down),
            (Dir::Up, 1) => Ok(Dir::Right),
            (Dir::Down, 1) => Ok(Dir::Left),
            _ => Err(crate::Error::boxed(Error::InvalidTurn)),
        }
    }
}

type Pos = (i32, i32);
type Grid = HashMap<Pos, Color>;

fn step(p: Pos, d: Dir) -> Pos {
    match d {
        Dir::Up => (p.0, p.1 - 1),
        Dir::Down => (p.0, p.1 + 1),
        Dir::Left => (p.0 - 1, p.1),
        Dir::Right => (p.0 + 1, p.1),
    }
}

fn run(input: &str, grid: &mut Grid) -> crate::Result<()> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let mut iss = Iss::new(mem);

    let mut p = (0, 0);
    let mut d = Dir::Up;
    loop {
        let c = grid.entry(p).or_insert(Color::Black);
        iss.feed_input((*c).into());

        if let (Some(o1), Some(o2)) =
            (iss.run_till_output()?, iss.run_till_output()?)
        {
            *c = o1.try_into()?;
            d = d.turn(o2)?;
            p = step(p, d);
        } else {
            break;
        }
    }

    Ok(())
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let mut g = Grid::new();
    run(input, &mut g)?;
    Ok(g.len())
}

pub fn part2(input: &str) -> crate::Result<String> {
    let mut g = Grid::new();
    g.insert((0, 0), Color::White);
    run(input, &mut g)?;

    let (xmin, xmax, ymin, ymax) = g.iter().fold(
        (0, 0, 0, 0),
        |mut acc, (&(x, y), _)| {
            acc.0 = min(acc.0, x);
            acc.1 = max(acc.1, x);
            acc.2 = min(acc.2, y);
            acc.3 = max(acc.3, y);
            acc
        },
    );

    let mut id = String::new();
    id.push('\n');
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            match g.get(&(x, y)) {
                Some(Color::White) => id.push('#'),
                _ => id.push(' '),
            }
        }
        id.push('\n');
    }

    Ok(id)
}
