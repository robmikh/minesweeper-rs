use bindings::windows::{
    system::DispatcherQueueController,
};

// Note: This COM ABI code will be generated for you when this issue is completed:
// https://github.com/microsoft/winrt-rs/issues/81

#[link(name = "coremessaging")]
extern "stdcall" {
    fn CreateDispatcherQueueController(
        options: DispatcherQueueOptions,
        dispatcherQueueController: &mut Option<DispatcherQueueController>,
    ) -> windows::ErrorCode;
}

#[repr(C)]
struct DispatcherQueueOptions {
    size: u32,
    thread_type: DispatcherQueueThreadType,
    apartment_type: DispatcherQueueThreadApartmentType,
}

#[allow(dead_code)]
#[repr(i32)]
pub enum DispatcherQueueThreadType {
    Dedicated = 1,
    Current = 2,
}

#[allow(dead_code)]
#[repr(i32)]
pub enum DispatcherQueueThreadApartmentType {
    None = 0,
    ASTA = 1,
    STA = 2,
}

pub fn create_dispatcher_queue_controller(
    thread_type: DispatcherQueueThreadType,
    apartment_type: DispatcherQueueThreadApartmentType,
) -> windows::Result<DispatcherQueueController> {
    let options = DispatcherQueueOptions {
        size: std::mem::size_of::<DispatcherQueueOptions>() as u32,
        thread_type,
        apartment_type,
    };
    unsafe {
        let mut result = None;
        CreateDispatcherQueueController(options, &mut result).and_some(result)
    }
}

pub fn create_dispatcher_queue_controller_for_current_thread(
) -> windows::Result<DispatcherQueueController> {
    create_dispatcher_queue_controller(
        DispatcherQueueThreadType::Current,
        DispatcherQueueThreadApartmentType::None,
    )
}
