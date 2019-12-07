use crate::day5::Iss;

use std::collections::VecDeque;

#[derive(Debug)]
enum Error {
    OutputNotProduced,
}

fn permute(
    cur: &mut Vec<i32>,
    rem: &mut VecDeque<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if rem.is_empty() {
        res.push(cur.clone());
    } else {
        for _ in 0..rem.len() {
            cur.push(rem.pop_front().unwrap());
            permute(cur, rem, res);
            rem.push_back(cur.pop().unwrap());
        }
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mem: Vec<i32> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let iss = crate::day5::Iss::new(mem.clone());

    let perms = {
        let mut rem = (0..=4).collect();
        let mut res = Vec::new();
        permute(&mut Vec::new(), &mut rem, &mut res);
        res
    };

    let mut signal_levels = Vec::new();
    for p in perms {
        let mut cores: Vec<Iss> = p
            .into_iter()
            .map(|phase| Iss::with_input(mem.clone(), vec![phase]))
            .collect();

        let mut v = 0;
        for c in 0..5 {
            cores[c].feed_input(v);
            v = *cores[c]
                .run()?
                .last()
                .ok_or_else(|| crate::Error::boxed(Error::OutputNotProduced))?;
        }
        signal_levels.push(v);
    }

    Ok(signal_levels.into_iter().max().unwrap())
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mem: Vec<i32> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let perms = {
        let mut rem = (5..=9).collect();
        let mut res = Vec::new();
        permute(&mut Vec::new(), &mut rem, &mut res);
        res
    };

    let mut signal_levels = Vec::new();
    for p in perms {
        let mut cores: Vec<Iss> = p
            .into_iter()
            .map(|phase| Iss::with_input(mem.clone(), vec![phase]))
            .collect();

        let mut c = 0;
        let mut v = 0;
        loop {
            cores[c].feed_input(v);
            if let Some(o) = cores[c].run_till_output()? {
                v = o;
            } else {
                break;
            }
            c = (c + 1) % 5;
        }
        signal_levels.push(v);
    }

    Ok(signal_levels.into_iter().max().unwrap())
}
