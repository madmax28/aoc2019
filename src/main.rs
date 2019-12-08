mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::{env, error, fmt, fs, result};

#[derive(Debug)]
struct UsageError;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct Error<T> {
    err: T,
}

impl<T> Error<T> {
    fn boxed(err: T) -> Box<Self> {
        Box::new(Self { err })
    }
}

impl<T: fmt::Debug> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.err)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}

fn time<F: Fn(A) -> B, A, B>(f: F, a: A) -> B {
    let now = std::time::Instant::now();
    let res = f(a);
    let d = now.elapsed();
    println!(
        "> {}.{:03} {:03} {:03} seconds",
        d.as_secs(),
        d.subsec_millis(),
        d.subsec_micros() % 1_000,
        d.subsec_nanos() % 1_000,
    );
    res
}

fn usage() -> Result<()> {
    eprintln!("usage: aoc2019 <day> [<input>]");
    Err(Error::boxed(UsageError {}))
}

fn main() -> Result<()> {
    let (day, input) = {
        let mut args = env::args().skip(1);
        let d = if let Some(d) = args.next() {
            if let Ok(d) = d.parse() {
                d
            } else {
                eprintln!("Could not parse day: '{}'", d);
                return usage();
            }
        } else {
            eprintln!("Not enough arguments");
            return usage();
        };

        let i = args.next().unwrap_or_else(|| format!("input/day{}", d));
        let i = if let Ok(i) = fs::read_to_string(&i) {
            i
        } else {
            eprintln!("No such file: '{}'", &i);
            return usage();
        };

        (d, i)
    };

    match day {
        1 => {
            println!("Part 1: {}", time(day1::part1, input.trim())?);
            println!("Part 2: {}", time(day1::part2, input.trim())?);
        },
        2 => {
            println!("Part 1: {}", time(day2::part1, input.trim())?);
            println!("Part 2: {}", time(day2::part2, input.trim())?);
        },
        3 => {
            println!("Part 1: {}", time(day3::part1, input.trim())?);
            println!("Part 2: {}", time(day3::part2, input.trim())?);
        },
        4 => {
            println!("Part 1: {}", time(day4::part1, input.trim())?);
            println!("Part 2: {}", time(day4::part2, input.trim())?);
        },
        5 => {
            println!("Part 1: {}", time(day5::part1, input.trim())?);
            println!("Part 2: {}", time(day5::part2, input.trim())?);
        },
        6 => {
            println!("Part 1: {}", time(day6::part1, input.trim())?);
            println!("Part 2: {}", time(day6::part2, input.trim())?);
        },
        7 => {
            println!("Part 1: {}", time(day7::part1, input.trim())?);
            println!("Part 2: {}", time(day7::part2, input.trim())?);
        },
        8 => {
            println!("Part 1: {}", time(day8::part1, input.trim())?);
            println!("Part 2: {}", time(day8::part2, input.trim())?);
        },
        _ => unimplemented!(),
    }

    Ok(())
}
