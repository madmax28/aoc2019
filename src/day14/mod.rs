use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type ChemId = i32;

#[derive(PartialEq, Eq, Hash)]
struct Component {
    id: ChemId,
    cnt: i64,
}

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
            let (name, cnt) = scan_for(&mut cs)?;
            let id = *ids.entry(name).or_insert_with(|| {
                idc += 1;
                idc - 1
            });

            chems.push(Component { id, cnt });
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
    id: ChemId,
    cnt: i64,
    reactions: &Reactions,
    spares: &mut HashMap<ChemId, i64>,
    ore_id: ChemId,
) -> crate::Result<i64> {
    let (product, reactants) = reactions
        .iter()
        .find(|(c, _)| c.id == id)
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    let num_reactions = (cnt + product.cnt - 1) / product.cnt;
    let cnt_remaining = (product.cnt - cnt).rem_euclid(product.cnt);

    spares.insert(id, cnt_remaining);

    let mut ore_cnt = 0;
    for reactant in reactants {
        let mut cnt_needed = reactant.cnt * num_reactions;

        if reactant.id == ore_id {
            ore_cnt += cnt_needed;
            continue;
        }

        if let Some(cnt_spare) = spares.get_mut(&reactant.id) {
            let spares_used = cnt_needed.min(*cnt_spare);
            *cnt_spare -= spares_used;
            cnt_needed -= spares_used;

            if cnt_needed == 0 {
                continue;
            }
        }

        ore_cnt += react(reactant.id, cnt_needed, reactions, spares, ore_id)?;
    }

    Ok(ore_cnt)
}

pub fn part1(input: &str) -> crate::Result<i64> {
    let (reactions, ids) = parse(input)?;
    let fuel_id = *ids
        .get("FUEL")
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    let ore_id = *ids
        .get("ORE")
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;

    Ok(react(fuel_id, 1, &reactions, &mut HashMap::new(), ore_id)?)
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
        let ore_needed =
            react(fuel_id, cand, &reactions, &mut HashMap::new(), ore_id)?;

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
