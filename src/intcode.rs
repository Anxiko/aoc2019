use anyhow::{Context, anyhow};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
enum OpCode {
    Add = 1,
    Mul = 2,
    Halt = 99,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct IntMachine {
    mem: Vec<u32>,
    pc: usize,
    halted: bool,
}

impl IntMachine {
    pub(crate) fn new(mem: Vec<u32>) -> Self {
        Self {
            mem,
            pc: 0,
            halted: false,
        }
    }

    pub(crate) fn is_halted(&self) -> bool {
        self.halted
    }

    pub(crate) fn step(&mut self) -> Result<(), anyhow::Error> {
        if self.is_halted() {
            anyhow::bail!("Attempted to run a halted machine")
        }

        let opcode: OpCode = self.read_pc().and_then(|value| {
            OpCode::try_from(value).map_err(|_| anyhow!("Unknown opcode {}", value))
        })?;

        self.execute(opcode)
    }

    pub(crate) fn run(&mut self) -> Result<u32, anyhow::Error> {
        while !self.halted {
            self.step()?;
        }

        self.read(0)
    }

    pub(crate) fn read(&mut self, address: usize) -> Result<u32, anyhow::Error> {
        self.mem
            .get(address)
            .ok_or_else(|| anyhow!("Failed to read at address={}, out of bounds", address))
            .copied()
    }

    fn read_pc(&mut self) -> Result<u32, anyhow::Error> {
        let result = self.read(self.pc).with_context(|| "Failed to read from PC");

        if result.is_ok() {
            self.pc += 1;
        }

        result
    }

    pub(crate) fn write(&mut self, index: usize, value: u32) -> Result<(), anyhow::Error> {
        let mem_ref = self
            .mem
            .get_mut(index)
            .ok_or_else(|| anyhow!("Failed to write at"))?;
        *mem_ref = value;
        Ok(())
    }

    fn execute_calculation(
        &mut self,
        formula: Box<dyn FnOnce(u32, u32) -> u32>,
    ) -> Result<(), anyhow::Error> {
        let left_ptr = self.read_pc()?;
        let left_value = self.read(left_ptr as usize)?;

        let right_ptr = self.read_pc()?;
        let right_value = self.read(right_ptr as usize)?;

        let dst_ptr = self.read_pc()?;

        let result = formula(left_value, right_value);

        self.write(dst_ptr as usize, result)
    }

    fn halt(&mut self) {
        self.halted = true;
    }

    fn execute(&mut self, op: OpCode) -> Result<(), anyhow::Error> {
        match op {
            OpCode::Add => self.execute_calculation(Box::new(|x, y| x + y)),
            OpCode::Mul => self.execute_calculation(Box::new(|x, y| x * y)),
            OpCode::Halt => {
                self.halt();
                Ok(())
            }
        }
    }
}
