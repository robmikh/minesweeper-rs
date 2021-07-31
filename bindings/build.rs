fn main() {
    windows::build! {
        Windows::ApplicationModel::Core::{
            CoreApplication, CoreApplicationView, IFrameworkViewSource, IFrameworkView,
        },
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
        Windows::UI::Core::{
            CoreDispatcher, CoreWindow, CoreProcessEventsOption, WindowSizeChangedEventArgs,
            PointerEventArgs,
        },
        Windows::UI::Input::{
            PointerPoint, PointerPointProperties,
        },
    };
}
