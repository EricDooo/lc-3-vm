// src/lc3.rs
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

// Memory size: 2^16 locations
pub const MEMORY_SIZE: usize = 1 << 16;

// Register names
#[allow(dead_code)] // Allow unused variants as they're part of the LC-3 architecture
#[derive(Debug, Clone, Copy)]
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

// Condition flags
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CondFlag {
    POS = 1 << 0, // Positive
    ZRO = 1 << 1, // Zero
    NEG = 1 << 2, // Negative
}

// Op codes
#[allow(dead_code)] // Allow unused variants as they're part of the LC-3 architecture
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    BR = 0, // Branch
    ADD,    // Add
    LD,     // Load
    ST,     // Store
    JSR,    // Jump register
    AND,    // Bitwise and
    LDR,    // Load register
    STR,    // Store register
    RTI,    // Unused
    NOT,    // Bitwise not
    LDI,    // Load indirect
    STI,    // Store indirect
    JMP,    // Jump
    RES,    // Reserved (unused)
    LEA,    // Load effective address
    TRAP,   // Execute trap
}

// Trap codes
#[derive(Debug, Clone, Copy)]
pub enum TrapCode {
    GETC = 0x20,  // Get character from keyboard, not echoed
    OUT = 0x21,   // Output a character
    PUTS = 0x22,  // Output a word string
    IN = 0x23,    // Get character from keyboard, echoed
    PUTSP = 0x24, // Output a byte string
    HALT = 0x25,  // Halt the program
}

// Memory-mapped registers
#[derive(Debug, Clone, Copy)]
pub enum MemoryMappedRegister {
    KBSR = 0xFE00, // Keyboard status
    KBDR = 0xFE02, // Keyboard data
}

pub struct LC3 {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: [u16; Register::COUNT as usize],
    pub running: bool,
}

impl LC3 {
    pub fn new() -> Self {
        let mut vm = LC3 {
            memory: [0; MEMORY_SIZE],
            registers: [0; Register::COUNT as usize],
            running: false,
        };

        // Set PC to starting position
        // 0x3000 is the default starting position for LC-3 programs
        vm.registers[Register::PC as usize] = 0x3000;

        vm
    }

    pub fn update_flags(&mut self, r: usize) {
        let value = self.registers[r];

        if value == 0 {
            self.registers[Register::COND as usize] = CondFlag::ZRO as u16;
        } else if (value >> 15) != 0 {
            // Check if the sign bit is set
            self.registers[Register::COND as usize] = CondFlag::NEG as u16;
        } else {
            self.registers[Register::COND as usize] = CondFlag::POS as u16;
        }
    }

    pub fn sign_extend(x: u16, bit_count: u16) -> u16 {
        // Check if the most significant bit of the range is set (indicating negative)
        if ((x >> (bit_count - 1)) & 1) != 0 {
            // If negative, set all bits above the bit_count to 1
            // Make sure to use 0xFFFF to keep within u16 bounds
            x | (0xFFFF << bit_count)
        } else {
            // If positive, leave as is (no need to mask)
            x
        }
    }

    pub fn read_image_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = [0; 2];

        // Read the origin first
        file.read_exact(&mut buffer)?;
        let origin = u16::from_be_bytes(buffer);

        let mut i = origin as usize;
        while let Ok(_) = file.read_exact(&mut buffer) {
            self.memory[i] = u16::from_be_bytes(buffer);
            i += 1;
        }

        Ok(())
    }

    pub fn mem_read(&mut self, address: u16) -> u16 {
        // Handle memory-mapped registers and other special cases
        match address {
            a if a == MemoryMappedRegister::KBSR as u16 => {
                // Check keyboard status (simplified for now)
                let key_pressed = false;

                // Check if a key is pressed using a non-blocking check
                // This is simplified; a proper implementation would use platform-specific code
                // For example, using termion or similar crate for terminal IO

                if key_pressed {
                    self.memory[MemoryMappedRegister::KBSR as usize] = 1 << 15;
                    // Read the actual key in a proper implementation
                    self.memory[MemoryMappedRegister::KBDR as usize] = 0; // Placeholder
                } else {
                    self.memory[MemoryMappedRegister::KBSR as usize] = 0;
                }

                self.memory[MemoryMappedRegister::KBSR as usize]
            }
            _ => self.memory[address as usize],
        }
    }

    pub fn mem_write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }

    pub fn execute_instruction(&mut self, instr: u16) {
        // Extract the op code (first 4 bits)
        let op = (instr >> 12) as u8;

        match op {
            op if op == OpCode::ADD as u8 => {
                // Destination register (DR)
                let dr = (instr >> 9) & 0x7;
                // First operand (SR1)
                let sr1 = (instr >> 6) & 0x7;
                // Check immediate mode flag (bit 5)
                if (instr & 0x20) == 0 {
                    // Register mode
                    let sr2 = instr & 0x7;
                    self.registers[dr as usize] =
                        self.registers[sr1 as usize].wrapping_add(self.registers[sr2 as usize]);
                } else {
                    // Immediate mode
                    let imm5 = Self::sign_extend(instr & 0x1F, 5);
                    self.registers[dr as usize] = self.registers[sr1 as usize].wrapping_add(imm5);
                }
                self.update_flags(dr as usize);
            }
            op if op == OpCode::AND as u8 => {
                let dr = (instr >> 9) & 0x7;
                let sr1 = (instr >> 6) & 0x7;
                if (instr & 0x20) == 0 {
                    // Register mode
                    let sr2 = instr & 0x7;
                    self.registers[dr as usize] =
                        self.registers[sr1 as usize] & self.registers[sr2 as usize];
                } else {
                    // Immediate mode
                    let imm5 = Self::sign_extend(instr & 0x1F, 5);
                    self.registers[dr as usize] = self.registers[sr1 as usize] & imm5;
                }
                self.update_flags(dr as usize);
            }
            op if op == OpCode::NOT as u8 => {
                let dr = (instr >> 9) & 0x7;
                let sr = (instr >> 6) & 0x7;
                self.registers[dr as usize] = !self.registers[sr as usize];
                self.update_flags(dr as usize);
            }
            op if op == OpCode::BR as u8 => {
                let pc_offset = Self::sign_extend(instr & 0x1FF, 9);
                let cond_flag = (instr >> 9) & 0x7;
                if (cond_flag & self.registers[Register::COND as usize]) != 0 {
                    self.registers[Register::PC as usize] =
                        self.registers[Register::PC as usize].wrapping_add(pc_offset);
                }
            }
            op if op == OpCode::JMP as u8 => {
                // Also handles RET when BaseR is R7
                let base_r = (instr >> 6) & 0x7;
                self.registers[Register::PC as usize] = self.registers[base_r as usize];
            }
            op if op == OpCode::JSR as u8 => {
                let long_flag = (instr >> 11) & 1;
                // Save the current PC to R7
                self.registers[Register::R7 as usize] = self.registers[Register::PC as usize];

                if long_flag != 0 {
                    // JSR - PC-relative jump
                    let pc_offset = Self::sign_extend(instr & 0x7FF, 11);
                    self.registers[Register::PC as usize] =
                        self.registers[Register::PC as usize].wrapping_add(pc_offset);
                } else {
                    // JSRR - Register-based jump
                    let base_r = (instr >> 6) & 0x7;
                    self.registers[Register::PC as usize] = self.registers[base_r as usize];
                }
            }
            op if op == OpCode::LD as u8 => {
                let dr = (instr >> 9) & 0x7;
                let pc_offset = Self::sign_extend(instr & 0x1FF, 9);
                let address = self.registers[Register::PC as usize].wrapping_add(pc_offset);
                self.registers[dr as usize] = self.mem_read(address);
                self.update_flags(dr as usize);
            }
            op if op == OpCode::LDI as u8 => {
                let dr = (instr >> 9) & 0x7;
                let pc_offset = Self::sign_extend(instr & 0x1FF, 9);
                let address = self.registers[Register::PC as usize].wrapping_add(pc_offset);

                // Fix for borrow checker - get the indirect address first
                let indirect_address = self.mem_read(address);
                // Then read from that address
                self.registers[dr as usize] = self.mem_read(indirect_address);
                self.update_flags(dr as usize);
            }
            op if op == OpCode::LDR as u8 => {
                let dr = (instr >> 9) & 0x7;
                let base_r = (instr >> 6) & 0x7;
                let offset = Self::sign_extend(instr & 0x3F, 6);
                let address = self.registers[base_r as usize].wrapping_add(offset);
                self.registers[dr as usize] = self.mem_read(address);
                self.update_flags(dr as usize);
            }
            op if op == OpCode::LEA as u8 => {
                let dr = (instr >> 9) & 0x7;
                let pc_offset = Self::sign_extend(instr & 0x1FF, 9);
                self.registers[dr as usize] =
                    self.registers[Register::PC as usize].wrapping_add(pc_offset);
                self.update_flags(dr as usize);
            }
            op if op == OpCode::ST as u8 => {
                let sr = (instr >> 9) & 0x7;
                let pc_offset = Self::sign_extend(instr & 0x1FF, 9);
                let address = self.registers[Register::PC as usize].wrapping_add(pc_offset);
                self.mem_write(address, self.registers[sr as usize]);
            }
            op if op == OpCode::STI as u8 => {
                let sr = (instr >> 9) & 0x7;
                let pc_offset = Self::sign_extend(instr & 0x1FF, 9);
                let address = self.registers[Register::PC as usize].wrapping_add(pc_offset);

                // store value and get indirect address first
                let value = self.registers[sr as usize];
                let indirect_address = self.mem_read(address);

                // Then write to that address
                self.mem_write(indirect_address, value);
            }
            op if op == OpCode::STR as u8 => {
                let sr = (instr >> 9) & 0x7;
                let base_r = (instr >> 6) & 0x7;
                let offset = Self::sign_extend(instr & 0x3F, 6);
                let address = self.registers[base_r as usize].wrapping_add(offset);
                self.mem_write(address, self.registers[sr as usize]);
            }
            op if op == OpCode::TRAP as u8 => {
                match instr & 0xFF {
                    trap if trap == TrapCode::GETC as u16 => {
                        // Read a single character from the keyboard
                        // For now, just a simple implementation using stdin
                        let mut buffer = [0; 1];
                        io::stdin().read_exact(&mut buffer).unwrap();
                        self.registers[Register::R0 as usize] = buffer[0] as u16;
                    }
                    trap if trap == TrapCode::OUT as u16 => {
                        // Output a character
                        let c = self.registers[Register::R0 as usize] as u8;
                        io::stdout().write_all(&[c]).unwrap();
                        io::stdout().flush().unwrap();
                    }
                    trap if trap == TrapCode::PUTS as u16 => {
                        // Output a string (null-terminated)
                        let mut address = self.registers[Register::R0 as usize];
                        loop {
                            let c = self.mem_read(address);
                            if c == 0 {
                                break;
                            }
                            let character = (c & 0xFF) as u8;
                            io::stdout().write_all(&[character]).unwrap();
                            address += 1;
                        }
                        io::stdout().flush().unwrap();
                    }
                    trap if trap == TrapCode::IN as u16 => {
                        // Input a character and echo it
                        print!("Enter a character: ");
                        io::stdout().flush().unwrap();

                        let mut buffer = [0; 1];
                        io::stdin().read_exact(&mut buffer).unwrap();
                        let c = buffer[0] as u16;

                        self.registers[Register::R0 as usize] = c;

                        // Echo the character
                        io::stdout().write_all(&[c as u8]).unwrap();
                        io::stdout().flush().unwrap();
                    }
                    trap if trap == TrapCode::PUTSP as u16 => {
                        // Output a byte string (two characters per memory location)
                        let mut address = self.registers[Register::R0 as usize];
                        loop {
                            let value = self.mem_read(address);
                            if value == 0 {
                                break;
                            }

                            let char1 = (value & 0xFF) as u8;
                            io::stdout().write_all(&[char1]).unwrap();

                            let char2 = ((value >> 8) & 0xFF) as u8;
                            if char2 != 0 {
                                // Check if second character is null terminator
                                io::stdout().write_all(&[char2]).unwrap();
                            }

                            address += 1;
                        }
                        io::stdout().flush().unwrap();
                    }
                    trap if trap == TrapCode::HALT as u16 => {
                        // Halt execution
                        println!("HALT");
                        self.running = false;
                    }
                    _ => panic!("Unknown trap code: {}", instr & 0xFF),
                }
            }
            _ => panic!("Unknown opcode: {}", op),
        }
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            // Fetch instruction
            let pc = self.registers[Register::PC as usize];
            self.registers[Register::PC as usize] += 1; // Increment PC
            let instr = self.mem_read(pc);

            // Execute instruction
            self.execute_instruction(instr);
        }
    }
}
