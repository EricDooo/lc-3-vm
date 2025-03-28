use std::io;
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::wincon::{ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, FlushConsoleInputBuffer};
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winnt::HANDLE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winbase::WAIT_OBJECT_0;
use winapi::shared::minwindef::DWORD;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_INPUT_HANDLE;

/// Windows-specific implementation of platform I/O operations
pub struct WindowsPlatform {
    stdin_handle: HANDLE,
    old_mode: DWORD,
}

impl WindowsPlatform {
    pub fn new() -> Self {
        WindowsPlatform {
            stdin_handle: INVALID_HANDLE_VALUE,
            old_mode: 0,
        }
    }

    pub fn disable_input_buffering(&mut self) -> io::Result<()> {
        unsafe {
            // Get the standard input handle
            self.stdin_handle = GetStdHandle(STD_INPUT_HANDLE);
            if self.stdin_handle == INVALID_HANDLE_VALUE {
                return Err(io::Error::last_os_error());
            }

            // Save the current console mode
            let mut old_mode: DWORD = 0;
            if GetConsoleMode(self.stdin_handle, &mut old_mode) == 0 {
                return Err(io::Error::last_os_error());
            }
            self.old_mode = old_mode;

            // Disable line input and echoing
            let new_mode = old_mode & !(ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT);
            if SetConsoleMode(self.stdin_handle, new_mode) == 0 {
                return Err(io::Error::last_os_error());
            }

            // Clear any pending input
            if FlushConsoleInputBuffer(self.stdin_handle) == 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    pub fn restore_input_buffering(&mut self) -> io::Result<()> {
        unsafe {
            if self.stdin_handle != INVALID_HANDLE_VALUE {
                if SetConsoleMode(self.stdin_handle, self.old_mode) == 0 {
                    return Err(io::Error::last_os_error());
                }
            }
        }
        Ok(())
    }

    pub fn check_key(&mut self) -> io::Result<bool> {
        unsafe {
            let result = WaitForSingleObject(self.stdin_handle, 0);
            if result == WAIT_OBJECT_0 {
                extern "C" {
                    fn _kbhit() -> i32;
                }
                Ok(_kbhit() != 0)
            } else {
                Ok(false)
            }
        }
    }

    pub fn read_key(&mut self) -> io::Result<u8> {
        extern "C" {
            fn getchar() -> i32;
        }
        let c = unsafe { getchar() };
        if c == -1 {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to read character"))
        } else {
            Ok(c as u8)
        }
    }
}