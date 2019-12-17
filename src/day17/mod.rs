use crate::day09::{Iss, StopReason, Value};

use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug)]
enum Error {
    RobotNotFound,
    CouldNotSolve,
}

type Pos = (i32, i32);

fn neighbors(p: Pos) -> Vec<Pos> {
    vec![
        (p.0, p.1 - 1),
        (p.0, p.1 + 1),
        (p.0 - 1, p.1),
        (p.0 + 1, p.1),
    ]
}

#[derive(Debug, PartialEq, Clone)]
enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum Facing {
    North,
    South,
    West,
    East,
}

trait ToAscii {
    fn to_ascii(&self) -> String;
}

type Segment = (Turn, i32);

impl ToAscii for Segment {
    fn to_ascii(&self) -> String {
        format!(
            "{},{}",
            match self.0 {
                Turn::Left => "L",
                Turn::Right => "R",
            },
            self.1
        )
    }
}

type Path = Vec<Segment>;

impl ToAscii for Path {
    fn to_ascii(&self) -> String {
        let mut res = String::new();
        for (i, s) in self.iter().enumerate() {
            if i > 0 {
                res.push(',');
            }
            res.push_str(&s.to_ascii());
        }
        res.push('\n');
        res
    }
}

type Main = Vec<usize>;

impl ToAscii for Main {
    fn to_ascii(&self) -> String {
        let mut res = String::new();
        for (i, c) in self.iter().enumerate() {
            if i > 0 {
                res.push(',');
            }
            res.push(match c {
                0 => 'A',
                1 => 'B',
                2 => 'C',
                _ => unreachable!(),
            });
        }
        res.push('\n');
        res
    }
}

fn matches(path: &[Segment], cand: &[Segment]) -> bool {
    if cand.len() > path.len() {
        return false;
    }
    path[..cand.len()] == cand[..cand.len()]
}

fn solve(
    path: &[Segment],
    segments: &[Path],
    main: &mut Main,
    s: usize,
) -> bool {
    for (idx, seg) in segments.iter().enumerate() {
        if matches(&path[s..], seg) {
            main.push(idx);
            if s + seg.len() >= path.len()
                || solve(path, segments, main, s + seg.len())
            {
                return true;
            }
            main.pop();
        }
    }

    false
}

fn find_routines(
    path: &[Segment],
    mut off: usize,
    main: &mut Main,
    routines: &mut Vec<Path>,
) -> bool {
    const MAX_LEN: usize = 20;
    const MAX_RTNS: usize = 3;

    if routines.len() == MAX_RTNS {
        return false;
    }

    while let Some(seg) = routines.iter().find(|p| matches(&path[off..], p)) {
        off += seg.len();
    }

    for len in (1..=path.len() - off).rev() {
        let cand = path[off..off + len].to_vec();
        if cand.to_ascii().len() > MAX_LEN {
            continue;
        }

        let cand_len = cand.len();
        routines.push(cand);
        if solve(path, routines, main, 0)
            || find_routines(path, off + cand_len, main, routines)
        {
            return true;
        }
        routines.pop();
    }

    false
}

#[derive(Clone)]
struct Grid {
    map: HashMap<Pos, char>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            map: HashMap::new(),
        }
    }

    fn is_intersection(&self, p: Pos) -> bool {
        if let Some(&'#') = self.map.get(&p) {
            neighbors(p).iter().all(|p| self.map.get(p) == Some(&'#'))
        } else {
            false
        }
    }

    fn turn(&self, p: Pos, f: Facing) -> Option<(Pos, Facing, Turn)> {
        match f {
            Facing::North => vec![
                ((p.0 - 1, p.1), Facing::West, Turn::Left),
                ((p.0 + 1, p.1), Facing::East, Turn::Right),
            ],
            Facing::South => vec![
                ((p.0 - 1, p.1), Facing::West, Turn::Right),
                ((p.0 + 1, p.1), Facing::East, Turn::Left),
            ],
            Facing::West => vec![
                ((p.0, p.1 - 1), Facing::North, Turn::Right),
                ((p.0, p.1 + 1), Facing::South, Turn::Left),
            ],
            Facing::East => vec![
                ((p.0, p.1 - 1), Facing::North, Turn::Left),
                ((p.0, p.1 + 1), Facing::South, Turn::Right),
            ],
        }
        .into_iter()
        .find(|(p, ..)| self.map.get(p) == Some(&'#'))
    }

    fn step(&self, p: Pos, f: Facing) -> Option<Pos> {
        let pp = match f {
            Facing::North => (p.0, p.1 - 1),
            Facing::South => (p.0, p.1 + 1),
            Facing::West => (p.0 - 1, p.1),
            Facing::East => (p.0 + 1, p.1),
        };

        self.map
            .get(&pp)
            .and_then(|&c| if c == '#' { Some(pp) } else { None })
    }

    fn calc_path(&self) -> crate::Result<Path> {
        let mut pos: (i32, i32) = *self
            .map
            .iter()
            .find(|(_, &c)| c == '^')
            .ok_or_else(|| crate::Error::boxed(Error::RobotNotFound))?
            .0;

        let mut facing = Facing::North;
        let mut path = Path::new();
        while let Some((p, f, t)) = self.turn(pos, facing) {
            facing = f;
            pos = p;

            let mut steps = 1;
            while let Some(p) = self.step(pos, facing) {
                steps += 1;
                pos = p;
            }

            path.push((t, steps));
        }

        Ok(path)
    }
}

fn create_grid(iss: &mut Iss) -> crate::Result<Grid> {
    let mut pos = (0, 0);
    let mut grid = Grid::new();
    while let Ok(StopReason::Output(c)) = iss.run() {
        let c = u8::try_from(c)?.into();
        match c {
            '\n' => pos = (0, pos.1 + 1),
            c => {
                grid.map.insert(pos, c);
                pos = (pos.0 + 1, pos.1);
            }
        }
    }

    Ok(grid)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let grid = create_grid(&mut Iss::new(mem))?;

    Ok(grid
        .map
        .keys()
        .filter(|&&p| grid.is_intersection(p))
        .fold(0, |acc, (x, y)| acc + x * y))
}

pub fn part2(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .trim()
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let mut iss = Iss::new(mem);
    *iss.access(0) = 2;

    let grid = create_grid(&mut iss)?;
    let path = grid.calc_path()?;

    let mut main = Vec::new();
    let mut routines = Vec::new();
    if !find_routines(&path, 0, &mut main, &mut routines) {
        return Err(crate::Error::boxed(Error::CouldNotSolve));
    }

    iss.feed_str(&main.to_ascii())?;
    iss.feed_str(&routines[0].to_ascii())?;
    iss.feed_str(&routines[1].to_ascii())?;
    iss.feed_str(&routines[2].to_ascii())?;
    iss.feed_str("n\n")?;

    let mut res = 0;
    while let StopReason::Output(o) = iss.run()? {
        res = o;
    }
    Ok(res)
}
