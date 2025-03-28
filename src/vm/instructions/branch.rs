use crate::vm::{LC3, Register};
use super::utils::{FromU16, sign_extend};

impl LC3 {
    /// Executes BR (branch) instruction
    /// Format: BR{n,z,p} OFFSET9
    pub(super) fn execute_br(&mut self, instr: u16) {
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let cond_flag = (instr >> 9) & 0x7;
        if (cond_flag & self.registers.get_condition_flag()) != 0 {
            let pc = self.registers.get(Register::PC);
            self.registers.set(Register::PC, pc.wrapping_add(pc_offset));
        }
    }

    /// Executes JMP/RET instruction
    /// Format: JMP BaseR (RET when BaseR is R7)
    pub(super) fn execute_jmp(&mut self, instr: u16) {
        let base_r = (instr >> 6) & 0x7;
        let value = self.registers.get(Register::from_u16(base_r));
        self.registers.set(Register::PC, value);
    }

    /// Executes JSR/JSRR instruction
    /// Format: JSR OFFSET11 or JSRR BaseR
    pub(super) fn execute_jsr(&mut self, instr: u16) {
        let long_flag = (instr >> 11) & 1;
        let pc = self.registers.get(Register::PC);
        self.registers.set(Register::R7, pc);

        if long_flag != 0 {
            let pc_offset = sign_extend(instr & 0x7FF, 11);
            self.registers.set(Register::PC, pc.wrapping_add(pc_offset));
        } else {
            let base_r = (instr >> 6) & 0x7;
            let value = self.registers.get(Register::from_u16(base_r));
            self.registers.set(Register::PC, value);
        }
    }
}