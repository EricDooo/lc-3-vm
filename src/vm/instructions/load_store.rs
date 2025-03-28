use std::io;
use crate::vm::{LC3, Register};
use super::utils::{FromU16, sign_extend};

impl LC3 {
    /// Executes LD (load) instruction
    /// Format: LD DR, OFFSET9
    pub(super) fn execute_ld(&mut self, instr: u16) -> io::Result<()> {
        let dr = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let pc = self.registers.get(Register::PC);
        let address = pc.wrapping_add(pc_offset);
        let value = self.memory.read(address, &mut self.console)?;
        self.registers.set(Register::from_u16(dr), value);
        self.registers.update_flags(Register::from_u16(dr));
        Ok(())
    }

    /// Executes LDI (load indirect) instruction
    /// Format: LDI DR, OFFSET9
    pub(super) fn execute_ldi(&mut self, instr: u16) -> io::Result<()> {
        let dr = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let pc = self.registers.get(Register::PC);
        let address = pc.wrapping_add(pc_offset);

        let indirect_address = self.memory.read(address, &mut self.console)?;
        let value = self.memory.read(indirect_address, &mut self.console)?;
        self.registers.set(Register::from_u16(dr), value);
        self.registers.update_flags(Register::from_u16(dr));
        Ok(())
    }

    /// Executes LDR (load register) instruction
    /// Format: LDR DR, BaseR, OFFSET6
    pub(super) fn execute_ldr(&mut self, instr: u16) -> io::Result<()> {
        let dr = (instr >> 9) & 0x7;
        let base_r = (instr >> 6) & 0x7;
        let offset = sign_extend(instr & 0x3F, 6);
        let base_value = self.registers.get(Register::from_u16(base_r));
        let address = base_value.wrapping_add(offset);
        let value = self.memory.read(address, &mut self.console)?;
        self.registers.set(Register::from_u16(dr), value);
        self.registers.update_flags(Register::from_u16(dr));
        Ok(())
    }

    /// Executes LEA (load effective address) instruction
    /// Format: LEA DR, OFFSET9
    pub(super) fn execute_lea(&mut self, instr: u16) {
        let dr = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let pc = self.registers.get(Register::PC);
        let result = pc.wrapping_add(pc_offset);
        self.registers.set(Register::from_u16(dr), result);
        self.registers.update_flags(Register::from_u16(dr));
    }

    /// Executes ST (store) instruction
    /// Format: ST SR, OFFSET9
    pub(super) fn execute_st(&mut self, instr: u16) -> io::Result<()> {
        let sr = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let pc = self.registers.get(Register::PC);
        let address = pc.wrapping_add(pc_offset);
        let value = self.registers.get(Register::from_u16(sr));
        self.memory.write(address, value);
        Ok(())
    }

    /// Executes STI (store indirect) instruction
    /// Format: STI SR, OFFSET9
    pub(super) fn execute_sti(&mut self, instr: u16) -> io::Result<()> {
        let sr = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let pc = self.registers.get(Register::PC);
        let address = pc.wrapping_add(pc_offset);

        let value = self.registers.get(Register::from_u16(sr));
        let indirect_address = self.memory.read(address, &mut self.console)?;

        self.memory.write(indirect_address, value);
        Ok(())
    }

    /// Executes STR (store register) instruction
    /// Format: STR SR, BaseR, OFFSET6
    pub(super) fn execute_str(&mut self, instr: u16) -> io::Result<()> {
        let sr = (instr >> 9) & 0x7;
        let base_r = (instr >> 6) & 0x7;
        let offset = sign_extend(instr & 0x3F, 6);
        let base_value = self.registers.get(Register::from_u16(base_r));
        let address = base_value.wrapping_add(offset);
        let value = self.registers.get(Register::from_u16(sr));
        self.memory.write(address, value);
        Ok(())
    }
}