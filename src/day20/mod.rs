use std::collections::{HashSet, VecDeque};
use std::convert::Into;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;
use std::marker::Sized;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    EntranceNotFound,
    ExitNotFound,
    InvalidInput,
    NoPathFound,
}

type Point = (i32, i32);

trait Pos
where
    Self: Sized + Debug + Eq + Clone + Copy + Hash,
{
    fn from_point(p: Point) -> Self;
    fn neighbors(&self, sz: Sz) -> Vec<Self>;
    fn port(&self, p: Point, outer: bool) -> Option<Self>;
    fn as_point(&self) -> Point;
}

impl Pos for Point {
    fn from_point(p: Point) -> Self {
        p
    }

    fn neighbors(&self, sz: Sz) -> Vec<Self> {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .map(move |p| (self.0 + p.0, self.1 + p.1))
            .filter(move |p| p.0 >= 0 && p.0 < sz.0 && p.1 >= 0 && p.1 < sz.1)
            .collect()
    }

    fn port(&self, p: Point, _outer: bool) -> Option<Self> {
        Some(p)
    }

    fn as_point(&self) -> Point {
        *self
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PosRec {
    pos: Point,
    depth: i32,
}

impl PosRec {
    fn new(pos: Point, depth: i32) -> Self {
        PosRec { pos, depth }
    }
}

impl Pos for PosRec {
    fn from_point(p: Point) -> Self {
        PosRec::new(p, 0)
    }

    fn neighbors(&self, sz: Sz) -> Vec<Self> {
        let d = self.depth;
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .map(move |p| (self.pos.0 + p.0, self.pos.1 + p.1))
            .filter(move |p| p.0 >= 0 && p.0 < sz.0 && p.1 >= 0 && p.1 < sz.1)
            .map(move |pp| PosRec::new(pp, d))
            .collect()
    }

    fn port(&self, p: Point, outer: bool) -> Option<Self> {
        if outer && self.depth > 0 {
            Some(PosRec::new(p, self.depth - 1))
        } else if !outer {
            Some(PosRec::new(p, self.depth + 1))
        } else {
            None
        }
    }

    fn as_point(&self) -> Point {
        self.pos
    }
}

type Dist = i32;
type Sz = (i32, i32);

fn is_outer(p: Point, sz: Sz) -> bool {
    p.0 == 2 || p.0 + 3 == sz.0 || p.1 == 2 || p.1 + 3 == sz.1
}

#[derive(Debug)]
struct Portal {
    inner: Point,
    outer: Point,
}

impl Portal {
    fn try_enter<T>(&self, p: T) -> Option<T>
    where
        T: Pos,
    {
        if p.as_point() == self.inner {
            p.port(self.outer, false)
        } else if p.as_point() == self.outer {
            p.port(self.inner, true)
        } else {
            None
        }
    }
}

struct Maze<T>
where
    T: Pos,
{
    map: Vec<char>,
    sz: Sz,
    entrance: T,
    exit: T,
    portals: Vec<Portal>,
}

impl<T> Maze<T>
where
    T: Pos,
{
    fn new(map: Vec<char>, sz: Sz) -> crate::Result<Self> {
        let mut m = Maze {
            map,
            sz,
            entrance: T::from_point((0, 0)),
            exit: T::from_point((0, 0)),
            portals: Vec::new(),
        };

        m.entrance = m
            .ps()
            .filter(|&p| m.get(p) == '.')
            .find(|&p| {
                if let Some(p) =
                    p.neighbors(m.sz).iter().find(|&p| m.get(*p) == 'A')
                {
                    p.neighbors(m.sz).iter().any(|&p| m.get(p) == 'A')
                } else {
                    false
                }
            })
            .ok_or_else(|| crate::Error::boxed(Error::EntranceNotFound))?;

        m.exit = m
            .ps()
            .filter(|&p| m.get(p) == '.')
            .find(|&p| {
                if let Some(p) =
                    p.neighbors(m.sz).iter().find(|&p| m.get(*p) == 'Z')
                {
                    p.neighbors(m.sz).iter().any(|&p| m.get(p) == 'Z')
                } else {
                    false
                }
            })
            .ok_or_else(|| crate::Error::boxed(Error::ExitNotFound))?;

        let mut portals = Vec::new();
        for entry_pos in m.ps().filter(|&p| p != m.entrance && m.get(p) == '.')
        {
            if let Some(&p) = entry_pos
                .neighbors(m.sz)
                .iter()
                .find(|&&p| m.get(p).is_ascii_uppercase())
            {
                let entry_char = m.get(p);
                let exit_char = m.get(
                    *p.neighbors(m.sz)
                        .iter()
                        .find(|&&p| m.get(p).is_ascii_uppercase())
                        .ok_or_else(|| {
                            crate::Error::boxed(Error::InvalidInput)
                        })?,
                );

                if entry_char != exit_char {
                    portals.push((
                        entry_pos.as_point(),
                        entry_char,
                        exit_char,
                        is_outer(entry_pos.as_point(), m.sz),
                    ));
                }
            }
        }

        while let Some(portal) = portals.pop() {
            let mut found = false;
            for idx in 0..portals.len() {
                let cand = &portals[idx];
                if cand.3 != portal.3
                    && ((cand.1 == portal.2 && cand.2 == portal.1)
                        || (cand.1 == portal.1 && cand.2 == portal.2))
                {
                    let (outer, inner) = if portal.3 {
                        (portal.0, cand.0)
                    } else {
                        (cand.0, portal.0)
                    };
                    m.portals.push(Portal { inner, outer });
                    portals.swap_remove(idx);
                    found = true;
                    break;
                }
            }

            if !found {
                return Err(crate::Error::boxed(Error::InvalidInput));
            }
        }

        Ok(m)
    }

    fn get(&self, p: T) -> char {
        let p = p.as_point();
        self.map[(p.0 + p.1 * self.sz.0) as usize]
    }

    fn ps(&self) -> impl Iterator<Item = T> {
        let sz: Sz = self.sz;
        (0..sz.1)
            .flat_map(move |y| (0..sz.0).zip(iter::repeat(y)))
            .map(T::from_point)
    }

    fn port(&self, p: T) -> Option<T> {
        self.portals.iter().find_map(|portal| {
            if let Some(res) = portal.try_enter(p) {
                Some(res)
            } else {
                None
            }
        })
    }

    fn reachable(&self, p: T) -> impl Iterator<Item = T> {
        let mut ns: Vec<T> = p
            .neighbors(self.sz)
            .into_iter()
            .filter(|&p| self.get(p) == '.')
            .collect();
        if let Some(p) = self.port(p) {
            ns.push(p)
        }
        ns.into_iter()
    }

    fn find_path(
        &self,
        mut visited: HashSet<T>,
        mut queue: VecDeque<T>,
    ) -> Option<Dist> {
        let mut dist = 0;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let pos = queue.pop_front().unwrap();

                for n in self.reachable(pos) {
                    if n == self.exit {
                        return Some(dist + 1);
                    }

                    if !visited.insert(n.clone()) {
                        continue;
                    }

                    queue.push_back(n);
                }
            }

            dist += 1;
        }

        None
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.sz.1 {
            for x in 0..self.sz.0 {
                print!("{}", self.get(T::from_point((x, y))));
            }
            println!();
        }
    }
}

impl<T> FromStr for Maze<T>
where
    T: Pos,
{
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut sz_x, mut sz_y) = (None, 0);
        let mut map = Vec::new();
        for l in s.lines() {
            let mut cur_sz_x = 0;
            for c in l.chars() {
                map.push(c);
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
        assert_eq!(map.len(), (sz_y * sz_x.unwrap_or(0)) as usize);

        Ok(Maze::new(map, (sz_x.unwrap_or(0), sz_y))?)
    }
}

pub fn part1(input: &str) -> crate::Result<Dist> {
    let maze = Maze::from_str(input)?;
    let entrance: Point = maze.entrance;
    let dist = maze
        .find_path([entrance].iter().cloned().collect(), vec![entrance].into())
        .ok_or_else(|| crate::Error::boxed(Error::NoPathFound))?;

    Ok(dist)
}

pub fn part2(input: &str) -> crate::Result<Dist> {
    let maze = Maze::from_str(input)?;
    let entrance: PosRec = maze.entrance;
    let dist = maze
        .find_path([entrance].iter().cloned().collect(), vec![entrance].into())
        .ok_or_else(|| crate::Error::boxed(Error::NoPathFound))?;

    Ok(dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut inp = String::new();
        inp.push_str("         A           \n");
        inp.push_str("         A           \n");
        inp.push_str("  #######.#########  \n");
        inp.push_str("  #######.........#  \n");
        inp.push_str("  #######.#######.#  \n");
        inp.push_str("  #######.#######.#  \n");
        inp.push_str("  #######.#######.#  \n");
        inp.push_str("  #####  B    ###.#  \n");
        inp.push_str("BC...##  C    ###.#  \n");
        inp.push_str("  ##.##       ###.#  \n");
        inp.push_str("  ##...DE  F  ###.#  \n");
        inp.push_str("  #####    G  ###.#  \n");
        inp.push_str("  #########.#####.#  \n");
        inp.push_str("DE..#######...###.#  \n");
        inp.push_str("  #.#########.###.#  \n");
        inp.push_str("FG..#########.....#  \n");
        inp.push_str("  ###########.#####  \n");
        inp.push_str("             Z       \n");
        inp.push_str("             Z       ");

        assert_eq!(part1(&inp).unwrap(), 23);
    }

    #[test]
    fn p2_ex1() {
        let mut inp = String::new();
        //            0         10        20        30        40
        //            012345678901234567890123456789012345678901234
        inp.push_str("             Z L X W       C                 \n"); // 0
        inp.push_str("             Z P Q B       K                 \n"); // 1
        inp.push_str("  ###########.#.#.#.#######.###############  \n"); // 2
        inp.push_str("  #...#.......#.#.......#.#.......#.#.#...#  \n"); // 3
        inp.push_str("  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n"); // 4
        inp.push_str("  #.#...#.#.#...#.#.#...#...#...#.#.......#  \n"); // 5
        inp.push_str("  #.###.#######.###.###.#.###.###.#.#######  \n"); // 6
        inp.push_str("  #...#.......#.#...#...#.............#...#  \n"); // 7
        inp.push_str("  #.#########.#######.#.#######.#######.###  \n"); // 8
        inp.push_str("  #...#.#    F       R I       Z    #.#.#.#  \n"); // 9
        inp.push_str("  #.###.#    D       E C       H    #.#.#.#  \n"); // 10
        inp.push_str("  #.#...#                           #...#.#  \n"); // 11
        inp.push_str("  #.###.#                           #.###.#  \n"); // 12
        inp.push_str("  #.#....OA                       WB..#.#..ZH\n"); // 13
        inp.push_str("  #.###.#                           #.#.#.#  \n"); // 14
        inp.push_str("CJ......#                           #.....#  \n"); // 15
        inp.push_str("  #######                           #######  \n"); // 16
        inp.push_str("  #.#....CK                         #......IC\n"); // 17
        inp.push_str("  #.###.#                           #.###.#  \n"); // 18
        inp.push_str("  #.....#                           #...#.#  \n"); // 19
        inp.push_str("  ###.###                           #.#.#.#  \n"); // 20
        inp.push_str("XF....#.#                         RF..#.#.#  \n"); // 21
        inp.push_str("  #####.#                           #######  \n"); // 22
        inp.push_str("  #......CJ                       NM..#...#  \n"); // 23
        inp.push_str("  ###.#.#                           #.###.#  \n"); // 24
        inp.push_str("RE....#.#                           #......RF\n"); // 25
        inp.push_str("  ###.###        X   X       L      #.#.#.#  \n"); // 26
        inp.push_str("  #.....#        F   Q       P      #.#.#.#  \n"); // 27
        inp.push_str("  ###.###########.###.#######.#########.###  \n"); // 28
        inp.push_str("  #.....#...#.....#.......#...#.....#.#...#  \n"); // 29
        inp.push_str("  #####.#.###.#######.#######.###.###.#.#.#  \n"); // 30
        inp.push_str("  #.......#.......#.#.#.#.#...#...#...#.#.#  \n"); // 31
        inp.push_str("  #####.###.#####.#.#.#.#.###.###.#.###.###  \n"); // 32
        inp.push_str("  #.......#.....#.#...#...............#...#  \n"); // 33
        inp.push_str("  #############.#.#.###.###################  \n"); // 34
        inp.push_str("               A O F   N                     \n"); // 35
        inp.push_str("               A A D   M                     "); // 36

        assert_eq!(part2(&inp).unwrap(), 396);
    }

    #[test]
    fn p2_mine1() {
        let mut inp = String::new();
        inp.push_str("   A   \n");
        inp.push_str("   A   \n");
        inp.push_str(" ##.#  \n");
        inp.push_str("#...#  \n");
        inp.push_str("#.#B#  \n");
        inp.push_str("#.#C#  \n");
        inp.push_str("#.#D#  \n");
        inp.push_str("#.#E## \n");
        inp.push_str("#.....#\n");
        inp.push_str("#.#.#.#\n");
        inp.push_str(" C D Z \n");
        inp.push_str(" B E Z ");

        assert_eq!(part2(&inp).unwrap(), 10);
    }
}
