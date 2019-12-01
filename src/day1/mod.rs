fn calc_total_fuel(m: i32) -> i32 {
    let fuel = m / 3 - 2;
    if fuel < 0 {
        0
    } else {
        fuel + calc_total_fuel(fuel)
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    input
        .lines()
        .map(|l| l.parse::<i32>().and_then(|m| Ok(m / 3 - 2)))
        .try_fold(0, |acc, r| Ok(acc + r?))
}

pub fn part2(input: &str) -> crate::Result<i32> {
    input
        .lines()
        .map(|l| l.parse::<i32>().and_then(|m| Ok(calc_total_fuel(m))))
        .try_fold(0, |acc, r| Ok(acc + r?))
}
