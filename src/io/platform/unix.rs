use std::io;
use libc::{self, termios, STDIN_FILENO, TCSANOW, ECHO, ICANON, VMIN, VTIME};

/// Unix-specific implementation of platform I/O operations
pub struct UnixPlatform {
    original_termios: Option<termios>,
}

impl UnixPlatform {
    pub fn new() -> Self {
        UnixPlatform {
            original_termios: None,
        }
    }

    pub fn disable_input_buffering(&mut self) -> io::Result<()> {
        unsafe {
            let mut term: termios = std::mem::zeroed();
            if libc::tcgetattr(STDIN_FILENO, &mut term) != 0 {
                return Err(io::Error::last_os_error());
            }
            
            self.original_termios = Some(term);
            
            // Set up raw mode - no echo, no line buffering
            let mut raw = term;
            raw.c_lflag &= !(ECHO | ICANON);
            raw.c_cc[VMIN] = 0;  // Don't wait for input
            raw.c_cc[VTIME] = 0; // No timeout
            
            if libc::tcsetattr(STDIN_FILENO, TCSANOW, &raw) != 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    pub fn restore_input_buffering(&mut self) -> io::Result<()> {
        if let Some(term) = self.original_termios {
            unsafe {
                if libc::tcsetattr(STDIN_FILENO, TCSANOW, &term) != 0 {
                    return Err(io::Error::last_os_error());
                }
            }
        }
        Ok(())
    }

    pub fn check_key(&mut self) -> io::Result<bool> {
        unsafe {
            let mut readfds: libc::fd_set = std::mem::zeroed();
            libc::FD_ZERO(&mut readfds);
            libc::FD_SET(STDIN_FILENO, &mut readfds);
            
            let mut timeout: libc::timeval = std::mem::zeroed();
            timeout.tv_sec = 0;
            timeout.tv_usec = 0;
            
            let result = libc::select(STDIN_FILENO + 1, &mut readfds, std::ptr::null_mut(), std::ptr::null_mut(), &mut timeout);
            
            if result == -1 {
                return Err(io::Error::last_os_error());
            }
            
            Ok(result > 0 && libc::FD_ISSET(STDIN_FILENO, &readfds))
        }
    }

    pub fn read_key(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8; 1];
        let result = unsafe {
            libc::read(STDIN_FILENO, buffer.as_mut_ptr() as *mut libc::c_void, 1)
        };
        
        if result <= 0 {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to read character"))
        } else {
            Ok(buffer[0])
        }
    }
}