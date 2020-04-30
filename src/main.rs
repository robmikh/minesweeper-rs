winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation.collections"
        "windows.foundation.numerics"
        "windows.ui"
        "windows.ui.composition"
        "windows.ui.composition.desktop"
        "windows.graphics"
        "windows.system"
);

mod interop;
mod minesweeper;
mod numerics;
mod time_span;

use interop::{
    create_dispatcher_queue_controller_for_current_thread, ro_initialize, CompositorDesktopInterop,
    RoInitType,
};
use minesweeper::Minesweeper;
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use windows::{foundation::numerics::Vector2, ui::composition::Compositor};
use winrt::TryInto;

fn run() -> winrt::Result<()> {
    ro_initialize(RoInitType::MultiThreaded)?;
    let _controller = create_dispatcher_queue_controller_for_current_thread()?;

    bad change test

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Minesweeper");

    // Get the window handle
    let window_handle = window.raw_window_handle();
    let window_handle = match window_handle {
        raw_window_handle::RawWindowHandle::Windows(window_handle) => window_handle.hwnd,
        _ => panic!("Unsupported platform!"),
    };

    let compositor = Compositor::new()?;
    let compositor_desktop: CompositorDesktopInterop = compositor.try_into()?;
    let target = compositor_desktop.create_desktop_window_target(window_handle, false)?;

    let root = compositor.create_container_visual()?;
    root.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
    target.set_root(&root)?;

    let window_size = window.inner_size();
    let window_size = Vector2 {
        x: window_size.width as f32,
        y: window_size.height as f32,
    };
    let mut game = Minesweeper::new(&root, &window_size)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let size = Vector2 {
                    x: size.width as f32,
                    y: size.height as f32,
                };
                game.on_parent_size_changed(&size).unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let point = Vector2 {
                    x: position.x as f32,
                    y: position.y as f32,
                };
                game.on_pointer_moved(&point).unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if state == ElementState::Pressed {
                    game.on_pointer_pressed(button == MouseButton::Right, false)
                        .unwrap();
                }
            }
            _ => (),
        }
    });
}

fn main() {
    let result = run();

    // We do this for nicer HRESULT printing when errors occur.
    if let Err(error) = result {
        error.code().unwrap();
    }
}
