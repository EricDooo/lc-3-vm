mod arithmetic;
mod branch;
mod load_store;
mod trap;
mod utils;

pub use self::utils::FromU16;
pub use self::utils::sign_extend;

use std::io;
use crate::vm::{LC3, OpCode};

impl LC3 {
    /// Executes a single instruction based on its opcode
    pub fn execute_instruction(&mut self, instr: u16) -> io::Result<()> {
        let op = OpCode::from((instr >> 12) as u8);

        match op {
            // Arithmetic operations
            OpCode::ADD => self.execute_add(instr),
            OpCode::AND => self.execute_and(instr),
            OpCode::NOT => self.execute_not(instr),
            
            // Branch and jump operations
            OpCode::BR => self.execute_br(instr),
            OpCode::JMP => self.execute_jmp(instr),
            OpCode::JSR => self.execute_jsr(instr),
            
            // Load operations
            OpCode::LD => self.execute_ld(instr)?,
            OpCode::LDI => self.execute_ldi(instr)?,
            OpCode::LDR => self.execute_ldr(instr)?,
            OpCode::LEA => self.execute_lea(instr),
            
            // Store operations
            OpCode::ST => self.execute_st(instr)?,
            OpCode::STI => self.execute_sti(instr)?,
            OpCode::STR => self.execute_str(instr)?,
            
            // Trap operation
            OpCode::TRAP => self.execute_trap(instr)?,
            
            // Unsupported operations
            OpCode::RTI | OpCode::RES => {
                return Err(io::Error::new(io::ErrorKind::Other, "Unsupported opcode"));
            }
        }

        Ok(())
    }
}