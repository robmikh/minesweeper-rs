use crate::windows::{
    system::DispatcherQueueController, ui::composition::desktop::DesktopWindowTarget,
};
use winrt::AbiTransferable;

#[repr(C)]
pub struct abi_ICompositorDesktopInterop {
    __base: [usize; 3],
    create_desktop_window_target: unsafe extern "system" fn(
        winrt::NonNullRawComPtr<CompositorDesktopInterop>,
        winrt::RawPtr,
        bool,
        *mut <DesktopWindowTarget as AbiTransferable>::Abi,
    ) -> winrt::ErrorCode,
}

unsafe impl winrt::ComInterface for CompositorDesktopInterop {
    type VTable = abi_ICompositorDesktopInterop;

    fn iid() -> winrt::Guid {
        winrt::Guid::from_values(702976506, 17767, 19914, [179, 25, 208, 242, 7, 235, 104, 7])
    }
}

unsafe impl AbiTransferable for CompositorDesktopInterop {
    type Abi = winrt::RawComPtr<Self>;

    fn get_abi(&self) -> Self::Abi {
        self.ptr.get_abi()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set_abi()
    }
}

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct CompositorDesktopInterop {
    ptr: winrt::ComPtr<CompositorDesktopInterop>,
}

impl CompositorDesktopInterop {
    pub fn create_desktop_window_target(
        &self,
        hwnd: winrt::RawPtr,
        is_topmost: bool,
    ) -> winrt::Result<DesktopWindowTarget> {
        match self.get_abi() {
            None => panic!("The `this` pointer was null when calling method"),
            Some(this) => unsafe {
                let mut result: DesktopWindowTarget = std::mem::zeroed();
                (this.vtable().create_desktop_window_target)(
                    this,
                    hwnd,
                    is_topmost,
                    result.set_abi(),
                )
                .and_then(|| result)
            },
        }
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
        dispatcherQueueController: *mut <DispatcherQueueController as AbiTransferable>::Abi,
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
        let mut result: DispatcherQueueController = std::mem::zeroed();
        CreateDispatcherQueueController(options, result.set_abi()).ok()?;
        Ok(result)
    }
}

pub fn create_dispatcher_queue_controller_for_current_thread(
) -> winrt::Result<DispatcherQueueController> {
    create_dispatcher_queue_controller(
        DispatcherQueueThreadType::Current,
        DispatcherQueueThreadApartmentType::None,
    )
}
