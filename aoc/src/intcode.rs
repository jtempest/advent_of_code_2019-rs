//! Run IntCode programs as defined by the puzzles in [Advent of Code 2019](https://adventofcode.com/2019).
//!
//! # Examples
//! ```
//! use aoc::intcode::{Program, Machine};
//!
//! let program = Program::from("3,10,4,10,99");
//! let mut machine = Machine::new(&program);
//! machine.input(42);
//! let output = machine.run();
//! assert_eq!(output, Some(42));
//!
//! let program = Program::from("104,1,104,2,104,3,99");
//! let output = Machine::new(&program).run_as_iter().collect::<Vec<_>>();
//! assert_eq!(output, [1, 2, 3]);
//! ```

use std::collections::VecDeque;
use std::fmt;
use std::ops::{Add, Mul};

// Set true for verbose debugging output when intcode machines are running
const INTCODE_DEBUG: bool = false;

// Simple pass-through macro to only print if INTCODE_DEBUG is true
macro_rules! intcode_debug {
    ($($arg:tt)*) => ({
        if INTCODE_DEBUG {
            println!($($arg)*);
        }
    })
}

/// A program that can be run on an IntCode [Machine](struct.Machine.html).
#[derive(Debug, Clone, PartialEq)]
pub struct Program(Vec<i64>);

impl Program {
    pub fn write(&mut self, position: usize, value: i64) {
        self.0[position] = value;
    }
}

impl From<&str> for Program {
    fn from(input: &str) -> Program {
        let program = input
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        Program(program)
    }
}

// An IntCode opcode
#[derive(Debug, Clone, Copy, PartialEq)]
enum Opcode {
    Halt,
    Input,
    Output,
    AdjustRelativeBase,
    Add,
    Mul,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

impl Opcode {
    fn new(value: i64) -> Opcode {
        let opcode = value % 100;
        match opcode {
            99 => Opcode::Halt,
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            9 => Opcode::AdjustRelativeBase,
            _ => panic!("Unknown opcode '{}'", opcode),
        }
    }
}

// An IntCode instruction parameter mode
#[derive(Debug, Clone, Copy, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn new(instruction: i64, param_index: usize) -> ParameterMode {
        assert!(param_index <= 2);
        let all_modes = instruction / 100;
        let mode = (all_modes / (10_i64.pow(param_index as u32))) % 10;
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown parameter mode {}", mode),
        }
    }
}

// A single IntCode instruction
#[derive(Copy, Clone, PartialEq)]
struct Instruction {
    value: i64,
    opcode: Opcode,
}

impl Instruction {
    fn new(value: i64) -> Instruction {
        Instruction {
            value,
            opcode: Opcode::new(value),
        }
    }

    // index is from 0.
    fn parameter_mode(&self, index: usize) -> ParameterMode {
        assert!(index <= 2);
        ParameterMode::new(self.value, index)
    }

    fn is_halt(&self) -> bool {
        if let Opcode::Halt = self.opcode {
            true
        } else {
            false
        }
    }

    fn is_input(&self) -> bool {
        if let Opcode::Input = self.opcode {
            true
        } else {
            false
        }
    }

    fn debug_param_modes(&self) -> Vec<ParameterMode> {
        let num_modes = match self.opcode {
            Opcode::Halt => 0,
            Opcode::Input => 1,
            Opcode::Output => 1,
            Opcode::AdjustRelativeBase => 1,
            Opcode::JumpIfTrue => 2,
            Opcode::JumpIfFalse => 2,
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::LessThan => 3,
            Opcode::Equals => 3,
        };
        self.debug_read_param_modes(num_modes)
    }

    fn debug_read_param_modes(&self, num_modes: usize) -> Vec<ParameterMode> {
        (0..num_modes).map(|n| self.parameter_mode(n)).collect()
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Instruction {{ {:?} {:?} }}",
            self.opcode,
            self.debug_param_modes()
        )
    }
}

// The next action for a Machine to take after executing an instruction.
#[derive(Debug)]
enum NextAction {
    Continue,
    Halt,
    Output(i64),
}

/// A machine that runs an IntCode [Program](struct.Program.html).
#[derive(Debug)]
pub struct Machine {
    ip: usize, // Instruction Pointer
    rbo: i64,  // Relative Base Offset
    memory: Vec<i64>,
    input: VecDeque<i64>,
}

impl Machine {
    /// Construct a new Machine to run the given [Program](struct.Program.html).
    pub fn new(program: &Program) -> Machine {
        Machine {
            ip: 0,
            rbo: 0,
            memory: program.0.clone(),
            input: VecDeque::new(),
        }
    }

    /// Construct a new Machine to run the program produced by the given source code.
    pub fn from_source(program: &str) -> Machine {
        Machine::new(&Program::from(program))
    }

    /// Construct a new Machine to run the given [Program](struct.Program.html),
    /// buffering an initial input value.
    pub fn with_input(program: &Program, input: i64) -> Machine {
        let mut machine = Machine::new(&program);
        machine.input(input);
        machine
    }

    /// Construct a new Machine to run the program produced by the given source code,
    /// buffering an initial input value.
    pub fn from_source_with_input(program: &str, input: i64) -> Machine {
        Machine::with_input(&Program::from(program), input)
    }

    /// Run until a pause state is reached.
    ///
    /// Returns once the machine halts execution, with the value:
    /// - None if there was a Halt instruction (99). See [is_halted](struct.Machine.html#method.is_halted).
    /// - None if there was an Input instruction (3) and no input was buffered.
    ///   See [is_awaiting_input](struct.Machine.html#method.is_awaiting_input).
    /// - Some(value) if there was an Output instruction (4).
    pub fn run(&mut self) -> Option<i64> {
        loop {
            let action = self.exec_next_instruction();
            match action {
                NextAction::Continue => continue,
                NextAction::Halt => {
                    intcode_debug!("HALTING");
                    break None;
                }
                NextAction::Output(value) => {
                    intcode_debug!("OUTPUT({})", value);
                    break Some(value);
                }
            }
        }
    }

    /// Calls [run](struct.Machine.html#method.run) after buffering the given
    /// input value.
    pub fn run_with_input(&mut self, input: i64) -> Option<i64> {
        self.input(input);
        self.run()
    }

    /// Constructs an iterator that calls [run](struct.Machine.html#method.run)
    /// on `next` such that multiple output values can be easily collected.
    ///
    /// ```
    /// use aoc::intcode::{Program, Machine};
    ///
    /// let program = Program::from("104,1,104,2,104,3,99");
    /// let output = Machine::new(&program).run_as_iter().collect::<Vec<_>>();
    /// assert_eq!(output, [1, 2, 3]);
    /// ```
    pub fn run_as_iter(&mut self) -> RunAsIter {
        RunAsIter(self)
    }

    /// Calls [run](struct.Machine.html#method.run) until the program pauses,
    /// returning the output values interpreted as an ASCII string.
    pub fn run_as_ascii(&mut self) -> String {
        self.run_as_iter().map(|v| v as u8 as char).collect()
    }

    /// Buffer the given input value so the next time the program is [run](struct.Machine.html#method.run)
    /// it may read it.
    pub fn input(&mut self, value: i64) {
        self.input.push_front(value);
    }

    /// Input the given ASCII string and then input an additional '\n'.
    pub fn input_ascii(&mut self, ascii_line: &str) {
        for c in ascii_line.chars() {
            self.input(c as i64);
        }
        self.input('\n' as i64);
    }

    /// Read a single value from the Machine's memory at the given address.
    pub fn read(&self, address: usize) -> i64 {
        if address < self.memory.len() {
            self.memory[address]
        } else {
            0
        }
    }

    /// Write a single value into the Machine's memory at the given address.
    pub fn write(&mut self, address: usize, value: i64) {
        self.ensure_memory(address);
        self.memory[address] = value;
    }

    /// The entire current memory state of this Machine.
    pub fn memory(&self) -> &Vec<i64> {
        &self.memory
    }

    /// True if the machine has reached a Halt instruction (99).
    pub fn is_halted(&self) -> bool {
        self.read_instruction().is_halt()
    }

    /// True if the machine is paused awaiting [input](struct.Machine.html#method.input).
    pub fn is_awaiting_input(&self) -> bool {
        self.read_instruction().is_input()
    }

    fn read_instruction(&self) -> Instruction {
        Instruction::new(self.read(self.ip))
    }

    fn exec_next_instruction(&mut self) -> NextAction {
        let instruction = self.read_instruction();
        intcode_debug!(
            "@{}: {} => {:?}",
            self.ip,
            self.memory[self.ip],
            instruction
        );
        match instruction.opcode {
            Opcode::Halt => NextAction::Halt,
            Opcode::Add => self.exec_binary_op(Add::add),
            Opcode::Mul => self.exec_binary_op(Mul::mul),
            Opcode::Input => self.exec_input_op(),
            Opcode::Output => self.exec_output_op(),
            Opcode::JumpIfFalse => self.exec_jump_if_op(|v| v == 0),
            Opcode::JumpIfTrue => self.exec_jump_if_op(|v| v != 0),
            Opcode::LessThan => self.exec_binary_op(|a, b| if a < b { 1 } else { 0 }),
            Opcode::Equals => self.exec_binary_op(|a, b| if a == b { 1 } else { 0 }),
            Opcode::AdjustRelativeBase => self.exec_adjust_rbo(),
        }
    }

    fn exec_binary_op<F: Fn(i64, i64) -> i64>(&mut self, func: F) -> NextAction {
        let v1 = self.exec_read(0);
        let v2 = self.exec_read(1);
        let result = func(v1, v2);
        self.exec_write(2, result);

        self.ip += 4;
        NextAction::Continue
    }

    fn exec_jump_if_op<F: Fn(i64) -> bool>(&mut self, predicate: F) -> NextAction {
        let value = self.exec_read(0);
        if predicate(value) {
            let dest = self.exec_read(1);
            intcode_debug!("jump => {}", dest);
            self.ip = dest as usize;
        } else {
            self.ip += 3;
        }

        NextAction::Continue
    }

    fn exec_input_op(&mut self) -> NextAction {
        match self.input.pop_back() {
            None => NextAction::Halt,
            Some(value) => {
                self.exec_write(0, value);
                self.ip += 2;
                NextAction::Continue
            }
        }
    }

    fn exec_output_op(&mut self) -> NextAction {
        let value = self.exec_read(0);
        self.ip += 2;
        NextAction::Output(value)
    }

    fn exec_adjust_rbo(&mut self) -> NextAction {
        let value = self.exec_read(0);
        self.rbo += value;
        intcode_debug!("rbo = {}", self.rbo);

        self.ip += 2;
        NextAction::Continue
    }

    // param is zero indexed
    fn exec_read(&mut self, param: usize) -> i64 {
        let value = self.read(self.ip + param + 1);
        match self.read_instruction().parameter_mode(param) {
            ParameterMode::Position => {
                let output = self.read_mut(value as usize);
                intcode_debug!("param@{} => {}", value, output);
                output
            }
            ParameterMode::Immediate => {
                intcode_debug!("param: {}", value);
                value
            }
            ParameterMode::Relative => {
                let pos = (self.rbo + value) as usize;
                let output = self.read_mut(pos);
                intcode_debug!("param@({} + {} = {}) => {}", self.rbo, value, pos, output);
                output
            }
        }
    }

    // param is zero indexed
    fn exec_write(&mut self, param: usize, value: i64) {
        let offset = self.read(self.ip + param + 1);
        let address = match self.read_instruction().parameter_mode(param) {
            ParameterMode::Position => {
                intcode_debug!("write@{} <= {}", offset, value);
                offset
            }
            ParameterMode::Relative => {
                let address = self.rbo + offset;
                intcode_debug!(
                    "write@({} + {} = {}) <= {}",
                    self.rbo,
                    offset,
                    address,
                    value
                );
                address
            }
            ParameterMode::Immediate => panic!("Cannot write in immediate mode"),
        };
        self.write(address as usize, value);
    }

    fn read_mut(&mut self, address: usize) -> i64 {
        self.ensure_memory(address);
        self.memory[address]
    }

    fn ensure_memory(&mut self, max_address: usize) {
        if max_address >= self.memory().len() {
            intcode_debug!("expanding memory to address {}", max_address);
            self.memory.resize(max_address + 1, 0);
        }
    }
}

/// Allows easy collection of multiple output values from a [Machine](struct.Machine.html).
///
/// See [Machine::run_as_iter](struct.Machine.html#method.run_as_iter).
pub struct RunAsIter<'a>(&'a mut Machine);

impl Iterator for RunAsIter<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.0.run()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_machine_run_state(program: &str, expected_final_state: &[i64]) {
        let mut m = Machine::from_source(program);
        m.run();
        assert!(m.is_halted());
        assert_eq!(m.memory(), &expected_final_state);
    }

    fn test_machine_run_output(program: &str, expected_output: i64) {
        let output = Machine::from_source(program).run().unwrap();
        assert_eq!(output, expected_output);
    }

    fn test_machine_run_io(program: &str, input: i64, expected_output: i64) {
        let output = Machine::from_source(&program)
            .run_with_input(input)
            .unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_machine_run() {
        test_machine_run_state("99", &[99]);
        test_machine_run_state("1,0,0,0,99", &[2, 0, 0, 0, 99]);
        test_machine_run_state("2,3,0,3,99", &[2, 3, 0, 6, 99]);
        test_machine_run_state("2,4,4,5,99,0", &[2, 4, 4, 5, 99, 9801]);
        test_machine_run_state("1,1,1,4,99,5,6,0,99", &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
        test_machine_run_state("1002,4,3,4,33", &[1002, 4, 3, 4, 99]);
        test_machine_run_state("1101,100,-1,4,0", &[1101, 100, -1, 4, 99]);

        let output_input = "3,0,4,0,99";
        test_machine_run_io(output_input, 0, 0);
        test_machine_run_io(output_input, 42, 42);

        let equals_eight_pm = "3,9,8,9,10,9,4,9,99,-1,8";
        test_machine_run_io(equals_eight_pm, 8, 1);
        test_machine_run_io(equals_eight_pm, -10, 0);

        let equals_eight_im = "3,3,1108,-1,8,3,4,3,99";
        test_machine_run_io(equals_eight_im, 8, 1);
        test_machine_run_io(equals_eight_im, -10, 0);

        let less_than_eight_pm = "3,9,7,9,10,9,4,9,99,-1,8";
        test_machine_run_io(less_than_eight_pm, 7, 1);
        test_machine_run_io(less_than_eight_pm, 8, 0);
        test_machine_run_io(less_than_eight_pm, 9, 0);

        let less_than_eight_pm = "3,3,1107,-1,8,3,4,3,99";
        test_machine_run_io(less_than_eight_pm, 7, 1);
        test_machine_run_io(less_than_eight_pm, 8, 0);
        test_machine_run_io(less_than_eight_pm, 9, 0);

        let jump_test_zero_if_zero_pm = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        test_machine_run_io(jump_test_zero_if_zero_pm, 0, 0);
        test_machine_run_io(jump_test_zero_if_zero_pm, 9, 1);

        let jump_test_zero_if_zero_im = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        test_machine_run_io(jump_test_zero_if_zero_im, 0, 0);
        test_machine_run_io(jump_test_zero_if_zero_im, 9, 1);

        let complex_cmp_eight = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31, \
                                 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104, \
                                 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        test_machine_run_io(complex_cmp_eight, 7, 999);
        test_machine_run_io(complex_cmp_eight, 8, 1000);
        test_machine_run_io(complex_cmp_eight, 9, 1001);

        let quine = Program::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let output = Machine::new(&quine).run_as_iter().collect();
        assert_eq!(Program(output), quine);

        test_machine_run_output("1102,34915192,34915192,7,4,7,99,0", 1_219_070_632_396_864);
        test_machine_run_output("104,1125899906842624,99", 1_125_899_906_842_624);
    }
}
