use std::io::{self, Write};
use crate::io::platform::Platform;

/// Console abstraction for handling input/output operations
pub struct Console {
    platform: Platform,
}

impl Console {
    pub fn new() -> Self {
        Console {
            platform: Platform::new(),
        }
    }

    /// Prepare the console for raw input mode
    pub fn setup(&mut self) -> io::Result<()> {
        self.platform.disable_input_buffering()
    }

    /// Restore the console to its original state
    pub fn cleanup(&mut self) -> io::Result<()> {
        self.platform.restore_input_buffering()
    }

    /// Check if a key is available without blocking
    pub fn check_key(&mut self) -> io::Result<bool> {
        self.platform.check_key()
    }

    /// Read a single key from the keyboard
    pub fn read_key(&mut self) -> io::Result<u8> {
        self.platform.read_key()
    }

    /// Write a single character to the console
    pub fn write_char(&mut self, c: u8) -> io::Result<()> {
        io::stdout().write_all(&[c])?;
        io::stdout().flush()
    }

    /// Write a string to the console
    pub fn write_str(&mut self, s: &str) -> io::Result<()> {
        io::stdout().write_all(s.as_bytes())?;
        io::stdout().flush()
    }
}