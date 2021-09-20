fn main() {
    windows::build! {
        Windows::Graphics::SizeInt32,
        Windows::Win32::System::WinRT::{
            CreateDispatcherQueueController, ICompositorDesktopInterop, RoInitialize,
        },
        Windows::UI::Colors,
        Windows::UI::Composition::Desktop::DesktopWindowTarget,
        Windows::UI::Composition::{
            CompositionColorBrush, CompositionContainerShape, CompositionEllipseGeometry,
            CompositionNineGridBrush, CompositionScopedBatch, CompositionShapeCollection,
            CompositionSpriteShape, Compositor, ShapeVisual, SpriteVisual,
            Vector3KeyFrameAnimation, VisualCollection,
        },
        Windows::Win32::Foundation::{
            RECT, BOOL, HINSTANCE, LRESULT, E_FAIL, E_HANDLE, HWND, WPARAM,
        },
        Windows::Win32::UI::WindowsAndMessaging::{
            WM_MOUSEMOVE, WM_SIZE, WM_SIZING, CW_USEDEFAULT, IDC_ARROW, WM_LBUTTONDOWN,
            WM_DESTROY, WINDOW_STYLE, WINDOW_EX_STYLE, WM_RBUTTONDOWN, WM_NCCREATE, WINDOW_LONG_PTR_INDEX,
            LoadCursorW, HMENU, WNDPROC, HCURSOR, CreateWindowExW, DefWindowProcW, DispatchMessageW,
            GetMessageW, PostQuitMessage, RegisterClassW, MSG, WNDCLASSW, TranslateMessage,
            CREATESTRUCTW, GetClientRect, AdjustWindowRectEx, ShowWindow, WINDOW_LONG_PTR_INDEX,
            SetWindowLongW, SetWindowLongPtrW, GetWindowLongW, GetWindowLongPtrW, PostQuitMessage,
        },
        Windows::Win32::System::LibraryLoader::GetModuleHandleW,
    };
}
