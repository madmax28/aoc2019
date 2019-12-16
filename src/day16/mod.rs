#[derive(Debug)]
enum Error {
    InvalidInput,
}

const BASE: &[i32] = &[0, 1, 0, -1];

fn fft(values: &mut Vec<i32>, skip: usize) {
    let init = values.clone();
    let total_len = init.len() + skip;

    for idx in 0..init.len() {
        let phase = skip + idx;

        if phase <= total_len / 3 {
            values[idx] = init[idx..]
                .iter()
                .enumerate()
                .map(|(idx, &val)| {
                    val * BASE[(idx / (phase + 1) + 1) % BASE.len()]
                })
                .sum::<i32>()
                .abs()
                % 10;
        } else {
            values[idx] = &init[idx..]
                .iter()
                .take((phase + 1).min(total_len - phase))
                .sum::<i32>()
                % 10;
        }
    }
}

fn parse(input: &str) -> crate::Result<Vec<i32>> {
    input
        .lines()
        .next()
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
        .chars()
        .map(|c| -> crate::Result<i32> {
            Ok(c.to_digit(10)
                .map(|n| n as i32)
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?)
        })
        .collect::<Result<_, _>>()
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mut nums = parse(input)?;

    for _ in 0..100 {
        fft(&mut nums, 0);
    }

    Ok(nums.iter().take(8).fold(0, |acc, n| acc * 10 + n))
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let nums = parse(input)?;
    let offset = nums.iter().take(7).fold(0, |acc, n| acc * 10 + n) as usize;

    let mut nums: Vec<i32> = {
        let len = nums.len() * 10_000 - offset;
        nums.into_iter().cycle().skip(offset).take(len).collect()
    };

    for _ in 0..100 {
        // Solves in 4976.325 008 743 seconds \o/
        // fft(&mut nums, offset)

        for idx in (0..nums.len() - 1).rev() {
            nums[idx] = (nums[idx] + nums[idx + 1]) % 10;
        }
    }

    Ok(nums.iter().take(8).fold(0, |acc, n| acc * 10 + n))
}
