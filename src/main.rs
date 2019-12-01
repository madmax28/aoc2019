mod day1;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn usage() -> ! {
    eprintln!("usage: aoc2019 <day> [<input>]");
    std::process::exit(1);
}

fn main() -> Result<()> {
    let (day, input) = {
        let mut args = std::env::args().skip(1);
        let d: u32 =
            args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
                usage();
            });
        let i = std::fs::read_to_string(
            args.next().unwrap_or(format!("input/day{}", d).to_string()),
        )?;
        (d, i)
    };

    match day {
        1 => {
            println!("Part 1: {}", day1::part1(&input)?);
            println!("Part 2: {}", day1::part2(&input)?);
        }
        _ => usage(),
    }

    Ok(())
}
