use std::ffi::c_void;
use crate::windows::{
    ui::composition::{
        desktop::{
            DesktopWindowTarget,
        },
    },
    system::DispatcherQueueController,
};

#[repr(C)]
struct abi_ICompositorDesktopInterop {
    __base : [usize; 3],
    create_desktop_window_target: extern "system" fn(winrt::RawPtr, *mut c_void, i32, *mut winrt::RawPtr) -> winrt::ErrorCode,
    ensure_on_thread: extern "system" fn(winrt::RawPtr, u32) -> winrt::ErrorCode,
}

unsafe impl winrt::ComInterface for CompositorDesktopInterop {
    const GUID: winrt::Guid = winrt::Guid::from_values(
        702976506, 17767, 19914, [179, 25, 208, 242, 7, 235, 104, 7]
    );
}

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct CompositorDesktopInterop {
    ptr: winrt::IUnknown,
}

impl CompositorDesktopInterop {
    pub fn create_desktop_window_target(&self, hwnd: *mut c_void, is_topmost: bool) -> winrt::Result<DesktopWindowTarget> {
        unsafe {
            let mut __ok = std::mem::zeroed();
            ((*(*(self.ptr.get() as *const *const abi_ICompositorDesktopInterop))).create_desktop_window_target)(self.ptr.get(), hwnd, is_topmost as i32, &mut __ok)
                .and_then(|| {
                    let result = std::mem::transmute_copy(&__ok);
                    std::mem::forget(__ok);
                    result
                })
        }
    }
}

#[link(name = "windowsapp")]
extern "stdcall" {
    fn RoInitialize(
        init_type: i32,
    ) -> winrt::ErrorCode;
}

#[allow(dead_code)]
#[repr(i32)]
pub enum RoInitType {
    MultiThreaded = 0,
    SingleThreaded = 1,
}

pub fn ro_initialize(init_type: RoInitType) -> winrt::Result<()> {
    unsafe {
        RoInitialize(init_type as i32).ok()
    }
}

#[link(name = "coremessaging")]
extern "stdcall" {
    fn CreateDispatcherQueueController(
        options: DispatcherQueueOptions,
        dispatcherQueueController: *mut winrt::RawPtr,
    ) -> winrt::ErrorCode;
}

#[repr(C)]
struct DispatcherQueueOptions {
    size: u32,
    thread_type: i32,
    apartment_type: i32,
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

pub fn create_dispatcher_queue_controller(thread_type: DispatcherQueueThreadType, apartment_type: DispatcherQueueThreadApartmentType) -> winrt::Result<DispatcherQueueController> {
    unsafe {
        let options = DispatcherQueueOptions {
            size: std::mem::size_of::<DispatcherQueueOptions>() as u32,
            thread_type: thread_type as i32,
            apartment_type: apartment_type as i32,
        };
        let mut ptr = std::mem::zeroed();
        CreateDispatcherQueueController(options, &mut ptr).ok()?;
        let result = std::mem::transmute_copy(&ptr);
        std::mem::forget(ptr);
        Ok(result)
    }
}

pub fn create_dispatcher_queue_controller_for_current_thread() -> winrt::Result<DispatcherQueueController> {
    create_dispatcher_queue_controller(DispatcherQueueThreadType::Current, DispatcherQueueThreadApartmentType::None)
}