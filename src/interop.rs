use bindings::windows::{
    system::DispatcherQueueController, ui::composition::desktop::DesktopWindowTarget,
};

// Note: This COM ABI code will be generated for you when this issue is completed:
// https://github.com/microsoft/winrt-rs/issues/81

#[repr(C)]
pub struct ICompositorDesktopInterop_vtable(
    usize,
    usize,
    usize,
    extern "system" fn(
        winrt::RawPtr,
        winrt::RawPtr,
        bool,
        &mut Option<DesktopWindowTarget>,
    ) -> winrt::ErrorCode,
);

unsafe impl winrt::Interface for ICompositorDesktopInterop {
    type Vtable = ICompositorDesktopInterop_vtable;

    const IID: winrt::Guid =
        winrt::Guid::from_values(702976506, 17767, 19914, [179, 25, 208, 242, 7, 235, 104, 7]);
}

#[repr(transparent)]
#[derive(Clone, PartialEq)]
pub struct ICompositorDesktopInterop(winrt::IUnknown);

impl ICompositorDesktopInterop {
    pub fn create_desktop_window_target(
        &self,
        hwnd: winrt::RawPtr,
        is_topmost: bool,
    ) -> winrt::Result<DesktopWindowTarget> {
        use winrt::{Abi, Interface};
        let mut result = None;
        unsafe { (self.vtable().3)(self.abi(), hwnd, is_topmost, &mut result).and_some(result) }
    }
}

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

pub fn ro_initialize(init_type: RoInitType) -> winrt::Result<()> {
    unsafe { RoInitialize(init_type).ok() }
}

#[link(name = "coremessaging")]
extern "stdcall" {
    fn CreateDispatcherQueueController(
        options: DispatcherQueueOptions,
        dispatcherQueueController: &mut Option<DispatcherQueueController>,
    ) -> winrt::ErrorCode;
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
) -> winrt::Result<DispatcherQueueController> {
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
) -> winrt::Result<DispatcherQueueController> {
    create_dispatcher_queue_controller(
        DispatcherQueueThreadType::Current,
        DispatcherQueueThreadApartmentType::None,
    )
}
