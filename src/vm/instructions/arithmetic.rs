use crate::vm::{LC3, Register};
use super::utils::{FromU16, sign_extend};

impl LC3 {
    /// Executes ADD instruction
    /// Format: ADD DR, SR1, SR2/IMM5
    pub(super) fn execute_add(&mut self, instr: u16) {
        let dr = (instr >> 9) & 0x7;
        let sr1 = (instr >> 6) & 0x7;
        
        if (instr & 0x20) == 0 {
            let sr2 = instr & 0x7;
            let result = self.registers.get(Register::from_u16(sr1))
                .wrapping_add(self.registers.get(Register::from_u16(sr2)));
            self.registers.set(Register::from_u16(dr), result);
        } else {
            let imm5 = sign_extend(instr & 0x1F, 5);
            let result = self.registers.get(Register::from_u16(sr1)).wrapping_add(imm5);
            self.registers.set(Register::from_u16(dr), result);
        }
        self.registers.update_flags(Register::from_u16(dr));
    }

    /// Executes AND instruction
    /// Format: AND DR, SR1, SR2/IMM5
    pub(super) fn execute_and(&mut self, instr: u16) {
        let dr = (instr >> 9) & 0x7;
        let sr1 = (instr >> 6) & 0x7;
        if (instr & 0x20) == 0 {
            let sr2 = instr & 0x7;
            let result = self.registers.get(Register::from_u16(sr1))
                & self.registers.get(Register::from_u16(sr2));
            self.registers.set(Register::from_u16(dr), result);
        } else {
            let imm5 = sign_extend(instr & 0x1F, 5);
            let result = self.registers.get(Register::from_u16(sr1)) & imm5;
            self.registers.set(Register::from_u16(dr), result);
        }
        self.registers.update_flags(Register::from_u16(dr));
    }

    /// Executes NOT instruction
    /// Format: NOT DR, SR
    pub(super) fn execute_not(&mut self, instr: u16) {
        let dr = (instr >> 9) & 0x7;
        let sr = (instr >> 6) & 0x7;
        let result = !self.registers.get(Register::from_u16(sr));
        self.registers.set(Register::from_u16(dr), result);
        self.registers.update_flags(Register::from_u16(dr));
    }
}