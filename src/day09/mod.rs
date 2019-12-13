use std::collections::{HashMap, VecDeque};
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
enum Error {
    IllegalInstruction,
    OutputNotProduced,
    NotEnoughInput,
}

pub type Value = i64;

#[derive(Debug, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<Value> for Mode {
    type Error = Box<crate::Error<Error>>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(crate::Error::boxed(Error::IllegalInstruction)),
        }
    }
}

#[derive(Debug)]
enum Insn {
    Add([Mode; 3]),
    Mul([Mode; 3]),
    In([Mode; 1]),
    Out([Mode; 1]),
    Jit([Mode; 2]),
    Jif([Mode; 2]),
    Lt([Mode; 3]),
    Equ([Mode; 3]),
    Rbo([Mode; 1]),
    Halt,
}

impl TryFrom<Value> for Insn {
    type Error = Box<crate::Error<Error>>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let opcode = value % 100;

        let (m1, m2, m3) = (
            Mode::try_from(value / 100 % 10)?,
            Mode::try_from(value / 1000 % 10)?,
            Mode::try_from(value / 10000 % 10)?,
        );

        match opcode {
            1 => Ok(Insn::Add([m1, m2, m3])),
            2 => Ok(Insn::Mul([m1, m2, m3])),
            3 => Ok(Insn::In([m1])),
            4 => Ok(Insn::Out([m1])),
            5 => Ok(Insn::Jit([m1, m2])),
            6 => Ok(Insn::Jif([m1, m2])),
            7 => Ok(Insn::Lt([m1, m2, m3])),
            8 => Ok(Insn::Equ([m1, m2, m3])),
            9 => Ok(Insn::Rbo([m1])),
            99 => Ok(Insn::Halt),
            _ => Err(crate::Error::boxed(Error::IllegalInstruction)),
        }
    }
}

#[derive(Clone)]
pub struct Iss {
    mem: Vec<Value>,
    mem0: HashMap<usize, Value>,
    pc: usize,
    rb: Value,
    input: VecDeque<Value>,
}

impl Iss {
    pub fn new(mem: Vec<Value>) -> Self {
        Iss {
            mem,
            mem0: HashMap::new(),
            pc: 0,
            rb: 0,
            input: VecDeque::new(),
        }
    }

    pub fn with_input(mem: Vec<Value>, input: Vec<Value>) -> Self {
        Iss {
            mem,
            mem0: HashMap::new(),
            pc: 0,
            rb: 0,
            input: input.into(),
        }
    }

    pub fn access(&mut self, addr: usize) -> &mut Value {
        if let Some(v) = self.mem.get_mut(addr) {
            v
        } else {
            self.mem0.entry(addr).or_insert(0)
        }
    }

    fn arg(&mut self, m: &[Mode], n: usize) -> crate::Result<&mut Value> {
        match m[n - 1] {
            Mode::Immediate => Ok(self.access(self.pc + n)),
            Mode::Position => {
                let val: usize = (*self.access(self.pc + n)).try_into()?;
                Ok(self.access(val))
            },
            Mode::Relative => {
                let val: i64 = *self.access(self.pc + n);
                Ok(self.access((val + self.rb).try_into()?))
            },
        }
    }

    pub fn feed_input(&mut self, i: i64) {
        self.input.push_back(i);
    }

    pub fn run_till_output(&mut self) -> crate::Result<Option<Value>> {
        loop {
            match Insn::try_from(*self.access(self.pc))? {
                Insn::Add(m) => {
                    *self.arg(&m, 3)? = *self.arg(&m, 1)? + *self.arg(&m, 2)?;
                    self.pc += 4;
                }
                Insn::Mul(m) => {
                    *self.arg(&m, 3)? = *self.arg(&m, 1)? * *self.arg(&m, 2)?;
                    self.pc += 4;
                }
                Insn::In(m) => {
                    *self.arg(&m, 1)? =
                        self.input.pop_front().ok_or_else(|| {
                            crate::Error::boxed(Error::NotEnoughInput)
                        })?;
                    self.pc += 2;
                }
                Insn::Out(m) => {
                    let o = *self.arg(&m, 1)?;
                    self.pc += 2;
                    return Ok(Some(o));
                }
                Insn::Jit(m) => {
                    if *self.arg(&m, 1)? != 0 {
                        self.pc = (*self.arg(&m, 2)?).try_into()?;
                    } else {
                        self.pc += 3;
                    }
                }
                Insn::Jif(m) => {
                    if *self.arg(&m, 1)? == 0 {
                        self.pc = (*self.arg(&m, 2)?).try_into()?;
                    } else {
                        self.pc += 3;
                    }
                }
                Insn::Lt(m) => {
                    *self.arg(&m, 3)? =
                        if *self.arg(&m, 1)? < *self.arg(&m, 2)? {
                            1
                        } else {
                            0
                        };
                    self.pc += 4;
                }
                Insn::Equ(m) => {
                    *self.arg(&m, 3)? =
                        if *self.arg(&m, 1)? == *self.arg(&m, 2)? {
                            1
                        } else {
                            0
                        };
                    self.pc += 4;
                }
                Insn::Rbo(m) => {
                    self.rb += *self.arg(&m, 1)?;
                    self.pc += 2;
                }
                Insn::Halt => return Ok(None),
            }
        }
    }

    pub fn run(&mut self) -> crate::Result<Vec<Value>> {
        let mut output = Vec::new();
        while let Some(o) = self.run_till_output()? {
            output.push(o);
        }
        Ok(output)
    }
}

pub fn part1(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let mut iss = Iss::with_input(mem, vec![1]);

    Ok(*iss
        .run()?
        .last()
        .ok_or_else(|| crate::Error::boxed(Error::OutputNotProduced))?)
}

pub fn part2(input: &str) -> crate::Result<Value> {
    let mem: Vec<Value> = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let mut iss = Iss::with_input(mem, vec![2]);

    Ok(*iss
        .run()?
        .last()
        .ok_or_else(|| crate::Error::boxed(Error::OutputNotProduced))?)
}
