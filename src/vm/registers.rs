/// LC-3 Register definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,    // Program Counter
    COND,  // Condition Register
    COUNT, // Count of registers
}

/// LC-3 Condition flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondFlag {
    POS = 1 << 0, // Positive
    ZRO = 1 << 1, // Zero
    NEG = 1 << 2, // Negative
}

/// Register file for the LC-3 VM
pub struct Registers {
    data: [u16; Register::COUNT as usize],
}

impl Registers {
    /// Creates a new register file with zeroed registers
    pub fn new() -> Self {
        Registers {
            data: [0; Register::COUNT as usize],
        }
    }

    /// Gets the value of a register
    pub fn get(&self, register: Register) -> u16 {
        self.data[register as usize]
    }

    /// Sets the value of a register
    pub fn set(&mut self, register: Register, value: u16) {
        self.data[register as usize] = value;
    }

    /// Updates condition flags based on the value in the specified register
    pub fn update_flags(&mut self, register: Register) {
        let value = self.get(register);

        let flag = if value == 0 {
            CondFlag::ZRO
        } else if (value >> 15) != 0 {
            CondFlag::NEG
        } else {
            CondFlag::POS
        };

        self.set_condition_flag(flag);
    }

    /// Sets the condition flag
    pub fn set_condition_flag(&mut self, flag: CondFlag) {
        self.data[Register::COND as usize] = flag as u16;
    }

    /// Gets the current condition flag
    pub fn get_condition_flag(&self) -> u16 {
        self.data[Register::COND as usize]
    }
}