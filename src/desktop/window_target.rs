use crate::desktop::interop::CompositorDesktopInterop;
use bindings::windows::ui::composition::{desktop::DesktopWindowTarget, Compositor};
use raw_window_handle::HasRawWindowHandle;
use winrt::TryInto;

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

        let compositor_desktop: CompositorDesktopInterop = compositor.try_into()?;
        let target = compositor_desktop.create_desktop_window_target(window_handle, is_topmost)?;
        Ok(target)
    }
}
