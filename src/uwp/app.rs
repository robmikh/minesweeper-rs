use crate::minesweeper::Minesweeper;
use bindings::*;
use bindings::windows::{
    application_model::core::{CoreApplicationView, IFrameworkView},
    foundation::numerics::Vector2,
    foundation::TypedEventHandler,
    ui::composition::{CompositionTarget, Compositor},
    ui::core::{CoreProcessEventsOption, CoreWindow, PointerEventArgs, WindowSizeChangedEventArgs},
};
use std::sync::{Arc, Mutex};

struct AppState {
    _window: CoreWindow,
    _compositor: Compositor,
    _target: CompositionTarget,

    game: Minesweeper,
}

#[winrt::implement(windows::application_model::core::IFrameworkViewSource)]
pub struct MinesweeperAppSource {}

impl MinesweeperAppSource {
    fn create_view(&mut self) -> winrt::Result<IFrameworkView> {
        let app = MinesweeperApp {
            state: Arc::new(Mutex::new(None)),
        };
        let view: IFrameworkView = app.into();
        Ok(view)
    }
}

// TOOD: A way to do this without the arc/mutex?
#[winrt::implement(windows::application_model::core::IFrameworkView)]
pub struct MinesweeperApp {
    state: Arc<Mutex<Option<AppState>>>,
}

impl MinesweeperApp {
    fn initialize(&mut self, _window: &Option<CoreApplicationView>) -> winrt::Result<()> {
        Ok(())
    }

    fn set_window(&mut self, _window: &Option<CoreWindow>) -> winrt::Result<()> {
        Ok(())
    }

    fn load(&mut self, _entry_point: &winrt::HString) -> winrt::Result<()> {
        Ok(())
    }

    fn run(&mut self) -> winrt::Result<()> {
        let window = CoreWindow::get_for_current_thread()?;

        // Init Composition
        let compositor = Compositor::new()?;
        let root = compositor.create_container_visual()?;
        root.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
        let target = compositor.create_target_for_current_view()?;
        target.set_root(&root)?;

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
                let size = args.size()?;
                let size = Vector2 {
                    x: size.width as f32,
                    y: size.height as f32,
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
                let point = args.current_point()?.position()?;
                let point = Vector2 {
                    x: point.x as f32,
                    y: point.y as f32,
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
                let properties = args.current_point()?.properties()?;
                let is_right = properties.is_right_button_pressed()?;
                let is_eraser = properties.is_eraser()?;
                let mut state = state.lock().unwrap();
                let state = state.as_mut().unwrap();
                let game = &mut state.game;
                game.on_pointer_pressed(is_right, is_eraser)?;
                Ok(())
            }
        });

        window.size_changed(size_changed_handler)?;
        window.pointer_moved(pointer_moved_handler)?;
        window.pointer_pressed(pointer_pressed_handler)?;

        // Activate the window and start running the dispatcher
        window.activate()?;

        let dispatcher = window.dispatcher()?;
        dispatcher.process_events(CoreProcessEventsOption::ProcessUntilQuit)?;
        
        Ok(())
    }

    fn uninitialize(&mut self) -> winrt::Result<()> {
        Ok(())
    }
}

fn get_window_size(window: &CoreWindow) -> winrt::Result<Vector2> {
    let bounds = window.bounds()?;
    Ok(Vector2 {
        x: bounds.width as f32,
        y: bounds.height as f32,
    })
}
