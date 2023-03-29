use windows::{
    core::Result,
    Foundation::AsyncActionCompletedHandler,
    System::DispatcherQueueController,
    Win32::{
        System::WinRT::{
            CreateDispatcherQueueController, DispatcherQueueOptions,
            DISPATCHERQUEUE_THREAD_APARTMENTTYPE, DISPATCHERQUEUE_THREAD_TYPE, DQTAT_COM_NONE,
            DQTYPE_THREAD_CURRENT,
        },
        UI::WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, PostQuitMessage, TranslateMessage, MSG,
        },
    },
};

pub fn create_dispatcher_queue_controller(
    thread_type: DISPATCHERQUEUE_THREAD_TYPE,
    apartment_type: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
) -> Result<DispatcherQueueController> {
    let options = DispatcherQueueOptions {
        dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
        threadType: thread_type,
        apartmentType: apartment_type,
    };
    unsafe { CreateDispatcherQueueController(options) }
}

pub fn create_dispatcher_queue_controller_for_current_thread() -> Result<DispatcherQueueController>
{
    create_dispatcher_queue_controller(DQTYPE_THREAD_CURRENT, DQTAT_COM_NONE)
}

pub fn shutdown_dispatcher_queue_controller_and_wait(
    controller: &DispatcherQueueController,
    exit_code: i32,
) -> Result<i32> {
    let async_action = controller.ShutdownQueueAsync()?;
    async_action.SetCompleted(&AsyncActionCompletedHandler::new(
        move |_, _| -> Result<()> {
            unsafe { PostQuitMessage(exit_code) };
            Ok(())
        },
    ))?;

    let mut message = MSG::default();
    unsafe {
        while GetMessageW(&mut message, None, 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
    Ok(message.wParam.0 as i32)
}

pub fn shutdown_dispatcher_queue_controller_and_exit(
    controller: &DispatcherQueueController,
    exit_code: i32,
) -> ! {
    let exit_code = shutdown_dispatcher_queue_controller_and_wait(controller, exit_code)
        .expect("Failed to shutdown DispatcherQueueController!");
    std::process::exit(exit_code)
}
