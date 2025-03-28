use crate::vm::Register;

/// Trait for converting u16 values to Register enum
pub trait FromU16 {
    fn from_u16(value: u16) -> Self;
}

impl FromU16 for Register {
    fn from_u16(value: u16) -> Self {
        match value {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            _ => panic!("Invalid register index: {}", value),
        }
    }
}

/// Extends the sign bit of a value to fill the entire 16-bit word
pub fn sign_extend(x: u16, bit_count: u16) -> u16 {
    if ((x >> (bit_count - 1)) & 1) != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}