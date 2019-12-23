mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

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

        let i = args.next().unwrap_or_else(|| format!("input/day{:02}", d));
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
            println!("Part 1: {}", time(day01::part1, input.trim())?);
            println!("Part 2: {}", time(day01::part2, input.trim())?);
        }
        2 => {
            println!("Part 1: {}", time(day02::part1, input.trim())?);
            println!("Part 2: {}", time(day02::part2, input.trim())?);
        }
        3 => {
            println!("Part 1: {}", time(day03::part1, input.trim())?);
            println!("Part 2: {}", time(day03::part2, input.trim())?);
        }
        4 => {
            println!("Part 1: {}", time(day04::part1, input.trim())?);
            println!("Part 2: {}", time(day04::part2, input.trim())?);
        }
        5 => {
            println!("Part 1: {}", time(day05::part1, input.trim())?);
            println!("Part 2: {}", time(day05::part2, input.trim())?);
        }
        6 => {
            println!("Part 1: {}", time(day06::part1, input.trim())?);
            println!("Part 2: {}", time(day06::part2, input.trim())?);
        }
        7 => {
            println!("Part 1: {}", time(day07::part1, input.trim())?);
            println!("Part 2: {}", time(day07::part2, input.trim())?);
        }
        8 => {
            println!("Part 1: {}", time(day08::part1, input.trim())?);
            println!("Part 2: {}", time(day08::part2, input.trim())?);
        }
        9 => {
            println!("Part 1: {}", time(day09::part1, input.trim())?);
            println!("Part 2: {}", time(day09::part2, input.trim())?);
        }
        10 => {
            println!("Part 1: {}", time(day10::part1, input.trim())?);
            println!("Part 2: {}", time(day10::part2, input.trim())?);
        }
        11 => {
            println!("Part 1: {}", time(day11::part1, input.trim())?);
            println!("Part 2: {}", time(day11::part2, input.trim())?);
        }
        12 => {
            println!("Part 1: {}", time(day12::part1, input.trim())?);
            println!("Part 2: {}", time(day12::part2, input.trim())?);
        }
        13 => {
            println!("Part 1: {}", time(day13::part1, input.trim())?);
            println!("Part 2: {}", time(day13::part2, input.trim())?);
        }
        14 => {
            println!("Part 1: {}", time(day14::part1, input.trim())?);
            println!("Part 2: {}", time(day14::part2, input.trim())?);
        }
        15 => {
            println!("Part 1: {}", time(day15::part1, input.trim())?);
            println!("Part 2: {}", time(day15::part2, input.trim())?);
        }
        16 => {
            println!("Part 1: {}", time(day16::part1, input.trim())?);
            println!("Part 2: {}", time(day16::part2, input.trim())?);
        }
        17 => {
            println!("Part 1: {}", time(day17::part1, input.trim())?);
            println!("Part 2: {}", time(day17::part2, input.trim())?);
        }
        18 => {
            println!("Part 1: {}", time(day18::part1, input.trim())?);
            println!("Part 2: {}", time(day18::part2, input.trim())?);
        }
        19 => {
            println!("Part 1: {}", time(day19::part1, input.trim())?);
            println!("Part 2: {}", time(day19::part2, input.trim())?);
        }
        20 => {
            println!("Part 1: {}", time(day20::part1, &input)?);
            println!("Part 2: {}", time(day20::part2, &input)?);
        }
        21 => {
            println!("Part 1: {}", time(day21::part1, input.trim())?);
            println!("Part 2: {}", time(day21::part2, input.trim())?);
        }
        22 => {
            println!("Part 1: {}", time(day22::part1, input.trim())?);
            println!("Part 2: {}", time(day22::part2, input.trim())?);
        }
        _ => unimplemented!(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day01() {
        let inp = include_str!("../input/day01");
        assert_eq!(crate::day01::part1(inp.trim()).unwrap(), 3479429);
        assert_eq!(crate::day01::part2(inp.trim()).unwrap(), 5216273);
    }

    #[test]
    fn day02() {
        let inp = include_str!("../input/day02");
        assert_eq!(crate::day02::part1(inp.trim()).unwrap(), 3409710);
        assert_eq!(crate::day02::part2(inp.trim()).unwrap(), 7912);
    }

    #[test]
    fn day03() {
        let inp = include_str!("../input/day03");
        assert_eq!(crate::day03::part1(inp.trim()).unwrap(), 855);
        assert_eq!(crate::day03::part2(inp.trim()).unwrap(), 11238);
    }

    #[test]
    fn day04() {
        let inp = include_str!("../input/day04");
        assert_eq!(crate::day04::part1(inp.trim()).unwrap(), 921);
        assert_eq!(crate::day04::part2(inp.trim()).unwrap(), 603);
    }

    #[test]
    fn day05() {
        let inp = include_str!("../input/day05");
        assert_eq!(crate::day05::part1(inp.trim()).unwrap(), 9431221);
        assert_eq!(crate::day05::part2(inp.trim()).unwrap(), 1409363);
    }

    #[test]
    fn day06() {
        let inp = include_str!("../input/day06");
        assert_eq!(crate::day06::part1(inp.trim()).unwrap(), 249308);
        assert_eq!(crate::day06::part2(inp.trim()).unwrap(), 349);
    }

    #[test]
    fn day07() {
        let inp = include_str!("../input/day07");
        assert_eq!(crate::day07::part1(inp.trim()).unwrap(), 34852);
        assert_eq!(crate::day07::part2(inp.trim()).unwrap(), 44282086);
    }

    #[test]
    fn day08() {
        let inp = include_str!("../input/day08");
        let mut p2 = String::new();
        p2.push('\n');
        p2.push_str("  ##  ##  ###   ##  ###  \n");
        p2.push_str("   # #  # #  # #  # #  # \n");
        p2.push_str("   # #    #  # #    ###  \n");
        p2.push_str("   # #    ###  #    #  # \n");
        p2.push_str("#  # #  # # #  #  # #  # \n");
        p2.push_str(" ##   ##  #  #  ##  ###  \n");
        assert_eq!(crate::day08::part1(inp.trim()).unwrap(), 1474);
        assert_eq!(crate::day08::part2(inp.trim()).unwrap(), p2);
    }

    #[test]
    fn day09() {
        let inp = include_str!("../input/day09");
        assert_eq!(crate::day09::part1(inp.trim()).unwrap(), 3780860499);
        assert_eq!(crate::day09::part2(inp.trim()).unwrap(), 33343);
    }

    #[test]
    fn day10() {
        let inp = include_str!("../input/day10");
        assert_eq!(crate::day10::part1(inp.trim()).unwrap(), 230);
        assert_eq!(crate::day10::part2(inp.trim()).unwrap(), 1205);
    }

    #[test]
    fn day11() {
        let inp = include_str!("../input/day11");
        let mut p2 = String::new();
        p2.push('\n');
        p2.push_str(" ###   ##  #  # ###  #    ###  #  # #  #   \n");
        p2.push_str(" #  # #  # # #  #  # #    #  # #  # # #    \n");
        p2.push_str(" #  # #    ##   #  # #    #  # #  # ##     \n");
        p2.push_str(" ###  #    # #  ###  #    ###  #  # # #    \n");
        p2.push_str(" #    #  # # #  # #  #    #    #  # # #    \n");
        p2.push_str(" #     ##  #  # #  # #### #     ##  #  #   \n");
        assert_eq!(crate::day11::part1(inp.trim()).unwrap(), 2373);
        assert_eq!(crate::day11::part2(inp.trim()).unwrap(), p2);
    }

    #[test]
    fn day12() {
        let inp = include_str!("../input/day12");
        assert_eq!(crate::day12::part1(inp.trim()).unwrap(), 10055);
        assert_eq!(crate::day12::part2(inp.trim()).unwrap(), 374307970285176);
    }

    #[test]
    fn day13() {
        let inp = include_str!("../input/day13");
        assert_eq!(crate::day13::part1(inp.trim()).unwrap(), 200);
        assert_eq!(crate::day13::part2(inp.trim()).unwrap(), 9803);
    }

    #[test]
    fn day14() {
        let inp = include_str!("../input/day14");
        assert_eq!(crate::day14::part1(inp.trim()).unwrap(), 278404);
        assert_eq!(crate::day14::part2(inp.trim()).unwrap(), 4436981);
    }

    #[test]
    fn day15() {
        let inp = include_str!("../input/day15");
        assert_eq!(crate::day15::part1(inp.trim()).unwrap(), 366);
        assert_eq!(crate::day15::part2(inp.trim()).unwrap(), 384);
    }

    #[test]
    fn day16() {
        let inp = include_str!("../input/day16");
        assert_eq!(crate::day16::part1(inp.trim()).unwrap(), 69549155);
        assert_eq!(crate::day16::part2(inp.trim()).unwrap(), 83253465);
    }

    #[test]
    fn day17() {
        let inp = include_str!("../input/day17");
        assert_eq!(crate::day17::part1(inp.trim()).unwrap(), 3888);
        assert_eq!(crate::day17::part2(inp.trim()).unwrap(), 927809);
    }

    #[test]
    fn day18() {
        let inp = include_str!("../input/day18");
        assert_eq!(crate::day18::part1(inp.trim()).unwrap(), 4954);
        assert_eq!(crate::day18::part2(inp.trim()).unwrap(), 2334);
    }

    #[test]
    fn day19() {
        let inp = include_str!("../input/day19");
        assert_eq!(crate::day19::part1(inp.trim()).unwrap(), 110);
        assert_eq!(crate::day19::part2(inp.trim()).unwrap(), 17302065);
    }

    #[test]
    fn day20() {
        let inp = include_str!("../input/day20");
        assert_eq!(crate::day20::part1(inp).unwrap(), 548);
        assert_eq!(crate::day20::part2(inp).unwrap(), 6452);
    }

    #[test]
    fn day22() {
        let inp = include_str!("../input/day22");
        assert_eq!(crate::day22::part1(inp.trim()).unwrap(), 8191);
        assert_eq!(crate::day22::part2(inp.trim()).unwrap(), 1644352419829);
    }
}
