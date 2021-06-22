use bindings::Windows::Win32::Foundation::HWND;
use bindings::Windows::Win32::System::WinRT::ICompositorDesktopInterop;
use bindings::Windows::UI::Composition::{Compositor, Desktop::DesktopWindowTarget};
use raw_window_handle::HasRawWindowHandle;
use windows::Interface;

pub trait CompositionDesktopWindowTargetSource {
    fn create_window_target(
        &self,
        compositor: &Compositor,
        is_topmost: bool,
    ) -> windows::Result<DesktopWindowTarget>;
}

impl<T> CompositionDesktopWindowTargetSource for T
where
    T: HasRawWindowHandle,
{
    fn create_window_target(
        &self,
        compositor: &Compositor,
        is_topmost: bool,
    ) -> windows::Result<DesktopWindowTarget> {
        // Get the window handle
        let window_handle = self.raw_window_handle();
        let window_handle = match window_handle {
            raw_window_handle::RawWindowHandle::Windows(window_handle) => window_handle.hwnd,
            _ => panic!("Unsupported platform!"),
        };

        let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;
        let mut result = None;

        unsafe {
            compositor_desktop
                .CreateDesktopWindowTarget(HWND(window_handle as isize), is_topmost, &mut result)
                .and_some(result)
        }
    }
}
