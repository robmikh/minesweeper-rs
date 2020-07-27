#![windows_subsystem = "windows"]

mod comp_assets;
mod comp_ui;
mod interop;
mod minesweeper;
mod numerics;
mod visual_grid;

#[cfg(target_vendor = "pc")]
mod desktop;
#[cfg(target_vendor = "uwp")]
mod uwp;

#[cfg(target_vendor = "pc")]
use desktop::run;
#[cfg(target_vendor = "uwp")]
use uwp::run;

fn main() {
    let result = run();

    // We do this for nicer HRESULT printing when errors occur.
    if let Err(error) = result {
        error.code().unwrap();
    }
}
