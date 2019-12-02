mod day1;
mod day2;

use std::{env, error, fmt, fs, process, result};

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

fn usage() -> ! {
    eprintln!("usage: aoc2019 <day> [<input>]");
    process::exit(1);
}

fn main() -> Result<()> {
    let (day, input) = {
        let mut args = env::args().skip(1);
        let d = if let Some(d) = args.next() {
            if let Ok(d) = d.parse() {
                d
            } else {
                eprintln!("Could not parse day: '{}'", d);
                usage();
            }
        } else {
            eprintln!("Not enough arguments");
            usage();
        };

        let i = args.next().unwrap_or_else(|| format!("input/day{}", d));
        let i = if let Ok(i) = fs::read_to_string(&i) {
            i
        } else {
            eprintln!("No such file: '{}'", &i);
            usage();
        };

        (d, i)
    };

    match day {
        1 => {
            println!("Part 1: {}", day1::part1(&input)?);
            println!("Part 2: {}", day1::part2(&input)?);
        }
        2 => {
            println!("Part 1: {}", day2::part1(input.trim())?);
            println!("Part 2: {}", day2::part2(input.trim())?);
        }
        _ => unimplemented!(),
    }

    Ok(())
}
