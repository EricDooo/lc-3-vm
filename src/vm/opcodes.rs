/// LC-3 Operation Codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    BR = 0,  // Branch
    ADD,     // Add
    LD,      // Load
    ST,      // Store
    JSR,     // Jump register
    AND,     // Bitwise and
    LDR,     // Load register
    STR,     // Store register
    RTI,     // Unused
    NOT,     // Bitwise not
    LDI,     // Load indirect
    STI,     // Store indirect
    JMP,     // Jump
    RES,     // Reserved (unused)
    LEA,     // Load effective address
    TRAP,    // Execute trap
}

/// LC-3 Trap Codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrapCode {
    GETC = 0x20,  // Get character from keyboard, not echoed
    OUT = 0x21,   // Output a character
    PUTS = 0x22,  // Output a word string
    IN = 0x23,    // Get character from keyboard, echoed
    PUTSP = 0x24, // Output a byte string
    HALT = 0x25,  // Halt the program
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::BR,
            1 => OpCode::ADD,
            2 => OpCode::LD,
            3 => OpCode::ST,
            4 => OpCode::JSR,
            5 => OpCode::AND,
            6 => OpCode::LDR,
            7 => OpCode::STR,
            8 => OpCode::RTI,
            9 => OpCode::NOT,
            10 => OpCode::LDI,
            11 => OpCode::STI,
            12 => OpCode::JMP,
            13 => OpCode::RES,
            14 => OpCode::LEA,
            15 => OpCode::TRAP,
            _ => panic!("Invalid opcode: {}", value),
        }
    }
}

impl From<u16> for TrapCode {
    fn from(value: u16) -> Self {
        match value {
            0x20 => TrapCode::GETC,
            0x21 => TrapCode::OUT,
            0x22 => TrapCode::PUTS,
            0x23 => TrapCode::IN,
            0x24 => TrapCode::PUTSP,
            0x25 => TrapCode::HALT,
            _ => panic!("Invalid trap code: {}", value),
        }
    }
}