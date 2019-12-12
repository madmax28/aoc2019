use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type Point = (i32, i32, i32);

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn attract_1d(p: i32, p_o: i32) -> i32 {
    if p < p_o {
        1
    } else if p > p_o {
        -1
    } else {
        0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Moon {
    p: Point,
    v: Point,
}

impl Moon {
    fn new(p: Point) -> Self {
        Moon { p, v: (0, 0, 0) }
    }

    fn attract(&mut self, p: Point) {
        self.v.0 += attract_1d(self.p.0, p.0);
        self.v.1 += attract_1d(self.p.1, p.1);
        self.v.2 += attract_1d(self.p.2, p.2);
    }

    fn mv(&mut self) {
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        self.p.2 += self.v.2;
    }

    fn energy(&self) -> i32 {
        (self.p.0.abs() + self.p.1.abs() + self.p.2.abs())
            * (self.v.0.abs() + self.v.1.abs() + self.v.2.abs())
    }
}

fn scan_for<I, T>(i: &mut I) -> crate::Result<T>
where
    I: Iterator<Item = char>,
    T: FromStr,
{
    Ok(i.skip_while(|&c| !c.is_digit(10) && c != '-')
        .take_while(|&c| c.is_digit(10) || c == '-')
        .collect::<String>()
        .parse()
        .map_err(|_| crate::Error::boxed(Error::InvalidInput))?)
}

fn gen_moons(input: &str) -> crate::Result<Vec<Moon>> {
    let mut moons = Vec::new();
    for l in input.lines() {
        let mut cs = l.chars();
        moons.push(Moon::new((
            scan_for(&mut cs)?,
            scan_for(&mut cs)?,
            scan_for(&mut cs)?,
        )));
    }
    Ok(moons)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mut moons: Vec<_> = gen_moons(input)?;

    for _ in 0..1000 {
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                let pi = moons[i].p;
                let pj = moons[j].p;
                moons[i].attract(pj);
                moons[j].attract(pi);
            }

            moons[i].mv();
        }
    }

    Ok(moons.iter().map(|m| m.energy()).sum())
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let moons: Vec<_> = gen_moons(input)?;

    let mut moons_1d = vec![Vec::new(); 3];
    for m in moons {
        moons_1d[0].push((m.p.0, m.v.0));
        moons_1d[1].push((m.p.1, m.v.1));
        moons_1d[2].push((m.p.2, m.v.2));
    }

    let rec_lens: Vec<_> = moons_1d
        .into_iter()
        .map(|moons| {
            let mut ms = moons.clone();

            let mut n = 0;
            loop {
                n += 1;

                for i in 0..ms.len() {
                    for j in i + 1..ms.len() {
                        ms[i].1 += attract_1d(ms[i].0, ms[j].0);
                        ms[j].1 += attract_1d(ms[j].0, ms[i].0);
                    }

                    ms[i].0 += ms[i].1;
                }

                if ms == moons {
                    break;
                }
            }

            n
        })
        .collect();

    Ok(rec_lens.into_iter().fold(1, |acc, n| acc * n / gcd(acc, n)))
}
