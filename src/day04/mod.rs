#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn num_to_vec(mut n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    while n != 0 {
        v.push(n % 10);
        n /= 10;
    }
    v
}

fn check(ns: &[u32]) -> bool {
    let mut found_double = false;
    for i in 0..ns.len() - 1 {
        if ns[i + 1] > ns[i] {
            return false;
        }
        if ns[i + 1] == ns[i] {
            found_double = true;
        }
    }
    found_double
}

fn check_p2(ns: &[u32]) -> bool {
    let mut found_double = false;
    let mut i = 0;
    while i < ns.len() - 1 {
        let mut j = i + 1;
        while j < ns.len() && ns[j] == ns[i] {
            j += 1;
        }

        if j - i == 2 {
            found_double = true;
        }

        if j < ns.len() && ns[j] > ns[i] {
            return false;
        }

        i = j;
    }
    found_double
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let (start, end): (u32, u32) = {
        let mut nums = input.split('-');
        (
            nums.next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?,
            nums.next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?,
        )
    };

    Ok((start..=end).map(num_to_vec).filter(|v| check(v)).count())
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let (start, end): (u32, u32) = {
        let mut nums = input.split('-');
        (
            nums.next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?,
            nums.next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?,
        )
    };

    Ok((start..=end).map(num_to_vec).filter(|v| check_p2(v)) .count())
}
