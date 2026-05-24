use crate::{
    compiler::web::{LEB128, SLEB128},
    instruction::Instruction,
    program::Program,
    tokeniser,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RightScan {
    source_mapping: tokeniser::SourceMapping,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeftScan {
    source_mapping: tokeniser::SourceMapping,
}

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

impl RightScan {
    pub fn new(source_mapping: tokeniser::SourceMapping) -> Self {
        Self { source_mapping }
    }
}

impl Instruction for RightScan {
    fn execute(&self, program: &mut Program) -> () {
        let memory = &program.memory;
        let memory_len = memory.len();
        let mut program_pointer = program.pointer;
        while memory[program_pointer] != 0 {
            program_pointer += 1;
            assert!(
                program_pointer < memory_len,
                "RuntimeError: Memory overflow at {}",
                self.source_mapping
            );
        }
        program.pointer = program_pointer;
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        // add try catch with runtime_error_tag throw
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
        result.push(0x00); // local index 0 (cell address)
        result.push(0x02); // block
        result.push(0x40); // void
        result.push(0x03); // loop
        result.push(0x40); // void
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 (cell address)
        result.push(0x2d); // i32.load8_u
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        result.push(0x45); // i32.eqz check if cell value is zero
        result.push(0x0d); // br_if
        result.push(0x01); // break depth
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 (cell address)
        result.push(0x41); // i32.const
        result.push(0x01); // i32 literal 1
        result.push(0x6a); // i32.add 1 to cell address
        result.push(0x21); // local.set
        result.push(0x00); // local index 0 (cell address)
        // use a loop to increment cell address by 1 then load the byte at cell address
        // break out of the loop block if value on stack is 0
        // wrap load instruction in try catch, use runtime_error_tag to propagate errors
        result.push(0x0c); // br
        result.push(0x00); // break depth
        result.push(0x0b); // end loop
        result.push(0x0b); // end block
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 (cell address)
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load memory pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        result.push(0x6b); // i32.sub sub program.memory pointer from from $cell_address to get program.pointer
        result.push(0x36); // i32.store
        result.push(0x02); // alignment
        result.push(0x00); // load offset

        result
    }
}

impl LeftScan {
    pub fn new(source_mapping: tokeniser::SourceMapping) -> Self {
        Self { source_mapping }
    }
}

impl Instruction for LeftScan {
    fn execute(&self, program: &mut Program) -> () {
        let memory = &program.memory;
        let mut program_pointer = program.pointer;
        while memory[program_pointer] != 0 {
            assert!(
                program_pointer > 0,
                "RuntimeError: Memory overflow at {}",
                self.source_mapping
            );
            program_pointer -= 1;
        }
        program.pointer = program_pointer;
        program.counter += 1;
    }

    fn emit(&self, program: &Program) -> Vec<u8> {
        // add try catch with runtime_error_tag throw
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
        result.push(0x00); // local index 0 (cell address)
        result.push(0x02); // block
        result.push(0x40); // void
        result.push(0x03); // loop
        result.push(0x40); // void
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 (cell address)
        result.push(0x2d); // i32.load8_u
        result.push(0x00); // alignment
        result.push(0x00); // load offset
        result.push(0x45); // i32.eqz check if cell value is zero
        result.push(0x0d); // br_if
        result.push(0x01); // break depth
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 (cell address)
        result.push(0x41); // i32.const
        result.push(0x01); // i32 literal 1
        result.push(0x6b); // i32.sub 1 from cell address
        result.push(0x21); // local.set
        result.push(0x00); // local index 0 (cell address)
        // use a loop to increment cell address by 1 then load the byte at cell address
        // break out of the loop block if value on stack is 0
        // wrap load instruction in try catch, use runtime_error_tag to propagate errors
        result.push(0x0c); // br
        result.push(0x00); // break depth
        result.push(0x0b); // end loop
        result.push(0x0b); // end block
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x20); // local.get
        result.push(0x00); // local index 0 (cell address)
        result.push(0x41); // i32.const
        result.append(
            &mut SLEB128::from(program as i32).inner, // i32 literal
        );
        result.push(0x28); // i32.load load memory pointer into stack
        result.push(0x02); // alignment
        result.push(0x0c); // load offset
        result.push(0x6b); // i32.sub sub program.memory pointer from from $cell_address to get program.pointer
        result.push(0x36); // i32.store
        result.push(0x02); // alignment
        result.push(0x00); // load offset

        result
    }
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
