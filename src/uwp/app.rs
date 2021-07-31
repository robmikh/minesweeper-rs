use crate::minesweeper::Minesweeper;
use bindings::Windows::{
    ApplicationModel::Core::{CoreApplicationView, IFrameworkView},
    Foundation::Numerics::Vector2,
    Foundation::TypedEventHandler,
    UI::Composition::{CompositionTarget, Compositor},
    UI::Core::{CoreProcessEventsOption, CoreWindow, PointerEventArgs, WindowSizeChangedEventArgs},
};
use bindings::*;
use std::sync::{Arc, Mutex};
use windows::implement;

struct AppState {
    _window: CoreWindow,
    _compositor: Compositor,
    _target: CompositionTarget,

    game: Minesweeper,
}

#[implement(Windows::ApplicationModel::Core::IFrameworkViewSource)]
pub struct MinesweeperAppSource {}

#[allow(non_snake_case)]
impl MinesweeperAppSource {
    fn CreateView(&mut self) -> windows::Result<IFrameworkView> {
        let app = MinesweeperApp {
            state: Arc::new(Mutex::new(None)),
        };
        let view: IFrameworkView = app.into();
        Ok(view)
    }
}

// TOOD: A way to do this without the arc/mutex?
#[implement(Windows::ApplicationModel::Core::IFrameworkView)]
pub struct MinesweeperApp {
    state: Arc<Mutex<Option<AppState>>>,
}

#[allow(non_snake_case)]
impl MinesweeperApp {
    fn Initialize(&mut self, _window: &Option<CoreApplicationView>) -> windows::Result<()> {
        Ok(())
    }

    fn SetWindow(&mut self, _window: &Option<CoreWindow>) -> windows::Result<()> {
        Ok(())
    }

    fn Load(&mut self, _entry_point: &windows::HSTRING) -> windows::Result<()> {
        Ok(())
    }

    fn Run(&mut self) -> windows::Result<()> {
        let window = CoreWindow::GetForCurrentThread()?;

        // Init Composition
        let compositor = Compositor::new()?;
        let root = compositor.CreateContainerVisual()?;
        root.SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })?;
        let target = compositor.CreateTargetForCurrentView()?;
        target.SetRoot(&root)?;

        // Init minesweeper
        let window_size = get_window_size(&window)?;
        let game = Minesweeper::new(&root, &window_size)?;

        // Initialize our internal state
        let state = AppState {
            _window: window.clone(),
            _compositor: compositor,
            _target: target,

            game,
        };
        self.state.lock().unwrap().replace(state);

        // Hook events
        type SizeChangedHandler = TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>;
        type PointerMovedHandler = TypedEventHandler<CoreWindow, PointerEventArgs>;
        type PointerPressedHandler = TypedEventHandler<CoreWindow, PointerEventArgs>;

        let size_changed_handler = SizeChangedHandler::new({
            let state = self.state.clone();
            move |_sender, args| {
                let args = args.as_ref().unwrap();
                let size = args.Size()?;
                let size = Vector2 {
                    X: size.Width as f32,
                    Y: size.Height as f32,
                };
                let mut state = state.lock().unwrap();
                let state = state.as_mut().unwrap();
                let game = &mut state.game;
                game.on_parent_size_changed(&size)?;
                Ok(())
            }
        });
        let pointer_moved_handler = PointerMovedHandler::new({
            let state = self.state.clone();
            move |_sender, args| {
                let args = args.as_ref().unwrap();
                let point = args.CurrentPoint()?.Position()?;
                let point = Vector2 {
                    X: point.X as f32,
                    Y: point.Y as f32,
                };
                let mut state = state.lock().unwrap();
                let state = state.as_mut().unwrap();
                let game = &mut state.game;
                game.on_pointer_moved(&point)?;
                Ok(())
            }
        });
        let pointer_pressed_handler = PointerPressedHandler::new({
            let state = self.state.clone();
            move |_sender, args| {
                let args = args.as_ref().unwrap();
                let properties = args.CurrentPoint()?.Properties()?;
                let is_right = properties.IsRightButtonPressed()?;
                let is_eraser = properties.IsEraser()?;
                let mut state = state.lock().unwrap();
                let state = state.as_mut().unwrap();
                let game = &mut state.game;
                game.on_pointer_pressed(is_right, is_eraser)?;
                Ok(())
            }
        });

        window.SizeChanged(size_changed_handler)?;
        window.PointerMoved(pointer_moved_handler)?;
        window.PointerPressed(pointer_pressed_handler)?;

        // Activate the window and start running the dispatcher
        window.Activate()?;

        let dispatcher = window.Dispatcher()?;
        dispatcher.ProcessEvents(CoreProcessEventsOption::ProcessUntilQuit)?;

        Ok(())
    }

    fn Uninitialize(&mut self) -> windows::Result<()> {
        Ok(())
    }
}

fn get_window_size(window: &CoreWindow) -> windows::Result<Vector2> {
    let bounds = window.Bounds()?;
    Ok(Vector2 {
        X: bounds.Width as f32,
        Y: bounds.Height as f32,
    })
}
