use windows::{
    core::{Error, Result},
    Win32::Foundation::{HINSTANCE, HWND},
};

pub trait CheckHandle: Sized {
    fn ok(self) -> Result<Self>;
}

impl CheckHandle for HWND {
    fn ok(self) -> Result<Self> {
        if self.0 == 0 {
            Err(Error::from_win32())
        } else {
            Ok(self)
        }
    }
}

impl CheckHandle for HINSTANCE {
    fn ok(self) -> Result<Self> {
        if self.0 == 0 {
            Err(Error::from_win32())
        } else {
            Ok(self)
        }
    }
}
