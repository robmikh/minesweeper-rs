fn main() {
    windows::build!(
        windows::foundation::numerics::{Vector2, Vector3},
        windows::foundation::TimeSpan,
        windows::graphics::SizeInt32,
        windows::system::DispatcherQueueController,
        windows::ui::composition::desktop::DesktopWindowTarget,
        windows::ui::composition::{
            AnimationIterationBehavior, CompositionAnimation, CompositionBatchTypes,
            CompositionBorderMode, CompositionColorBrush, CompositionContainerShape,
            CompositionEllipseGeometry, CompositionGeometry, CompositionNineGridBrush,
            CompositionScopedBatch, CompositionShape, CompositionShapeCollection,
            CompositionSpriteShape, Compositor, ContainerVisual, ShapeVisual, SpriteVisual,
            Vector3KeyFrameAnimation, VisualCollection,
        },
        windows::ui::{Color, Colors},
        windows::win32::system_services::{CreateDispatcherQueueController, BOOL},
        windows::win32::windows_and_messaging::HWND,
        windows::win32::winrt::ICompositorDesktopInterop,
    );
}
