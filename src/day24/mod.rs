use std::collections::{HashMap, HashSet};
use std::iter;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type Sz = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
    lvl: i32,
}

impl Pos {
    fn new(x: i32, y: i32, lvl: i32) -> Self {
        Pos { x, y, lvl }
    }

    fn add(self, p: Pos) -> Self {
        Self::new(self.x + p.x, self.y + p.y, self.lvl)
    }

    fn all(sz: Sz) -> impl Iterator<Item = Pos> {
        (0..sz.1)
            .flat_map(move |y| (0..sz.0).zip(iter::repeat(y)))
            .map(|(x, y)| Self::new(x, y, 0))
    }

    fn neighbors(&self, sz: Sz) -> impl Iterator<Item = Self> {
        let mut ns = Vec::new();

        let (x, y, lvl) = (self.x, self.y, self.lvl);
        let (xmax, ymax) = (sz.0 - 1, sz.1 - 1);
        let center = (xmax / 2, ymax / 2);

        // x - 1
        if x == 0 {
            ns.push((xmax / 2 - 1, ymax / 2, lvl - 1));
        } else if (x - 1, y) == center {
            for y in 0..=ymax {
                ns.push((xmax, y, lvl + 1));
            }
        } else {
            ns.push((x - 1, y, lvl));
        }

        // x + 1
        if x == xmax {
            ns.push((xmax / 2 + 1, ymax / 2, lvl - 1));
        } else if (x + 1, y) == center {
            for y in 0..=ymax {
                ns.push((0, y, lvl + 1));
            }
        } else {
            ns.push((x + 1, y, lvl));
        }

        // y - 1
        if y == 0 {
            ns.push((xmax / 2, ymax / 2 - 1, lvl - 1));
        } else if (x, y - 1) == center {
            for x in 0..=xmax {
                ns.push((x, ymax, lvl + 1));
            }
        } else {
            ns.push((x, y - 1, lvl));
        }

        // y + 1
        if y == ymax {
            ns.push((xmax / 2, ymax / 2 + 1, lvl - 1));
        } else if (x, y + 1) == center {
            for x in 0..=xmax {
                ns.push((x, 0, lvl + 1));
            }
        } else {
            ns.push((x, y + 1, lvl));
        }

        ns.into_iter().map(|(x, y, lvl)| Self::new(x, y, lvl))
    }
}

#[derive(Debug, Clone)]
struct Grid {
    map: HashMap<Pos, char>,
    sz: Sz,
}

impl Grid {
    fn new(map: HashMap<Pos, char>, sz: Sz) -> Self {
        assert!(sz.0 % 2 == 1 && sz.1 % 2 == 1 && sz.0 == sz.1);
        Grid { map, sz }
    }

    fn tick(&mut self) {
        let patterns = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut new_map = HashMap::new();
        for p in Pos::all(self.sz) {
            let cnt = patterns
                .iter()
                .filter(|&&(x, y)| self.get(p.add(Pos::new(x, y, 0))) == '#')
                .count();

            match (self.get(p), cnt) {
                ('.', 1..=2) | ('#', 1) => {
                    new_map.insert(p, '#');
                }
                _ => (),
            }
        }
        self.map = new_map;
    }

    fn tick_rec(&mut self) {
        let to_check: HashSet<Pos> = self
            .map
            .keys()
            .cloned()
            .flat_map(|k| iter::once(k).chain(k.neighbors(self.sz)))
            .collect();

        let mut new_map = HashMap::new();
        for k in to_check {
            let cnt =
                k.neighbors(self.sz).filter(|&k| self.get(k) == '#').count();

            match (self.get(k), cnt) {
                ('.', 1..=2) | ('#', 1) => {
                    new_map.insert(k, '#');
                }
                _ => (),
            }
        }

        self.map = new_map;
    }

    fn biodiversity_rating(&self) -> i32 {
        let mut res = 0;
        for y in 0..self.sz.1 {
            for x in 0..self.sz.0 {
                res <<= 1;
                if self.get(Pos::new(x, y, 0)) == '#' {
                    res |= 1;
                }
            }
        }
        res
    }

    fn count_bugs(&self) -> i32 {
        self.map.values().filter(|&&c| c == '#').count() as i32
    }

    fn get(&self, k: Pos) -> char {
        *self.map.get(&k).unwrap_or(&'.')
    }

    #[allow(dead_code)]
    fn print(&self, lvl: i32) {
        for y in 0..self.sz.1 {
            for x in 0..self.sz.0 {
                print!("{}", self.get(Pos::new(x, y, lvl)));
            }
            println!();
        }
    }
}

impl FromStr for Grid {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut sz_x, mut sz_y) = (None, 0);
        let mut map = HashMap::new();
        for l in s.lines() {
            let mut cur_sz_x = 0;
            for c in l.chars() {
                if c == '#' {
                    map.insert(Pos::new(cur_sz_x, sz_y, 0), c);
                }
                cur_sz_x += 1;
            }

            // dumb error handling
            if let Some(sz_x) = sz_x {
                if cur_sz_x != sz_x {
                    return Err(crate::Error::boxed(Error::InvalidInput));
                }
            } else {
                sz_x = Some(cur_sz_x);
            }
            sz_y += 1;
        }

        Ok(Grid::new(map, (sz_x.unwrap_or(0), sz_y)))
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mut g = Grid::from_str(input)?;

    let mut seen = HashSet::new();
    loop {
        let mut key: Vec<Pos> = g.map.keys().cloned().collect();
        key.sort();
        if !seen.insert(key) {
            break;
        }

        g.tick();
    }

    Ok(g.biodiversity_rating())
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mut g = Grid::from_str(input)?;

    for _ in 0..200 {
        g.tick_rec();
    }

    Ok(g.count_bugs())
}
