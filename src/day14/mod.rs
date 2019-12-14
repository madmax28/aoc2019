use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type ChemId = i32;
type Component = (ChemId, i64);
type Reactions = HashMap<Component, Vec<Component>>;
type IdMap = HashMap<String, ChemId>;

fn scan_for<I>(i: &mut I) -> crate::Result<(String, i64)>
where
    I: Iterator<Item = char>,
{
    let n = i
        .skip_while(|&c| !c.is_digit(10))
        .take_while(|&c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .map_err(|_| crate::Error::boxed(Error::InvalidInput))?;
    let c = i
        .skip_while(|&c| !c.is_ascii_uppercase())
        .take_while(|&c| c.is_ascii_uppercase())
        .collect::<String>();

    if c.is_empty() {
        Err(crate::Error::boxed(Error::InvalidInput))
    } else {
        Ok((c, n))
    }
}

fn parse(input: &str) -> crate::Result<(Reactions, IdMap)> {
    let mut reactions = Reactions::new();
    let mut ids = IdMap::new();
    let mut idc = 0;

    for l in input.lines() {
        let mut cs = l.chars().peekable();
        let mut chems = Vec::new();
        while cs.peek().is_some() {
            let (c, n) = scan_for(&mut cs)?;
            let id = ids.entry(c).or_insert_with(|| {
                idc += 1;
                idc - 1
            });

            chems.push((*id, n));
        }

        let product = chems
            .pop()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
        if chems.is_empty() {
            return Err(crate::Error::boxed(Error::InvalidInput));
        }

        reactions.insert(product, chems);
    }

    Ok((reactions, ids))
}

fn react(
    fuel_need: i64,
    reactions: &Reactions,
    fuel_id: ChemId,
    ore_id: ChemId,
) -> crate::Result<i64> {
    let mut need = HashMap::new();
    need.insert(fuel_id, fuel_need);
    let mut spare = HashMap::new();

    while !(need.len() == 1 && need.get(&ore_id).is_some()) {
        let mut need_new = HashMap::new();
        need_new.insert(ore_id, need.remove(&ore_id).unwrap_or(0));

        for (chem_need, cnt_need) in need {
            let r = reactions
                .iter()
                .find(|((cc, _), _)| *cc == chem_need)
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;

            {
                let mut times = cnt_need / (r.0).1;
                let mut rem = cnt_need % (r.0).1;
                if rem != 0 {
                    rem = (r.0).1 - rem;
                    times += 1;
                }

                for (chem_gen, cnt_gen) in r.1 {
                    *need_new.entry(*chem_gen).or_insert(0) += times * cnt_gen;
                }
                *spare.entry(chem_need).or_insert(0) += rem;
            }

            for (chem_spare, cnt_spare) in &mut spare {
                let cnt_need = need_new.entry(*chem_spare).or_insert(0);
                let d = *cnt_need - *cnt_spare;
                *cnt_spare = (-d).max(0);
                *cnt_need = d.max(0);
            }
        }

        need_new.retain(|_, &mut n| n != 0);
        need = need_new;
    }

    Ok(*need.get(&ore_id).unwrap())
}

pub fn part1(input: &str) -> crate::Result<i64> {
    let (reactions, ids) = parse(input)?;
    let fuel_id = *ids
        .get("FUEL")
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    let ore_id = *ids
        .get("ORE")
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;

    Ok(react(1, &reactions, fuel_id, ore_id)?)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    const GOAL: i64 = 1_000_000_000_000;
    let (reactions, ids) = parse(input)?;
    let fuel_id = *ids
        .get("FUEL")
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    let ore_id = *ids
        .get("ORE")
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;

    let mut upper_bound = 10_000_000; // arbitrary
    let mut lower_bound = 0;
    Ok(loop {
        let cand = (upper_bound + lower_bound) / 2;
        let ore_needed = react(cand, &reactions, fuel_id, ore_id)?;

        if ore_needed > GOAL {
            upper_bound = cand;
        } else {
            lower_bound = cand
        }

        if upper_bound - lower_bound == 1 {
            break lower_bound;
        }
    })
}
