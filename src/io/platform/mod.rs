#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::WindowsPlatform as PlatformImpl;

#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
pub use self::unix::UnixPlatform as PlatformImpl;

use std::io;

/// Platform abstraction layer that handles platform-specific terminal operations
pub struct Platform {
    inner: PlatformImpl,
}

impl Platform {
    pub fn new() -> Self {
        Platform {
            inner: PlatformImpl::new(),
        }
    }

    /// Puts the terminal in raw mode for immediate character input
    pub fn disable_input_buffering(&mut self) -> io::Result<()> {
        self.inner.disable_input_buffering()
    }

    /// Restores the terminal to its original state
    pub fn restore_input_buffering(&mut self) -> io::Result<()> {
        self.inner.restore_input_buffering()
    }

    /// Checks if a key is available to be read without blocking
    pub fn check_key(&mut self) -> io::Result<bool> {
        self.inner.check_key()
    }

    /// Reads a single key from the keyboard
    pub fn read_key(&mut self) -> io::Result<u8> {
        self.inner.read_key()
    }
}