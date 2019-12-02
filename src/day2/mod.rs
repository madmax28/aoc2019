use std::{error, fmt};

#[derive(Debug)]
enum Error {
    IllegalInstruction,
    AddressOutOfRange,
    OutputNotProduced,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}

fn access(mem: &mut [usize], addr: usize) -> crate::Result<&mut usize> {
    Ok(mem
        .get_mut(addr)
        .ok_or_else(|| Box::new(Error::AddressOutOfRange))?)
}

fn run(mem: &mut [usize]) -> crate::Result<()> {
    let mut pc = 0;

    while *access(mem, pc)? != 99 {
        let (a1, a2, a3) = (
            *access(mem, pc + 1)?,
            *access(mem, pc + 2)?,
            *access(mem, pc + 3)?,
        );

        match *access(mem, pc)? {
            1 => *access(mem, a3)? = *access(mem, a1)? + *access(mem, a2)?,
            2 => *access(mem, a3)? = *access(mem, a1)? * *access(mem, a2)?,
            _ => return Err(Box::new(Error::IllegalInstruction)),
        }

        pc += 4
    }

    Ok(())
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let mut mem: Vec<usize> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    *access(&mut mem, 1)? = 12;
    *access(&mut mem, 2)? = 2;

    run(&mut mem)?;

    Ok(*access(&mut mem, 0)?)
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let mem: Vec<usize> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let output = 19_690_720;

    for i1 in 0..100 {
        for i2 in 0..100 {
            let mut mem = mem.clone();

            *access(&mut mem, 1)? = i1;
            *access(&mut mem, 2)? = i2;

            run(&mut mem)?;

            if *access(&mut mem, 0)? == output {
                return Ok(100 * i1 + i2);
            }
        }
    }

    Err(Box::new(Error::OutputNotProduced))
}
