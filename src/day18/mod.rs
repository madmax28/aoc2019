use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    EntraceNotFound,
    PathNotFound,
}

type Pos = (u8, u8);
type Dist = i32;

fn neighbors(p: Pos) -> Vec<Pos> {
    vec![
        (p.0, p.1 - 1),
        (p.0, p.1 + 1),
        (p.0 - 1, p.1),
        (p.0 + 1, p.1),
    ]
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

    fn entrance(&self) -> Option<Pos> {
        self.map.iter().find(|(_, &c)| c == '@').map(|(p, _)| *p)
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

fn collect_keys_bfs(
    vault: Vault,
    keys: HashSet<Pos>,
    queue: &mut VecDeque<(VecDeque<Pos>, HashSet<Pos>, HashSet<char>)>,
) -> Option<Dist> {
    let mut dist = 0;
    while !queue.is_empty() {
        for off in 0..queue.len() {
            let (mut ps, mut visited, have_keys) = queue.pop_front().unwrap();

            for _ in 0..ps.len() {
                let pos = ps.pop_front().unwrap();

                for n in neighbors(pos) {
                    match vault.map.get(&n).unwrap() {
                        '#' => (),
                        c @ 'a'..='z' => {
                            if !visited.insert(n) {
                                continue;
                            }

                            if have_keys.contains(&c) {
                                // got key, treat as empty
                                ps.push_back(n);
                                continue;
                            }

                            let mut have_keys = have_keys.clone();
                            have_keys.insert(*c);
                            if have_keys.len() == keys.len() {
                                return Some(dist + 1);
                            }

                            // memory optimization: avoid duplicate queue
                            // entries with same keys
                            if queue.len() > off + 1 {
                                let skip = queue.len() - off - 1;
                                if let Some((ps, vs, _)) = queue
                                    .iter_mut()
                                    .skip(skip)
                                    .find(|(.., hks)| hks == &have_keys)
                                {
                                    ps.push_back(n);
                                    vs.insert(n);
                                    continue;
                                }
                            }

                            let mut vs = HashSet::new();
                            vs.insert(n);
                            queue.push_back((vec![n].into(), vs, have_keys));
                        }
                        c @ 'A'..='Z' => {
                            if !have_keys.contains(&c.to_ascii_lowercase()) {
                                // key missing, treat as blocked
                                continue;
                            }

                            if !visited.insert(n) {
                                continue;
                            }

                            ps.push_back(n);
                        }
                        '.' | '@' => {
                            if !visited.insert(n) {
                                continue;
                            }

                            ps.push_back(n);
                        }
                        _ => unreachable!(),
                    }
                }
            }

            if !ps.is_empty() {
                queue.push_back((ps, visited, have_keys));
            }
        }

        dist += 1;
    }

    None
}

fn collect_keys(vault: Vault) -> crate::Result<Option<Dist>> {
    let entrance = vault
        .entrance()
        .ok_or_else(|| crate::Error::boxed(Error::EntraceNotFound))?;
    let keys = vault.keys();
    let mut visited = HashSet::new();
    visited.insert(entrance);

    Ok(collect_keys_bfs(
        vault,
        keys.into_iter().collect(),
        &mut vec![(vec![entrance].into(), visited, HashSet::new())].into(),
    ))
}

pub fn part1(input: &str) -> crate::Result<Dist> {
    let vault = Vault::from_str(input).unwrap();
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
        assert_eq!(vault.entrance().unwrap(), (5, 1));
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
        assert_eq!(vault.entrance().unwrap(), (15, 1));
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
        assert_eq!(vault.entrance().unwrap(), (6, 3));
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
        assert_eq!(vault.entrance().unwrap(), (8, 4));
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
        assert_eq!(vault.entrance().unwrap(), (1, 1));
        assert_eq!(vault.keys().len(), 9);
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 81);
    }

    #[test]
    fn mine1() {
        let mut inp = String::new();
        inp.push_str("#########\n");
        inp.push_str("#.b...c.#\n");
        inp.push_str("####@####\n");
        inp.push_str("#.a...d.#\n");
        inp.push_str("#########");

        let vault = Vault::from_str(&inp).unwrap();
        assert_eq!(vault.entrance().unwrap(), (4, 2));
        assert_eq!(vault.keys().len(), 4);
        assert_eq!(collect_keys(vault).unwrap().unwrap(), 17);
    }
}
