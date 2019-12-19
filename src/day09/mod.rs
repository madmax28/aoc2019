use std::cell::RefCell;
use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};
use std::rc::Rc;

#[derive(Debug)]
pub enum Error {
    IllegalInstruction,
    OutputNotProduced,
}

pub type Value = i64;

#[derive(Debug, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<usize> for Mode {
    type Error = Box<crate::Error<Error>>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(crate::Error::boxed(Error::IllegalInstruction)),
        }
    }
}

const MAX_INSN_VALUE: usize = 33399;

#[derive(Debug, Clone, Copy)]
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

impl TryFrom<usize> for Insn {
    type Error = Box<crate::Error<Error>>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
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
struct Icache {
    cache: Rc<RefCell<[Option<Insn>; MAX_INSN_VALUE + 1]>>,
}

impl Icache {
    fn new() -> Self {
        Icache {
            cache: Rc::new(RefCell::new([None; MAX_INSN_VALUE + 1])),
        }
    }

    fn fetch_insn(&self, value: Value) -> crate::Result<Insn> {
        if value < 0 || value as usize > MAX_INSN_VALUE {
            println!("{}", value);
            return Err(crate::Error::boxed(Error::IllegalInstruction));
        }
        let value = value as usize;

        let mut cache = self.cache.borrow_mut();
        if let Some(insn) = &cache[value] {
            Ok(*insn)
        } else {
            let insn = Insn::try_from(value)?;
            cache[value] = Some(insn);
            Ok(insn)
        }
    }
}

#[derive(Debug)]
pub enum StopReason {
    Output(Value),
    OutOfInput,
    Halted,
}

#[derive(Clone)]
pub struct Iss {
    mem: Vec<Value>,
    pc: usize,
    rb: Value,
    input: VecDeque<Value>,
    icache: Icache,
}

impl Iss {
    pub fn new(mem: Vec<Value>) -> Self {
        Iss {
            mem,
            pc: 0,
            rb: 0,
            input: VecDeque::new(),
            icache: Icache::new(),
        }
    }

    pub fn with_input(mem: Vec<Value>, input: Vec<Value>) -> Self {
        Iss {
            mem,
            pc: 0,
            rb: 0,
            input: input.into(),
            icache: Icache::new(),
        }
    }

    pub fn access(&mut self, addr: usize) -> &mut Value {
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        &mut self.mem[addr]
    }

    fn arg(&mut self, m: &[Mode], n: usize) -> crate::Result<&mut Value> {
        match m[n - 1] {
            Mode::Immediate => Ok(self.access(self.pc + n)),
            Mode::Position => {
                let val = (*self.access(self.pc + n)) as usize;
                Ok(self.access(val))
            }
            Mode::Relative => {
                let val: Value = *self.access(self.pc + n);
                Ok(self.access((val + self.rb) as usize))
            }
        }
    }

    pub fn feed_str(&mut self, s: &str) -> crate::Result<()> {
        for c in s.chars() {
            self.input.push_back(u32::from(c).try_into()?);
        }
        Ok(())
    }

    pub fn feed_input(&mut self, i: Value) {
        self.input.push_back(i);
    }

    pub fn run(&mut self) -> crate::Result<StopReason> {
        loop {
            let val = *self.access(self.pc);
            match self.icache.fetch_insn(val)? {
                Insn::Add(m) => {
                    *self.arg(&m, 3)? = *self.arg(&m, 1)? + *self.arg(&m, 2)?;
                    self.pc += 4;
                }
                Insn::Mul(m) => {
                    *self.arg(&m, 3)? = *self.arg(&m, 1)? * *self.arg(&m, 2)?;
                    self.pc += 4;
                }
                Insn::In(m) => {
                    *self.arg(&m, 1)? = if let Some(i) = self.input.pop_front()
                    {
                        i
                    } else {
                        return Ok(StopReason::OutOfInput);
                    };
                    self.pc += 2;
                }
                Insn::Out(m) => {
                    let o = *self.arg(&m, 1)?;
                    self.pc += 2;
                    return Ok(StopReason::Output(o));
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
                Insn::Halt => return Ok(StopReason::Halted),
            }
        }
    }

    fn run_continuous(&mut self) -> crate::Result<Vec<Value>> {
        let mut output = Vec::new();
        while let StopReason::Output(o) = self.run()? {
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
        .run_continuous()?
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
        .run_continuous()?
        .last()
        .ok_or_else(|| crate::Error::boxed(Error::OutputNotProduced))?)
}
