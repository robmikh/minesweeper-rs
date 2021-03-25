use bindings::Windows::System::DispatcherQueueController;
use bindings::Windows::Win32::SystemServices::{
    CreateDispatcherQueueController, DispatcherQueueOptions, DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
    DISPATCHERQUEUE_THREAD_TYPE,
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
    create_dispatcher_queue_controller(
        DISPATCHERQUEUE_THREAD_TYPE::DQTYPE_THREAD_CURRENT,
        DISPATCHERQUEUE_THREAD_APARTMENTTYPE::DQTAT_COM_NONE,
    )
}
