#![windows_subsystem = "windows"]

mod colors;
mod comp_assets;
mod comp_ui;
mod minesweeper;
mod numerics;
mod visual_grid;

use minesweeper::Minesweeper;
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;
use windows_composition::{Compositor, DispatcherQueueController, Result, Vector2};
use windows_window::Window;

const WINDOW_CLIENT_WIDTH: i32 = 800;
const WINDOW_CLIENT_HEIGHT: i32 = 600;
const WS_OVERLAPPEDWINDOW: u32 = 0x00CF_0000;
const WS_EX_NOREDIRECTIONBITMAP: u32 = 0x0020_0000;
const WM_MOUSEMOVE: u32 = 0x0200;
const WM_LBUTTONDOWN: u32 = 0x0201;
const WM_RBUTTONDOWN: u32 = 0x0204;

fn main() -> Result<()> {
    preserve_legacy_dpi_behavior();

    let _queue = DispatcherQueueController::create_on_current_thread()?;
    let compositor = Compositor::new()?;

    let root = compositor.create_container_visual();
    root.set_relative_size_adjustment(Vector2::new(1.0, 1.0));

    let game: Rc<RefCell<Option<Minesweeper>>> = Rc::new(RefCell::new(None));

    let window = {
        let game_message = game.clone();
        let game_resize = game.clone();
        let (window_width, window_height) =
            outer_size_for_client_size(WINDOW_CLIENT_WIDTH, WINDOW_CLIENT_HEIGHT)?;
        Window::new("Minesweeper")
            .size(window_width, window_height)
            .ex_style(WS_EX_NOREDIRECTIONBITMAP)
            .on_message(move |_hwnd, message, _wparam, lparam| {
                if let Some(game) = game_message.borrow_mut().as_mut() {
                    match message {
                        WM_MOUSEMOVE => game.on_pointer_moved(&point_from_lparam(lparam)).unwrap(),
                        WM_LBUTTONDOWN => game.on_pointer_pressed(false, false).unwrap(),
                        WM_RBUTTONDOWN => game.on_pointer_pressed(true, false).unwrap(),
                        _ => {}
                    }
                }
                None
            })
            .on_resize(move |width, height| {
                if let Some(game) = game_resize.borrow_mut().as_mut() {
                    game.on_parent_size_changed(&Vector2::new(width as f32, height as f32))
                        .unwrap();
                }
            })
            .create()?
    };

    let target = compositor.create_desktop_window_target(&window, false)?;
    target.set_root(&root);

    let (width, height) = window.client_size();
    *game.borrow_mut() = Some(Minesweeper::new(
        &root,
        &Vector2::new(width as f32, height as f32),
    )?);

    windows_window::run();
    Ok(())
}

fn point_from_lparam(lparam: isize) -> Vector2 {
    Vector2::new((lparam as i16) as f32, ((lparam >> 16) as i16) as f32)
}

fn preserve_legacy_dpi_behavior() {
    const DPI_AWARENESS_CONTEXT_UNAWARE: *mut c_void = -1_isize as *mut c_void;

    _ = unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_UNAWARE) };
}

fn outer_size_for_client_size(width: i32, height: i32) -> Result<(i32, i32)> {
    let mut rect = Rect {
        left: 0,
        top: 0,
        right: width,
        bottom: height,
    };

    if unsafe { AdjustWindowRectEx(&mut rect, WS_OVERLAPPEDWINDOW, 0, WS_EX_NOREDIRECTIONBITMAP) }
        == 0
    {
        Err(windows_core::Error::from_thread())
    } else {
        Ok((rect.right - rect.left, rect.bottom - rect.top))
    }
}

#[repr(C)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[link(name = "user32")]
extern "system" {
    fn AdjustWindowRectEx(rect: *mut Rect, style: u32, has_menu: i32, ex_style: u32) -> i32;
    fn SetProcessDpiAwarenessContext(value: *mut c_void) -> i32;
}
