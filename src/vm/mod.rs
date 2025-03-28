mod memory;
mod registers;
mod instructions;
mod opcodes;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub use self::memory::*;
pub use self::registers::*;
pub use self::opcodes::*;
pub use self::instructions::*;

use crate::io::console::Console;

/// Memory size: 2^16 locations
pub const MEMORY_SIZE: usize = 1 << 16;
/// Default program start location
pub const PC_START: u16 = 0x3000;

/// LC-3 Virtual Machine
pub struct LC3 {
    pub memory: Memory,
    pub registers: Registers,
    pub running: bool,
    console: Console,
}

impl LC3 {
    /// Creates a new LC-3 VM instance
    pub fn new() -> Self {
        let mut vm = LC3 {
            memory: Memory::new(),
            registers: Registers::new(),
            running: false,
            console: Console::new(),
        };

        vm.registers.set(Register::PC, PC_START);
        vm.registers.set_condition_flag(CondFlag::ZRO);

        vm
    }

    /// Loads a program from a binary file
    pub fn read_image_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = [0; 2];

        file.read_exact(&mut buffer)?;
        let origin = u16::from_be_bytes(buffer);

        let mut i = origin as usize;
        while let Ok(_) = file.read_exact(&mut buffer) {
            self.memory.write(i as u16, u16::from_be_bytes(buffer));
            i += 1;
        }

        Ok(())
    }

    /// Runs the VM until halted
    pub fn run(&mut self) -> io::Result<()> {
        self.running = true;
        self.console.setup()?;

        while self.running {
            let pc = self.registers.get(Register::PC);
            self.registers.set(Register::PC, pc + 1);
            let instr = self.memory.read(pc, &mut self.console)?;

            self.execute_instruction(instr)?;
        }

        self.console.cleanup()?;
        Ok(())
    }
}