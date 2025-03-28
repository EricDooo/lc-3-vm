use std::io;
use crate::vm::{LC3, Register, TrapCode};

impl LC3 {
    /// Executes TRAP instruction
    /// Format: TRAP TRAPVECT8
    pub(super) fn execute_trap(&mut self, instr: u16) -> io::Result<()> {
        let pc = self.registers.get(Register::PC);
        self.registers.set(Register::R7, pc);

        let trap_code = TrapCode::from(instr & 0xFF);

        match trap_code {
            TrapCode::GETC => {
                let c = self.console.read_key()?;
                self.registers.set(Register::R0, c as u16);
                self.registers.update_flags(Register::R0);
            }
            TrapCode::OUT => {
                let c = self.registers.get(Register::R0) as u8;
                self.console.write_char(c)?;
            }
            TrapCode::PUTS => {
                let mut address = self.registers.get(Register::R0);
                loop {
                    let c = self.memory.read(address, &mut self.console)?;
                    if c == 0 {
                        break;
                    }
                    let character = (c & 0xFF) as u8;
                    self.console.write_char(character)?;
                    address += 1;
                }
            }
            TrapCode::IN => {
                self.console.write_str("Enter a character: ")?;
                let c = self.console.read_key()?;
                self.console.write_char(c)?;
                self.registers.set(Register::R0, c as u16);
                self.registers.update_flags(Register::R0);
            }
            TrapCode::PUTSP => {
                let mut address = self.registers.get(Register::R0);
                loop {
                    let value = self.memory.read(address, &mut self.console)?;
                    if value == 0 {
                        break;
                    }

                    let char1 = (value & 0xFF) as u8;
                    self.console.write_char(char1)?;

                    let char2 = ((value >> 8) & 0xFF) as u8;
                    if char2 != 0 {
                        self.console.write_char(char2)?;
                    }

                    address += 1;
                }
            }
            TrapCode::HALT => {
                self.console.write_str("HALT\n")?;
                self.running = false;
            }
        }

        Ok(())
    }
}