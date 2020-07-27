#[link(name = "windowsapp")]
extern "stdcall" {
    fn RoInitialize(init_type: RoInitType) -> winrt::ErrorCode;
}

#[allow(dead_code)]
#[repr(i32)]
pub enum RoInitType {
    MultiThreaded = 0,
    SingleThreaded = 1,
}

#[allow(dead_code)]
pub fn ro_initialize(init_type: RoInitType) -> winrt::Result<()> {
    unsafe { RoInitialize(init_type).ok() }
}
