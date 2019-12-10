use crate::day05::Iss;

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

    let mut perms = Vec::new();
    permute(&mut Vec::new(), &mut (0..=4).collect(), &mut perms);

    let mut signal_levels = Vec::new();
    for p in perms {
        let mut v = 0;
        for mut core in p
            .into_iter()
            .map(|phase| Iss::with_input(mem.clone(), vec![phase]))
        {
            core.feed_input(v);
            v = *core
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

    let mut perms = Vec::new();
    permute(&mut Vec::new(), &mut (5..=9).collect(), &mut perms);

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
