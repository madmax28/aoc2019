mod day1;
mod day2;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn usage() -> ! {
    eprintln!("usage: aoc2019 <day> [<input>]");
    std::process::exit(1);
}

fn main() -> Result<()> {
    let (day, input) = {
        let mut args = std::env::args().skip(1);
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
        let i = if let Ok(i) = std::fs::read_to_string(&i) {
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
