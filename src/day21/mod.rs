use crate::day09::{Iss, StopReason, Value};

use std::convert::TryFrom;

#[derive(Debug)]
enum Error {
    NoOutput,
}

pub fn part1(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let mut iss = Iss::new(mem);

    /*
     * J = !A || !B || !C && D
     */
    iss.feed_str("NOT A J\n")?;

    iss.feed_str("NOT B T\n")?;
    iss.feed_str("OR T J\n")?;

    iss.feed_str("NOT C T\n")?;
    iss.feed_str("OR T J\n")?;

    iss.feed_str("AND D J\n")?;

    iss.feed_str("WALK\n")?;

    while let StopReason::Output(o) = iss.run()? {
        if u8::try_from(o).is_err() {
            return Ok(o);
        }
    }

    Err(crate::Error::boxed(Error::NoOutput))
}

pub fn part2(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let mut iss = Iss::new(mem);

    /*
     * @
     * #_???????? -> jump
     *  ABCDEFGHI
     *  (1) = !A
     *
     * @
     * #?_?#????? -> jump
     *  ABCDEFGHI
     *  (2) = !B & D
     *
     * @
     * #??_##???? -> jump
     *  ABCDEFGHI
     *  (3) = !C & D & E
     *
     * @
     * #??_#?_??? -> jump
     *  ABCDEFGHI
     *  (4) = !C & D & !F
     *
     * @
     * #??_#??_#? -> jump
     *  ABCDEFGHI
     *  (4) = !C & D & !G & H
     *
     *  (1) | (2) | (3) | (4)
     *  = !A | (!B & D) | (!C & D & (E | !F | (!G & H)))
     *  = !(A & (B | !D) & (C | !D | (!E & F & (G | !H)))
     *      (5) |    (4)   |   (3)   |   (2)   | (1)
     */

    // (1)
    iss.feed_str("NOT H J\n")?;
    iss.feed_str("OR G J\n")?;

    // (2)
    iss.feed_str("AND F J\n")?;
    iss.feed_str("NOT E T\n")?;
    iss.feed_str("AND T J\n")?;

    // (3)
    iss.feed_str("NOT D T\n")?;
    iss.feed_str("OR T J\n")?;
    iss.feed_str("OR C J\n")?;

    // (4)
    iss.feed_str("NOT D T\n")?;
    iss.feed_str("OR B T\n")?;
    iss.feed_str("AND T J\n")?;

    // (5)
    iss.feed_str("AND A J\n")?;
    iss.feed_str("NOT J J\n")?;

    iss.feed_str("RUN\n")?;

    while let StopReason::Output(o) = iss.run()? {
        if u8::try_from(o).is_err() {
            return Ok(o);
        }
    }

    Err(crate::Error::boxed(Error::NoOutput))
}
