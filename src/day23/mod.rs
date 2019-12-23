use crate::day09::{Iss, Value};

use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug)]
enum Error {
    UnexpectedIssResult,
    InvalidAddress,
}

#[derive(Debug)]
struct Packet {
    address: Value,
    x: Value,
    y: Value,
}

fn run(iss: &mut Iss) -> crate::Result<Option<Packet>> {
    use crate::day09::StopReason::*;
    match (iss.run()?, iss.run()?, iss.run()?) {
        (Output(address), Output(x), Output(y)) => {
            Ok(Some(Packet { address, x, y }))
        }
        (OutOfInput, _, _) => Ok(None),
        _ => Err(crate::Error::boxed(Error::UnexpectedIssResult)),
    }
}

pub fn part1(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let initial = Iss::new(mem);

    let mut vms: Vec<Iss> = (0..50)
        .map(|address| {
            let mut vm = initial.clone();
            vm.feed_input(address);
            vm
        })
        .collect();

    for address in (0..50).cycle() {
        vms[address].feed_input(-1);

        while let Some(pkt) = run(&mut vms[address])? {
            if pkt.address == 255 {
                return Ok(pkt.y);
            }

            let tgt: &mut Iss = vms
                .get_mut(usize::try_from(pkt.address)?)
                .ok_or_else(|| crate::Error::boxed(Error::InvalidAddress))?;
            tgt.feed_input(pkt.x);
            tgt.feed_input(pkt.y);
        }
    }

    unreachable!();
}

#[derive(Debug)]
struct Nat {
    x: Value,
    y: Value,
}

pub fn part2(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let initial = Iss::new(mem);

    let mut vms: Vec<Iss> = (0..50)
        .map(|address| {
            let mut vm = initial.clone();
            vm.feed_input(address);
            vm
        })
        .collect();

    let mut nat = Nat { x: 0, y: 0 };
    let mut idle_iter_cnt = 0;
    let mut ys_sent = HashSet::new();
    for address in (0..50).cycle() {
        vms[address].feed_input(-1);

        let mut got_packet = false;
        while let Some(pkt) = run(&mut vms[address])? {
            got_packet = true;

            if pkt.address == 255 {
                nat.x = pkt.x;
                nat.y = pkt.y;
            } else {
                let tgt: &mut Iss =
                    vms.get_mut(usize::try_from(pkt.address)?).ok_or_else(
                        || crate::Error::boxed(Error::InvalidAddress),
                    )?;
                tgt.feed_input(pkt.x);
                tgt.feed_input(pkt.y);
            }
        }

        if got_packet {
            idle_iter_cnt = 0;
        } else {
            idle_iter_cnt += 1;
        }

        if idle_iter_cnt == 50 {
            if !ys_sent.insert(nat.y) {
                return Ok(nat.y);
            }

            vms[0].feed_input(nat.x);
            vms[0].feed_input(nat.y);
        }
    }

    unreachable!();
}
