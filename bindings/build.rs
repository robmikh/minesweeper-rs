fn main() {
    windows::build!(
        Windows::Foundation::Numerics::{Vector2, Vector3},
        Windows::Foundation::TimeSpan,
        Windows::Graphics::SizeInt32,
        Windows::System::DispatcherQueueController,
        Windows::UI::Composition::Desktop::DesktopWindowTarget,
        Windows::UI::Composition::{
            AnimationIterationBehavior, CompositionAnimation, CompositionBatchTypes,
            CompositionBorderMode, CompositionColorBrush, CompositionContainerShape,
            CompositionEllipseGeometry, CompositionGeometry, CompositionNineGridBrush,
            CompositionScopedBatch, CompositionShape, CompositionShapeCollection,
            CompositionSpriteShape, Compositor, ContainerVisual, ShapeVisual, SpriteVisual,
            Vector3KeyFrameAnimation, VisualCollection,
        },
        Windows::UI::{Color, Colors},
        Windows::Win32::Foundation::{BOOL, HWND},
        Windows::Win32::System::WinRT::{
            ICompositorDesktopInterop, CreateDispatcherQueueController,
            DISPATCHERQUEUE_THREAD_TYPE, DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
        },
    );
}
