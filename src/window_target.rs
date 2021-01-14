use bindings::windows::win32::winrt::ICompositorDesktopInterop;
use bindings::windows::ui::composition::{desktop::DesktopWindowTarget, Compositor};
use raw_window_handle::HasRawWindowHandle;
use winrt::Interface;

pub trait CompositionDesktopWindowTargetSource {
    fn create_window_target(
        &self,
        compositor: &Compositor,
        is_topmost: bool,
    ) -> winrt::Result<DesktopWindowTarget>;
}

impl<T> CompositionDesktopWindowTargetSource for T
where
    T: HasRawWindowHandle,
{
    fn create_window_target(
        &self,
        compositor: &Compositor,
        is_topmost: bool,
    ) -> winrt::Result<DesktopWindowTarget> {
        // Get the window handle
        let window_handle = self.raw_window_handle();
        let window_handle = match window_handle {
            raw_window_handle::RawWindowHandle::Windows(window_handle) => window_handle.hwnd,
            _ => panic!("Unsupported platform!"),
        };

        let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;
        let mut result = None;
        let hr = compositor_desktop.CreateDesktopWindowTarget(window_handle as isize, is_topmost as i32, &mut result);
        winrt::ErrorCode(hr as u32).ok()?;
        Ok(result.unwrap())
    }
}
