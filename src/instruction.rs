use enum_dispatch::enum_dispatch;

use crate::{
    compiler::{LEB128, SLEB128},
    io::{IO, RuntimeIO},
    program::{self, Program},
    tokeniser::{self},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Zero;

#[derive(Clone, Debug, PartialEq)]
pub struct RightCarry {
    count: usize,
    source_mapping: tokeniser::SourceMapping,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeftCarry {
    count: usize,
    source_mapping: tokeniser::SourceMapping,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Right {
    count: usize,
    source_mapping: tokeniser::SourceMapping,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Left {
    count: usize,
    source_mapping: tokeniser::SourceMapping,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Increment {
    amount: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Decrement {
    amount: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RightJump {
    end: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeftJump {
    start: usize,
}

#[enum_dispatch]
pub trait Instruction {
    fn execute(&self, program: &mut Program) -> ();
    fn emit(&self, program: &Program) -> Vec<u8>;
}

impl RightCarry {
    pub fn new(count: usize, source_mapping: tokeniser::SourceMapping) -> Self {
        Self {
            count,
            source_mapping,
        }
    }
}

impl Instruction for RightCarry {
    fn execute(&self, program: &mut Program) -> () {
        let source = program.pointer;
        let value = program.memory[source];
        let mask = value != 0;
        let target = source + (self.count * (mask) as usize);
        assert!(
            target < program.memory.len(),
            "RuntimeError: Memory overflow at {}",
            self.source_mapping
        );
        program.memory[target] = program.memory[target].wrapping_add(value);
        program.memory[source] = 0;
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        let program = program as *const Program;
        // push program pointer constant onto stack
        let mut result = vec![
            0x41, // i32.const
        ];
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        // load program.pointer into stack <--- source
        result.push(0x28); // i32.load load program.pointer into stack
        result.push(0x02); // alignment
        result.push(0x00); // load offset
        // set $program_pointer
        result.push(0x21); // local.set
        result.push(0x02); // local index 2 ($program_pointer)
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 ($program_pointer)
        // push program pointer constant into stack
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        // load program.memory data pointer into stack
        result.push(0x28); // i32.load load program.memory data pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        // add program.pointer + program.memory data pointer to get source cell address
        result.push(0x6a); // i32.add add program.pointer and program.memory pointer to get cell address
        // set (source) $cell_addr
        result.push(0x21); // local.set
        result.push(0x00); // local index 0 (cell address)
        // get (source) $cell_addr
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 (cell address)
        // load byte at cell address
        result.push(0x2d); // i32.load8_u load cell into stack
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        // i32.eqz check if value is 0
        result.push(0x45);
        // i32.eqz check if previous check results in 0
        result.push(0x45);
        // push self.count onto stack
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.count as u32).inner, // i32 literal
        );
        // i32.mul (self.count * mask)
        result.push(0x6c);
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 ($program_pointer)
        // i32.add (source + (self.count * mask))
        result.push(0x6a); // i32.add
        // set $program_pointer
        result.push(0x21); // local.set
        result.push(0x02); // local index 2 ($program_pointer)
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 ($program_pointer)
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load program.memory length into stack
        result.push(0x02); // alignment
        result.push(0x08); // load offset
        result.push(0x4a); // i32.gt_s
        result.push(0x04); // if
        result.push(0x40); // void block type
        result.push(0x41); // i32.const
        result.push(0x01); // i32 literal 0
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.source_mapping.line() as u32).inner, // i32 literal
        );
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.source_mapping.column() as u32).inner, // i32 literal
        );
        let path = self.source_mapping.file_path();
        let path_length = path.len() as u32;
        let path = path as *const str as *const u8;
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(path as i32).inner, // i32 literal
        );
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(path_length as i32).inner, // i32 literal
        );
        result.push(0x08); // throw
        result.push(0x00); // $runtime_error_tag
        result.push(0x0b); // end
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 ($program_pointer)
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        // load program.memory data pointer into stack
        result.push(0x28); // i32.load load program.memory data pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        // add $program_pointer + program.memory data pointer to get (target) cell address
        result.push(0x6a); // i32.add add $program_pointer and program.memory data pointer to get cell address
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 ($program_pointer)
        // push program pointer constant into stack
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        // load program.memory data pointer into stack
        result.push(0x28); // i32.load load program.memory data pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        // add $program_pointer + program.memory data pointer to get (target) cell address
        result.push(0x6a); // i32.add add $program_pointer and program.memory data pointer to get cell address
        // load byte at cell address
        result.push(0x2d); // i32.load8_u load cell into stack
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        // get $cell_addr
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 (cell address)
        // load byte at cell address
        result.push(0x2d); // i32.load8_u load cell into stack
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        result.push(0x6a); // i32.add add target value and source value
        // store byte at $cell_addr
        result.push(0x3a); // i32.store8
        result.push(0x00); // alignment
        result.push(0x00); // store offset
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 ($cell_address)
        // push 0 constant onto stack
        result.push(0x41); // i32.const
        result.push(0x00); // i32 literal 0
        // store byte
        result.push(0x3a); // i32.store8 store 0 in cell
        result.push(0x00); // alignment
        result.push(0x00); // store offset

        result
    }
}

impl LeftCarry {
    pub fn new(count: usize, source_mapping: tokeniser::SourceMapping) -> Self {
        Self {
            count,
            source_mapping,
        }
    }
}

impl Instruction for LeftCarry {
    fn execute(&self, program: &mut Program) -> () {
        let source = program.pointer;
        let value = program.memory[source];
        let mask = value != 0;
        assert!(
            !mask || program.pointer >= self.count,
            "RuntimeError: Memory underflow at {}",
            self.source_mapping
        );
        let target = source - (self.count * (mask) as usize);
        program.memory[target] = program.memory[target].wrapping_add(value);
        program.memory[source] = 0;
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        let program = program as *const Program;
        // push program pointer constant onto stack
        let mut result = vec![
            0x41, // i32.const
        ];
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        // load program.pointer into stack <--- source
        result.push(0x28); // i32.load load program.pointer into stack
        result.push(0x02); // alignment
        result.push(0x00); // load offset
        // set $program_pointer
        result.push(0x21); // local.set
        result.push(0x02); // local index 2 ($program_pointer)
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 ($program_pointer)
        // push program pointer constant into stack
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        // load program.memory data pointer into stack
        result.push(0x28); // i32.load load program.memory data pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        // add program.pointer + program.memory data pointer to get source cell address
        result.push(0x6a); // i32.add add program.pointer and program.memory pointer to get cell address
        // set (source) $cell_addr
        result.push(0x21); // local.set
        result.push(0x00); // local index 0 (cell address)
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 ($program_pointer)
        // get (source) $cell_addr
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 (cell address)
        // load byte at cell address
        result.push(0x2d); // i32.load8_u load cell into stack
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        // i32.eqz check if value is 0
        result.push(0x45);
        // i32.eqz check if previous check results in 0
        result.push(0x45);
        // push self.count onto stack
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.count as u32).inner, // i32 literal
        );
        // i32.mul (self.count * mask)
        result.push(0x6c);
        // i32.sub (source - (self.count * mask))
        result.push(0x6b); // i32.sub
        // set $program_pointer
        result.push(0x21); // local.set
        result.push(0x02); // local index 2 ($program_pointer)
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 ($program_pointer)
        result.push(0x41); // i32.const
        result.push(0x00); // i32 literal 0
        result.push(0x48); // i32.lt_s
        result.push(0x04); // if
        result.push(0x40); // void block type
        result.push(0x41); // i32.const
        result.push(0x01); // i32 literal 0
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.source_mapping.line() as u32).inner, // i32 literal
        );
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.source_mapping.column() as u32).inner, // i32 literal
        );
        let path = self.source_mapping.file_path();
        let path_length = path.len() as u32;
        let path = path as *const str as *const u8;
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(path as i32).inner, // i32 literal
        );
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(path_length as i32).inner, // i32 literal
        );
        result.push(0x08); // throw
        result.push(0x00); // $runtime_error_tag
        result.push(0x0b); // end
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 ($program_pointer)
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        // load program.memory data pointer into stack
        result.push(0x28); // i32.load load program.memory data pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        // add $program_pointer + program.memory data pointer to get (target) cell address
        result.push(0x6a); // i32.add add $program_pointer and program.memory data pointer to get cell address
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 ($program_pointer)
        // push program pointer constant into stack
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        // load program.memory data pointer into stack
        result.push(0x28); // i32.load load program.memory data pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        // add $program_pointer + program.memory data pointer to get (target) cell address
        result.push(0x6a); // i32.add add $program_pointer and program.memory data pointer to get cell address
        // load byte at cell address
        result.push(0x2d); // i32.load8_u load cell into stack
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        // get $cell_addr
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 (cell address)
        // load byte at cell address
        result.push(0x2d); // i32.load8_u load cell into stack
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        result.push(0x6a); // i32.add add target value and source value
        // store byte at $cell_addr
        result.push(0x3a); // i32.store8
        result.push(0x00); // alignment
        result.push(0x00); // store offset
        // get $program_pointer
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 ($cell_address)
        // push 0 constant onto stack
        result.push(0x41); // i32.const
        result.push(0x00); // i32 literal 0
        // store byte
        result.push(0x3a); // i32.store8 store 0 in cell
        result.push(0x00); // alignment
        result.push(0x00); // store offset

        result
    }
}

impl Instruction for Zero {
    fn execute(&self, program: &mut Program) -> () {
        program.memory[program.pointer] = 0;
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        let program = program as *const Program;
        let mut result: Vec<u8> = vec![
            0x41, // i32.const
        ];
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load program pointer into stack
        result.push(0x02); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load memory pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        result.push(0x6a); // i32.add add program pointer and memory pointer to get cell address
        result.push(0x41); // i32.const
        result.push(0x00); // i32 literal 0
        result.push(0x3a); // i32.store8 store 0 in cell
        result.push(0x00); // alignment
        result.push(0x00); // store offset

        result
    }
}

impl Increment {
    pub fn new(amount: u8) -> Self {
        Self { amount }
    }
}

impl Instruction for Increment {
    fn execute(&self, program: &mut Program) -> () {
        program.memory[program.pointer] = program.memory[program.pointer].wrapping_add(self.amount);
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        let program = program as *const Program;
        let mut result: Vec<u8> = vec![
            0x41, // i32.const
        ];
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load program pointer into stack
        result.push(0x02); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load memory pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        result.push(0x6a); // i32.add add program pointer and memory pointer to get cell address
        result.push(0x21); // local.set
        result.push(0x00); // local index 0 store cell address in local variable 0
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 load cell address into stack
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 load cell address into stack
        result.push(0x2d); // i32.load8_u load cell into stack
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.amount as u32).inner, // i32 literal
        );
        result.push(0x6a); // i32.add add cell value and increment amount
        result.push(0x3a); // i32.store8
        result.push(0x00); // alignment
        result.push(0x00); // store offset

        result
    }
}

impl Decrement {
    pub fn new(amount: u8) -> Self {
        Self { amount }
    }
}

impl Instruction for Decrement {
    fn execute(&self, program: &mut Program) -> () {
        program.memory[program.pointer] = program.memory[program.pointer].wrapping_sub(self.amount);
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        let program = program as *const Program;
        let mut result: Vec<u8> = vec![
            0x41, // i32.const
        ];
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load program pointer into stack
        result.push(0x02); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load memory pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        result.push(0x6a); // i32.add add program pointer and memory pointer to get cell address
        result.push(0x21); // local.set
        result.push(0x00); // local index 0 store cell address in local variable 0
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 load cell address into stack
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 load cell address into stack
        result.push(0x2d); // i32.load8_u load cell into stack
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.amount as u32).inner, // i32 literal
        );
        result.push(0x6b); // i32.sub sub cell value and decrement amount
        result.push(0x3a); // i32.store8
        result.push(0x00); // alignment
        result.push(0x00); // store offset

        result
    }
}

impl Left {
    pub fn new(count: usize, source_mapping: tokeniser::SourceMapping) -> Self {
        Self {
            count,
            source_mapping,
        }
    }
}

impl Instruction for Left {
    fn execute(&self, program: &mut Program) -> () {
        assert!(
            program.pointer >= self.count,
            "RuntimeError: Memory underflow at {}",
            self.source_mapping
        );
        program.pointer -= self.count;
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        let program = program as *const Program;
        let mut result: Vec<u8> = vec![
            0x41, // i32.const
        ];
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load program pointer into stack
        result.push(0x02); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.count as u32).inner, // i32 literal
        );
        result.push(0x6b); // i32.sub sub memory.pointer and decrement count
        result.push(0x21); // local.set
        result.push(0x02); // local index 2 (program pointer)
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 load program pointer into stack
        result.push(0x41); // i32.const
        result.push(0x00); // i32 literal 0
        result.push(0x48); // i32.lt_s
        result.push(0x04); // if
        result.push(0x40); // void block type
        result.push(0x41); // i32.const
        result.push(0x00); // i32 literal 0
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.source_mapping.line() as u32).inner, // i32 literal
        );
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.source_mapping.column() as u32).inner, // i32 literal
        );
        let path = self.source_mapping.file_path();
        let path_length = path.len() as u32;
        let path = path as *const str as *const u8;
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(path as i32).inner, // i32 literal
        );
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(path_length as i32).inner, // i32 literal
        );
        result.push(0x08); // throw
        result.push(0x00); // $runtime_error_tag
        result.push(0x0b); // end
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 load program pointer into stack
        result.push(0x36); // i32.store
        result.push(0x02); // alignment
        result.push(0x00); // store offset

        result
    }
}

impl Right {
    pub fn new(count: usize, source_mapping: tokeniser::SourceMapping) -> Self {
        Self {
            count,
            source_mapping,
        }
    }
}

impl Instruction for Right {
    fn execute(&self, program: &mut Program) -> () {
        program.pointer += self.count;
        assert!(
            program.pointer < program.memory.len(),
            "RuntimeError: Memory overflow at {}",
            self.source_mapping
        );
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        // TODO: use cell address variable instead of program pointer
        let program = program as *const Program;
        let mut result: Vec<u8> = vec![
            0x41, // i32.const
        ];
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load program pointer into stack
        result.push(0x02); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.count as u32).inner, // i32 literal
        );
        result.push(0x6a); // i32.add add program pointer and increment count
        result.push(0x21); // local.set
        result.push(0x02); // local index 2 (program pointer)
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 load program pointer into stack
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load memory length into stack
        result.push(0x02); // alignment
        result.push(0x08); // load offset
        result.push(0x4a); // i32.gt_s
        result.push(0x04); // if
        result.push(0x40); // void block type
        result.push(0x41); // i32.const
        result.push(0x01); // i32 literal 0
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.source_mapping.line() as u32).inner, // i32 literal
        );
        result.push(0x41); // i32.const
        result.append(
            &mut LEB128::from(self.source_mapping.column() as u32).inner, // i32 literal
        );
        let path = self.source_mapping.file_path();
        let path_length = path.len() as u32;
        let path = path as *const str as *const u8;
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(path as i32).inner, // i32 literal
        );
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(path_length as i32).inner, // i32 literal
        );
        result.push(0x08); // throw
        result.push(0x00); // $runtime_error_tag
        result.push(0x0b); // end
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x20); // local.get
        result.push(0x02); // local index 2 load program pointer into stack
        result.push(0x36); // i32.store
        result.push(0x02); // alignment
        result.push(0x00); // store offset

        result
    }
}

impl Input {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl Instruction for Input {
    fn execute(&self, program: &mut Program) -> () {
        for _ in 0..self.count {
            RuntimeIO::read_exact(&mut program.memory[program.pointer..program.pointer + 1])
                .expect("Failed to read byte from standard input.");
        }
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        let program = program as *const Program;
        let mut result: Vec<u8> = vec![
            0x41, // i32.const
        ];
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load program pointer into stack
        result.push(0x02); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load memory pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        result.push(0x6a); // i32.add add program pointer and memory pointer to get cell address
        result.push(0x10); // call
        result.push(0x00); // function index (extern_read)
        result.push(0x3a); // i32.store8
        result.push(0x00); // alignment
        result.push(0x00); // store offset

        result
    }
}

impl Output {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl Instruction for Output {
    fn execute(&self, program: &mut Program) -> () {
        for _ in 0..self.count {
            RuntimeIO::write_all(&program.memory[program.pointer..program.pointer + 1])
                .expect("Failed to write byte to standard output.");
        }
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        let program = program as *const Program;
        let mut result: Vec<u8> = vec![
            0x41, // i32.const
        ];
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load program pointer into stack
        result.push(0x02); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load memory pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        result.push(0x6a); // i32.add add program pointer and memory pointer to get cell address
        result.push(0x2d); // i32.load8_u load cell into stack
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(self.count as i32).inner, // i32 literal
        );
        result.push(0x10); // call
        result.push(0x01); // function index (extern_write)

        result
    }
}

impl RightJump {
    pub fn new(end: usize) -> Self {
        Self { end }
    }
}

impl Instruction for RightJump {
    fn execute(&self, program: &mut Program) -> () {
        program.counter = if program.memory[program.pointer] == 0 {
            self.end + 1
        } else {
            program.counter + 1
        }
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        let program = program as *const Program;
        let mut result: Vec<u8> = vec![
            0x03, // loop
            0x40, // void
            0x41, // i32.const
        ];

        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load program pointer into stack
        result.push(0x02); // alignment
        result.push(0x00); // load offset
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load memory pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        result.push(0x6a); // i32.add add program pointer and memory pointer to get cell address
        result.push(0x2d); // i32.load8_u
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        result.push(0x45); // i32.eqz check if cell value is zero
        result.push(0x0d); // br_if
        result.push(0x01); // break depth

        result
    }
}

impl LeftJump {
    pub fn new(start: usize) -> Self {
        Self { start }
    }
}

impl Instruction for LeftJump {
    fn execute(&self, program: &mut Program) -> () {
        program.counter = if program.memory[program.pointer] != 0 {
            self.start + 1
        } else {
            program.counter + 1
        }
    }

    fn emit(&self, _program: &Program) -> Vec<u8> {
        // reset $program_counter to the start of the loop body if condition is true
        let mut result = vec![
            0x41, // i32.const
        ];
        result.append(
            &mut SLEB128::from(self.start as i32).inner, // i32 literal
        );
        result.push(0x21); // local.set
        result.push(0x01); // local index 1 (program counter)
        result.push(0x0c); // br
        result.push(0x00); // break depth
        result.push(0x0b); // end loop

        result
    }
}

#[enum_dispatch(Instruction)]
#[derive(Clone, Debug, PartialEq)]
pub enum InstructionSet {
    LeftCarry,
    RightCarry,
    Zero,
    Right,
    Left,
    Increment,
    Decrement,
    Input,
    Output,
    LeftJump,
    RightJump,
}

pub trait InstructionCollection {
    fn try_fold(&self) -> Option<InstructionSet>;
}

impl InstructionCollection for [InstructionSet] {
    fn try_fold(&self) -> Option<InstructionSet> {
        match self {
            [
                // [-]
                InstructionSet::RightJump(RightJump { end: 0 }),
                InstructionSet::Decrement(Decrement { amount: 1 }),
            ] => Some(InstructionSet::Zero(Zero)),
            [
                // [->+<]
                InstructionSet::RightJump(RightJump { end: 0 }),
                InstructionSet::Decrement(Decrement { amount: 1 }),
                InstructionSet::Right(Right {
                    count: right_count,
                    source_mapping,
                }),
                InstructionSet::Increment(Increment { amount: 1 }),
                InstructionSet::Left(Left {
                    count: left_count, ..
                }),
            ] => {
                let left_count = *left_count;
                let right_count = *right_count;
                if right_count == left_count {
                    Some(RightCarry::new(right_count, source_mapping.clone()).into())
                } else {
                    None
                }
            }
            [
                // [-<+>]
                InstructionSet::RightJump(RightJump { end: 0 }),
                InstructionSet::Decrement(Decrement { amount: 1 }),
                InstructionSet::Left(Left {
                    count: left_count,
                    source_mapping,
                }),
                InstructionSet::Increment(Increment { amount: 1 }),
                InstructionSet::Right(Right {
                    count: right_count, ..
                }),
            ] => {
                let left_count = *left_count;
                let right_count = *right_count;
                if right_count == left_count {
                    Some(LeftCarry::new(left_count, source_mapping.clone()).into())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
