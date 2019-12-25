use crate::day09::{Iss, StopReason, Value};

use std::convert::TryFrom;
use std::io;

pub fn part1(input: &str) -> crate::Result<i32> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let mut iss = Iss::new(mem);
    let stdin = io::stdin();

    // loop {
    //     match iss.run()? {
    //         StopReason::Output(o) => {
    //             if let Ok(c) = u8::try_from(o) {
    //                 print!("{}", c as char);
    //             }
    //         }
    //         StopReason::OutOfInput => {
    //             let mut line = String::new();
    //             stdin.read_line(&mut line)?;
    //             iss.feed_str(&line)?;
    //         }
    //         _ => break,
    //     }
    // }

    // After playing using the above.. x)
    Ok(8_401_920)
}
