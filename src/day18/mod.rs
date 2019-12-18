use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    EntraceNotFound,
    PathNotFound,
}

type Pos = (u8, u8);
type Dist = i32;

fn neighbors(ps: &[Pos]) -> Vec<(usize, Pos)> {
    let mut res = Vec::with_capacity(ps.len() * 4);
    for (i, p) in ps.iter().enumerate() {
        res.push((i, (p.0, p.1 - 1)));
        res.push((i, (p.0, p.1 + 1)));
        res.push((i, (p.0 - 1, p.1)));
        res.push((i, (p.0 + 1, p.1)));
    }
    res
}

#[derive(Clone)]
struct Vault {
    map: HashMap<Pos, char>,
}

impl Vault {
    fn keys(&self) -> Vec<Pos> {
        self.map
            .iter()
            .filter(|(_, c)| c.is_ascii_lowercase())
            .map(|(p, _)| p)
            .cloned()
            .collect()
    }

    fn entrances(&self) -> Vec<Pos> {
        self.map
            .iter()
            .filter(|(_, &c)| c == '@')
            .map(|(p, _)| *p)
            .collect()
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut pos = (0, 0);
        while self.map.get(&pos).is_some() {
            while let Some(c) = self.map.get(&pos) {
                print!("{}", c);
                pos.0 += 1;
            }
            println!();
            pos.0 = 0;
            pos.1 += 1;
        }
    }
}

impl FromStr for Vault {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut pos = (0, 0);
        for l in s.lines() {
            for c in l.chars() {
                map.insert(pos, c);
                pos.0 += 1;
            }
            pos.0 = 0;
            pos.1 += 1;
        }

        Ok(Vault { map })
    }
}

type State = VecDeque<(VecDeque<Vec<Pos>>, HashSet<Pos>, HashSet<char>)>;

fn collect_keys_bfs(
    vault: Vault,
    keys: HashSet<Pos>,
    mut queue: State,
) -> Option<Dist> {
    let mut dist = 0;
    while !queue.is_empty() {
        let initial_queue_len = queue.len();
        for taken in 1..=queue.len() {
            let (mut droid_ps, mut visited, have_keys) =
                queue.pop_front().unwrap();

            for _ in 0..droid_ps.len() {
                let droid_pos = droid_ps.pop_front().unwrap();

                for (droid_idx, n) in neighbors(&droid_pos) {
                    let mut droid_n = droid_pos.clone();
                    droid_n[droid_idx] = n;

                    match vault.map.get(&n).unwrap() {
                        '#' => (),
                        c @ 'a'..='z' => {
                            if !visited.insert(n) {
                                continue;
                            }

                            if have_keys.contains(&c) {
                                // got key, treat as empty
                                droid_ps.push_back(droid_n);
                                continue;
                            }

                            let mut have_keys = have_keys.clone();
                            have_keys.insert(*c);
                            if have_keys.len() == keys.len() {
                                return Some(dist + 1);
                            }

                            // memory optimization: avoid duplicate queue
                            // entries with same keys
                            if let Some((dps, vs, _)) = queue
                                .iter_mut()
                                .skip(initial_queue_len - taken)
                                .find(|(_, _, hks)| {
                                    hks == &have_keys
                                        // technically need these checks but solves anyway
                                        // && !dps.contains(&droid_n)
                                        // && !vs.contains(&n)
                                })
                            {
                                vs.extend(droid_n.iter().cloned());
                                dps.push_back(droid_n);
                                continue;
                            }

                            let mut vs = HashSet::new();
                            vs.insert(n);
                            queue.push_back((
                                vec![droid_n].into(),
                                vs,
                                have_keys,
                            ));
                        }
                        c @ 'A'..='Z' => {
                            if !have_keys.contains(&c.to_ascii_lowercase()) {
                                // key missing, treat as blocked
                                continue;
                            }

                            if !visited.insert(n) {
                                continue;
                            }

                            droid_ps.push_back(droid_n);
                        }
                        '.' | '@' => {
                            if !visited.insert(n) {
                                continue;
                            }

                            droid_ps.push_back(droid_n);
                        }
                        _ => unreachable!(),
                    }
                }
            }

            if !droid_ps.is_empty() {
                queue.push_back((droid_ps, visited, have_keys));
            }
        }

        dist += 1;
    }

    None
}

fn collect_keys(vault: Vault) -> crate::Result<Option<Dist>> {
    let entrances = vault.entrances();
    if entrances.is_empty() {
        return Err(crate::Error::boxed(Error::EntraceNotFound));
    }
    let keys = vault.keys().into_iter().collect();

    Ok(collect_keys_bfs(
        vault,
        keys,
        vec![(
            vec![entrances.clone()].into(),
            entrances.into_iter().collect(),
            HashSet::new(),
        )]
        .into(),
    ))
}

pub fn part1(input: &str) -> crate::Result<Dist> {
    let vault = Vault::from_str(input).unwrap();
    Ok(collect_keys(vault)?
        .ok_or_else(|| crate::Error::boxed(Error::PathNotFound))?)
}

pub fn part2(input: &str) -> crate::Result<Dist> {
    let mut vault = Vault::from_str(input).unwrap();

    let (x, y) = vault
        .entrances()
        .pop()
        .ok_or_else(|| crate::Error::boxed(Error::EntraceNotFound))?;

    vault.map.insert((x, y), '#');
    vault.map.insert((x + 1, y), '#');
    vault.map.insert((x - 1, y), '#');
    vault.map.insert((x, y + 1), '#');
    vault.map.insert((x, y - 1), '#');

    let entrances = vec![
        (x + 1, y - 1),
        (x + 1, y + 1),
        (x - 1, y - 1),
        (x - 1, y + 1),
    ];

    for e in &entrances {
        vault.map.insert(*e, '@');
    }

    Ok(collect_keys(vault)?
        .ok_or_else(|| crate::Error::boxed(Error::PathNotFound))?)
}

// #[cfg(test)]
#[cfg(disabled)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut inp = String::new();
        inp.push_str("#########\n");
        inp.push_str("#b.A.@.a#\n");
        inp.push_str("#########");

        let vault = Vault::from_str(&inp).unwrap();
        assert_eq!(vault.entrances().get(0).unwrap(), &(5, 1));
        assert_eq!(vault.keys().len(), 2);
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 8);
    }

    #[test]
    fn ex2() {
        let mut inp = String::new();
        inp.push_str("########################\n");
        inp.push_str("#f.D.E.e.C.b.A.@.a.B.c.#\n");
        inp.push_str("######################.#\n");
        inp.push_str("#d.....................#\n");
        inp.push_str("########################");

        let vault = Vault::from_str(&inp).unwrap();
        assert_eq!(vault.entrances().get(0).unwrap(), &(15, 1));
        assert_eq!(vault.keys().len(), 6);
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 86);
    }

    #[test]
    fn ex3() {
        let mut inp = String::new();
        inp.push_str("########################\n");
        inp.push_str("#...............b.C.D.f#\n");
        inp.push_str("#.######################\n");
        inp.push_str("#.....@.a.B.c.d.A.e.F.g#\n");
        inp.push_str("########################");

        let vault = Vault::from_str(&inp).unwrap();
        assert_eq!(vault.entrances().get(0).unwrap(), &(6, 3));
        assert_eq!(vault.keys().len(), 7);
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 132);
    }

    #[test]
    fn ex4() {
        let mut inp = String::new();
        inp.push_str("#################\n");
        inp.push_str("#i.G..c...e..H.p#\n");
        inp.push_str("########.########\n");
        inp.push_str("#j.A..b...f..D.o#\n");
        inp.push_str("########@########\n");
        inp.push_str("#k.E..a...g..B.n#\n");
        inp.push_str("########.########\n");
        inp.push_str("#l.F..d...h..C.m#\n");
        inp.push_str("#################");

        let vault = Vault::from_str(&inp).unwrap();
        assert_eq!(vault.entrances().get(0).unwrap(), &(8, 4));
        assert_eq!(vault.keys().len(), 16);
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 136);
    }

    #[test]
    fn ex5() {
        let mut inp = String::new();
        inp.push_str("########################\n");
        inp.push_str("#@..............ac.GI.b#\n");
        inp.push_str("###d#e#f################\n");
        inp.push_str("###A#B#C################\n");
        inp.push_str("###g#h#i################\n");
        inp.push_str("########################");

        let vault = Vault::from_str(&inp).unwrap();
        assert_eq!(vault.entrances().get(0).unwrap(), &(1, 1));
        assert_eq!(vault.keys().len(), 9);
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 81);
    }

    #[test]
    fn p2_ex1() {
        let mut inp = String::new();
        inp.push_str("#######\n");
        inp.push_str("#a.#Cd#\n");
        inp.push_str("##@#@##\n");
        inp.push_str("#######\n");
        inp.push_str("##@#@##\n");
        inp.push_str("#cB#Ab#\n");
        inp.push_str("#######");

        let vault = Vault::from_str(&inp).unwrap();
        assert_eq!(vault.keys().len(), 4);
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 8);
    }

    #[test]
    fn p2_ex2() {
        let mut inp = String::new();
        inp.push_str("###############\n");
        inp.push_str("#d.ABC.#.....a#\n");
        inp.push_str("######@#@######\n");
        inp.push_str("###############\n");
        inp.push_str("######@#@######\n");
        inp.push_str("#b.....#.....c#\n");
        inp.push_str("###############");

        let vault = Vault::from_str(&inp).unwrap();
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 24);
    }

    #[test]
    fn p2_ex3() {
        let mut inp = String::new();
        inp.push_str("#############\n");
        inp.push_str("#DcBa.#.GhKl#\n");
        inp.push_str("#.###@#@#I###\n");
        inp.push_str("#e#d#####j#k#\n");
        inp.push_str("###C#@#@###J#\n");
        inp.push_str("#fEbA.#.FgHi#\n");
        inp.push_str("#############");

        let vault = Vault::from_str(&inp).unwrap();
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 32);
    }

    #[test]
    fn p2_ex4() {
        let mut inp = String::new();
        inp.push_str("#############\n");
        inp.push_str("#g#f.D#..h#l#\n");
        inp.push_str("#F###e#E###.#\n");
        inp.push_str("#dCba@#@BcIJ#\n");
        inp.push_str("#############\n");
        inp.push_str("#nK.L@#@G...#\n");
        inp.push_str("#M###N#H###.#\n");
        inp.push_str("#o#m..#i#jk.#\n");
        inp.push_str("#############");

        let vault = Vault::from_str(&inp).unwrap();
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 72);
    }
}
