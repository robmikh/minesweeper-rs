#![windows_subsystem = "windows"]

mod comp_assets;
mod comp_ui;
mod handle;
mod interop;
mod minesweeper;
mod numerics;
mod visual_grid;
mod window;

use interop::{
    create_dispatcher_queue_controller_for_current_thread,
    shutdown_dispatcher_queue_controller_and_exit,
};
use minesweeper::Minesweeper;
use window::Window;

use windows::{
    core::Result,
    Foundation::Numerics::Vector2,
    Win32::{
        System::WinRT::{RoInitialize, RO_INIT_SINGLETHREADED},
        UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, TranslateMessage, MSG},
    },
    UI::Composition::Compositor,
};

fn run() -> Result<()> {
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };
    let controller = create_dispatcher_queue_controller_for_current_thread()?;

    let window_width = 800;
    let window_height = 600;

    let window_size = Vector2 {
        X: window_width as f32,
        Y: window_height as f32,
    };

    let compositor = Compositor::new()?;
    let root = compositor.CreateContainerVisual()?;
    root.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;

    let game = Minesweeper::new(&root, &window_size)?;

    let window = Window::new("Minesweeper", window_width, window_height, game)?;
    let target = window.create_window_target(&compositor, false)?;
    target.SetRoot(&root)?;

    let mut message = MSG::default();
    unsafe {
        while GetMessageW(&mut message, None, 0, 0).into() {
            _ = TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
    shutdown_dispatcher_queue_controller_and_exit(&controller, message.wParam.0 as i32);
}

fn main() {
    let result = run();

    // We do this for nicer HRESULT printing when errors occur.
    if let Err(error) = result {
        error.code().unwrap();
    }
}
