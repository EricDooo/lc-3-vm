use std::io;
use crate::io::console::Console;
use super::MEMORY_SIZE;

/// Memory-mapped registers for I/O operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryMappedRegister {
    KBSR = 0xFE00, // Keyboard status
    KBDR = 0xFE02, // Keyboard data
}

/// Memory subsystem for the LC-3 VM
pub struct Memory {
    data: [u16; MEMORY_SIZE],
}

impl Memory {
    /// Creates a new memory instance with zeroed memory
    pub fn new() -> Self {
        Memory {
            data: [0; MEMORY_SIZE],
        }
    }

    /// Reads a word from memory, handling memory-mapped registers
    pub fn read(&mut self, address: u16, console: &mut Console) -> io::Result<u16> {
        match address {
            addr if addr == MemoryMappedRegister::KBSR as u16 => {
                if console.check_key()? {
                    self.data[MemoryMappedRegister::KBSR as usize] = 1 << 15;
                    self.data[MemoryMappedRegister::KBDR as usize] = console.read_key()? as u16;
                } else {
                    self.data[MemoryMappedRegister::KBSR as usize] = 0;
                }
                Ok(self.data[MemoryMappedRegister::KBSR as usize])
            }
            _ => Ok(self.data[address as usize]),
        }
    }

    /// Writes a word to memory
    pub fn write(&mut self, address: u16, value: u16) {
        self.data[address as usize] = value;
    }

    /// Returns a slice to memory starting at the given address
    pub fn get_ptr(&self, address: u16) -> &[u16] {
        &self.data[address as usize..]
    }
}