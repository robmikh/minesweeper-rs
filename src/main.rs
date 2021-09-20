#![windows_subsystem = "windows"]

mod comp_assets;
mod comp_ui;
mod interop;
mod minesweeper;
mod numerics;
mod visual_grid;
mod wide_string;
mod window;

use interop::create_dispatcher_queue_controller_for_current_thread;
use minesweeper::Minesweeper;
use window::Window;

use bindings::Windows::{
    Foundation::Numerics::Vector2,
    Win32::{
        Foundation::HWND,
        System::WinRT::{RoInitialize, RO_INIT_SINGLETHREADED},
        UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, TranslateMessage, MSG},
    },
    UI::Composition::Compositor,
};

fn run() -> windows::Result<()> {
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED)? };
    let _controller = create_dispatcher_queue_controller_for_current_thread()?;

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
        while GetMessageW(&mut message, HWND(0), 0, 0).into() {
            TranslateMessage(&mut message);
            DispatchMessageW(&mut message);
        }
    }

    Ok(())
}

fn main() {
    let result = run();

    // We do this for nicer HRESULT printing when errors occur.
    if let Err(error) = result {
        error.code().unwrap();
    }
}
