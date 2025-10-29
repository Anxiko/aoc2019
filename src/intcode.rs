use crate::types::IntCell;
use anyhow::{Context, anyhow};
use itertools::Itertools;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
enum OpCode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    RelativeBaseOffset = 9,
    Halt = 99,
}

impl OpCode {
    fn number_arguments(&self) -> usize {
        match self {
            Self::Add | Self::Mul => 3,
            Self::Input | Self::Output => 1,
            Self::JumpIfTrue | Self::JumpIfFalse => 2,
            Self::LessThan | Self::Equals => 3,
            Self::RelativeBaseOffset => 1,
            Self::Halt => 0,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Add => "add",
            Self::Mul => "mul",
            Self::Input => "in",
            Self::Output => "out",
            Self::JumpIfTrue => "jumpt",
            Self::JumpIfFalse => "jumpf",
            Self::LessThan => "lt",
            Self::Equals => "eq",
            Self::RelativeBaseOffset => "rel",
            Self::Halt => "halt",
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum OperandMode {
    Indirect = 0,
    Direct = 1,
    Relative = 2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Instruction {
    code: OpCode,
    operand_modes: [OperandMode; 3],
}

impl Instruction {
    fn extract_operand_mode(value: IntCell, idx: u32) -> anyhow::Result<OperandMode> {
        let raw = (value / ((10 as IntCell).pow(idx))) % 10;
        let operand = OperandMode::try_from(raw as u8)?;
        Ok(operand)
    }
}

impl TryFrom<IntCell> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: IntCell) -> Result<Self, Self::Error> {
        let code = OpCode::try_from((value % 100) as u32)?;
        let parameter_flags = value / 100;
        let operand_modes = [
            Self::extract_operand_mode(parameter_flags, 0)?,
            Self::extract_operand_mode(parameter_flags, 1)?,
            Self::extract_operand_mode(parameter_flags, 2)?,
        ];

        Ok(Self {
            code,
            operand_modes,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operand {
    Direct(IntCell),
    Indirect(IntCell),
    Relative(IntCell),
}

impl Operand {
    fn new(value: IntCell, operand_mode: OperandMode) -> Self {
        match operand_mode {
            OperandMode::Indirect => Self::Indirect(value),
            OperandMode::Direct => Self::Direct(value),
            OperandMode::Relative => Self::Relative(value),
        }
    }

    fn argument(&self) -> IntCell {
        match *self {
            Self::Direct(value) => value,
            Self::Indirect(value) => value,
            Self::Relative(value) => value,
        }
    }

    fn read(&self, machine: &mut IntMachine) -> anyhow::Result<IntCell> {
        match self {
            Self::Direct(value) => Ok(*value),
            Self::Indirect(ptr) => {
                let address = usize::try_from(*ptr)
                    .map_err(|_| anyhow::anyhow!("Can't convert ptr={ptr} to address"))?;
                machine.read(address)
            }
            Self::Relative(value) => {
                let address = machine.relative_base + value;
                let address = usize::try_from(address)
                    .map_err(|_| anyhow::anyhow!("Can't convert address={address}"))?;
                machine.read(address)
            }
        }
    }

    fn as_address(&self, machine: &IntMachine) -> anyhow::Result<IntCell> {
        match *self {
            Self::Direct(_) => Err(anyhow::anyhow!("Can't treat a direct operand as pointer")),
            Self::Indirect(value) => Ok(value),
            Self::Relative(value) => Ok(value + machine.relative_base),
        }
    }

    fn write(&self, machine: &mut IntMachine, value: IntCell) -> anyhow::Result<()> {
        let address = self.as_address(machine)?;
        let address = usize::try_from(address)?;

        machine.write(address, value)
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Indirect(_) => {
                write!(f, "*")?;
            }
            Operand::Relative(_) => {
                write!(f, "+")?;
            }
            Operand::Direct(_) => {}
        }

        write!(f, "{}", self.argument())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ExecutableInstruction {
    instruction: Instruction,
    operands: Vec<Operand>,
}

impl ExecutableInstruction {
    fn new(instruction: Instruction, operands: Vec<Operand>) -> Self {
        Self {
            instruction,
            operands,
        }
    }

    fn decode(machine: &mut IntMachine) -> anyhow::Result<Self> {
        let instruction = machine
            .read_pc()
            .with_context(|| "Failed to read instruction")?;
        let instruction =
            Instruction::try_from(instruction).with_context(|| "Failed to parse instruction")?;

        let n_arguments = instruction.code.number_arguments();
        let arguments: Vec<_> = std::iter::repeat_with(|| machine.read_pc())
            .take(n_arguments)
            .collect::<Result<_, _>>()
            .with_context(|| "Failed to parse arguments")?;

        if arguments.len() != n_arguments {
            anyhow::bail!(
                "Expected {} arguments, got {}",
                n_arguments,
                arguments.len()
            );
        }

        let operands = arguments
            .into_iter()
            .zip(instruction.operand_modes.iter())
            .map(|(argument, &direct_flag)| Operand::new(argument, direct_flag))
            .collect_vec();

        Ok(Self::new(instruction, operands))
    }

    fn execute_calculation(
        &self,
        machine: &mut IntMachine,
        formula: Box<dyn FnOnce(IntCell, IntCell) -> IntCell>,
    ) -> anyhow::Result<()> {
        let [lhs, rhs, dst_ptr]: [Operand; 3] =
            self.operands
                .as_slice()
                .try_into()
                .map_err(|invalid_operands| {
                    anyhow::anyhow!(
                        "Unexpected number of arguments for operation: {invalid_operands:?}"
                    )
                })?;

        let left_value = lhs.read(machine)?;
        let right_value = rhs.read(machine)?;

        let result = formula(left_value, right_value);

        dst_ptr.write(machine, result)?;

        Ok(())
    }

    fn execute_jump(
        &self,
        machine: &mut IntMachine,
        predicate: Box<dyn FnOnce(IntCell) -> bool>,
    ) -> anyhow::Result<()> {
        let [test_operand, address_ptr]: [Operand; 2] = self.operands.as_slice().try_into()?;
        let test_value = test_operand.read(machine)?;

        if predicate(test_value) {
            let address = address_ptr.read(machine)?;
            machine.write_pc(address)?;
        }
        Ok(())
    }

    fn execute_comparison(
        &self,
        machine: &mut IntMachine,
        comparison: Box<dyn FnOnce(IntCell, IntCell) -> bool>,
    ) -> anyhow::Result<()> {
        let [lhs, rhs, dst_ptr]: [Operand; 3] = self.operands.as_slice().try_into()?;

        let left_value = lhs.read(machine)?;
        let right_value = rhs.read(machine)?;

        let result: IntCell = comparison(left_value, right_value).into();
        dst_ptr.write(machine, result)?;

        Ok(())
    }

    fn execute(&self, machine: &mut IntMachine) -> Result<(), anyhow::Error> {
        match self.instruction.code {
            OpCode::Add => self.execute_calculation(machine, Box::new(|x, y| x + y)),
            OpCode::Mul => self.execute_calculation(machine, Box::new(|x, y| x * y)),
            OpCode::Input => {
                let [dst_ptr]: [Operand; 1] = self.operands.as_slice().try_into()?;

                let value = machine.read_input()?;

                dst_ptr.write(machine, value)?;

                Ok(())
            }
            OpCode::Output => {
                let [src]: [Operand; 1] = self.operands.as_slice().try_into()?;
                let value = src.read(machine)?;

                machine.write_output(value);
                Ok(())
            }
            OpCode::JumpIfTrue => self.execute_jump(machine, Box::new(|value| value != 0)),
            OpCode::JumpIfFalse => self.execute_jump(machine, Box::new(|value| value == 0)),
            OpCode::LessThan => self.execute_comparison(machine, Box::new(|x, y| x < y)),
            OpCode::Equals => self.execute_comparison(machine, Box::new(|x, y| x == y)),
            OpCode::RelativeBaseOffset => {
                let [delta]: [Operand; 1] = self.operands.as_slice().try_into()?;
                let delta = delta.read(machine)?;

                machine.delta_relative_base(delta);

                Ok(())
            }
            OpCode::Halt => {
                machine.halt();
                Ok(())
            }
        }
    }
}

impl Display for ExecutableInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.instruction.code)?;
        let operands = self.operands.iter().map(ToString::to_string).join(" ");
        write!(f, " {operands}")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct IntMachine {
    mem: Vec<IntCell>,
    pc: usize,
    relative_base: IntCell,
    halted: bool,
    input: VecDeque<IntCell>,
    output: Vec<IntCell>,
}

const MIN_MEM_SIZE: usize = 10 * 1024;

impl IntMachine {
    pub(crate) fn new(mut mem: Vec<IntCell>) -> Self {
        if mem.len() < MIN_MEM_SIZE {
            mem.resize(MIN_MEM_SIZE, 0);
        }

        Self {
            mem,
            pc: 0,
            relative_base: 0,
            halted: false,
            input: Default::default(),
            output: Default::default(),
        }
    }

    pub(crate) fn with_input(&mut self, input: VecDeque<IntCell>) {
        self.input = input;
    }

    pub(crate) fn is_halted(&self) -> bool {
        self.halted
    }

    pub(crate) fn step(&mut self) -> Result<(), anyhow::Error> {
        if self.is_halted() {
            anyhow::bail!("Attempted to run a halted machine")
        }

        let executable_instruction = ExecutableInstruction::decode(self)?;

        executable_instruction.execute(self)
    }

    pub(crate) fn run(&mut self) -> Result<IntCell, anyhow::Error> {
        while !self.halted {
            self.step()?;
        }

        self.read(0)
    }

    pub(crate) fn run_until_input(&mut self) -> Result<(), anyhow::Error> {
        while !self.halted && !self.will_run_out_of_input()? {
            self.step()?;
        }

        Ok(())
    }

    pub(crate) fn run_until_output(&mut self) -> Result<IntCell, anyhow::Error> {
        while !self.halted {
            if let Ok(output) = self.pop_output() {
                return Ok(output);
            }

            self.step()?;
        }

        Err(anyhow::anyhow!("Machine halted before producing output"))
    }

    fn peek_instruction(&self) -> anyhow::Result<Instruction> {
        let instruction = self.read(self.pc)?;
        instruction.try_into()
    }

    fn will_run_out_of_input(&self) -> anyhow::Result<bool> {
        Ok(self.peek_instruction()?.code == OpCode::Input && self.input.is_empty())
    }

    pub(crate) fn read(&self, address: usize) -> Result<IntCell, anyhow::Error> {
        self.mem
            .get(address)
            .ok_or_else(|| anyhow!("Failed to read at address={}, out of bounds", address))
            .copied()
    }

    fn read_pc(&mut self) -> Result<IntCell, anyhow::Error> {
        let result = self.read(self.pc).with_context(|| "Failed to read from PC");

        if result.is_ok() {
            self.pc += 1;
        }

        result
    }

    pub(crate) fn write(&mut self, address: usize, value: IntCell) -> Result<(), anyhow::Error> {
        let mem_ref = self
            .mem
            .get_mut(address)
            .ok_or_else(|| anyhow!("Failed to write at"))?;
        *mem_ref = value;
        Ok(())
    }

    fn read_input(&mut self) -> anyhow::Result<IntCell> {
        self.input
            .pop_front()
            .ok_or_else(|| anyhow!("Failed to read input"))
    }

    pub(crate) fn add_input(&mut self, value: IntCell) {
        self.input.push_back(value);
    }

    fn write_output(&mut self, value: IntCell) {
        self.output.push(value);
    }

    fn halt(&mut self) {
        self.halted = true;
    }

    pub(crate) fn get_output(self) -> Vec<IntCell> {
        self.output.clone()
    }

    pub fn write_pc(&mut self, value: IntCell) -> anyhow::Result<()> {
        self.pc = value
            .try_into()
            .with_context(|| format!("Invalid value {value} given for PC"))?;
        Ok(())
    }

    pub(crate) fn pop_output(&mut self) -> anyhow::Result<IntCell> {
        match self.output.clone().as_slice() {
            [value, rest @ ..] => {
                let new_output = rest.to_vec();
                self.output = new_output;
                Ok(*value)
            }
            [] => Err(anyhow::anyhow!("No output to pop")),
        }
    }

    pub(crate) fn delta_relative_base(&mut self, delta: IntCell) {
        self.relative_base += delta;
    }
}
