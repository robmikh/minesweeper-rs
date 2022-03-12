use windows::core::PCWSTR;

pub struct WideString(pub Vec<u16>);

pub trait ToWide {
    fn to_wide(&self) -> WideString;
}

impl ToWide for &str {
    fn to_wide(&self) -> WideString {
        let mut result: Vec<u16> = self.encode_utf16().collect();
        result.push(0);
        WideString(result)
    }
}

impl WideString {
    pub fn as_pcwstr(&self) -> PCWSTR {
        PCWSTR(self.0.as_ptr() as *mut _)
    }
}
