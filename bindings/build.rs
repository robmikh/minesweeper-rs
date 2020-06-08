winrt::build!(
    dependencies
        os
    types
        windows::foundation::numerics::{Vector2, Vector3}
        windows::foundation::TimeSpan
        windows::graphics::SizeInt32
        windows::system::DispatcherQueueController
        windows::ui::composition::{
            AnimationIterationBehavior,
            CompositionBatchTypes,
            CompositionBorderMode,
            CompositionColorBrush,
            CompositionGeometry,
            CompositionShape,
            CompositionSpriteShape,
            Compositor,
            ContainerVisual,
            SpriteVisual,
        }
        windows::ui::composition::desktop::DesktopWindowTarget
        windows::ui::Colors
);

fn main() {
    build();
}
