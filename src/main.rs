#![windows_subsystem = "windows"]

mod comp_assets;
mod comp_ui;
mod minesweeper;
mod numerics;
mod visual_grid;
mod uwp;
#[cfg(target_vendor = "pc")]
mod desktop;

#[cfg(target_vendor = "pc")]
use desktop::run;
#[cfg(target_vendor = "uwp")]
use uwp::run;

// TODO: Validate that this works
#[cfg(target_vendor = "uwp")]
fn test() {
    println!("UWP");
}

#[cfg(target_vendor = "pc")]
fn test() {
    println!("Win32")
}

fn main() {
    let result = run();

    // We do this for nicer HRESULT printing when errors occur.
    if let Err(error) = result {
        error.code().unwrap();
    }
}


#[no_mangle]
extern "system" fn wWinMain(
    _h_instance: *const i32,
    _h_prev_instance: *const i32,
    _cmd_line: *const u16,
    _n_cmd_show: i32,
) -> i32 {
    match run() {
        Ok(()) => 0,
        Err(e) => e.code().0 as i32,
    }
}