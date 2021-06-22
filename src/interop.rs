use bindings::Windows::System::DispatcherQueueController;
use bindings::Windows::Win32::System::WinRT::{
    CreateDispatcherQueueController, DispatcherQueueOptions, DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
    DISPATCHERQUEUE_THREAD_TYPE, DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT,
};

pub fn create_dispatcher_queue_controller(
    thread_type: DISPATCHERQUEUE_THREAD_TYPE,
    apartment_type: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
) -> windows::Result<DispatcherQueueController> {
    let options = DispatcherQueueOptions {
        dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
        threadType: thread_type,
        apartmentType: apartment_type,
    };
    unsafe {
        let mut result = None;
        CreateDispatcherQueueController(options, &mut result).and_some(result)
    }
}

pub fn create_dispatcher_queue_controller_for_current_thread(
) -> windows::Result<DispatcherQueueController> {
    create_dispatcher_queue_controller(DQTYPE_THREAD_CURRENT, DQTAT_COM_NONE)
}
