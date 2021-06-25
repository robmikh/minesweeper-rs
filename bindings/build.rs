fn main() {
    windows::build! {
        Windows::Graphics::SizeInt32,
        Windows::Win32::System::WinRT::{
            CreateDispatcherQueueController, ICompositorDesktopInterop,
        },
        Windows::UI::Colors,
        Windows::UI::Composition::Desktop::DesktopWindowTarget,
        Windows::UI::Composition::{
            CompositionColorBrush, CompositionContainerShape, CompositionEllipseGeometry,
            CompositionNineGridBrush, CompositionScopedBatch, CompositionShapeCollection,
            CompositionSpriteShape, Compositor, ShapeVisual, SpriteVisual,
            Vector3KeyFrameAnimation, VisualCollection,
        },
    };
}
