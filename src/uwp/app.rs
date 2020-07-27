use bindings::{
    windows::{
        application_model::core::{
            CoreApplication, CoreApplicationView, IFrameworkViewSource, IFrameworkView,
            abi_IFrameworkViewSource, abi_IFrameworkView,
        },
        foundation::TypedEventHandler,
        foundation::numerics::Vector2,
        ui::Colors,
        ui::core::{
            CoreDispatcher, CoreWindow, CoreProcessEventsOption, WindowSizeChangedEventArgs, 
            PointerEventArgs
        },
        ui::composition::{
            Compositor, CompositionTarget
        }
    }
};
use std::sync::{Arc, Mutex};
use crate::uwp::app_adapter::UwpApp;
use crate::minesweeper::Minesweeper;

struct AppState {
    window: CoreWindow,
    compositor: Compositor,
    target: CompositionTarget,

    game: Minesweeper,
}

pub struct MinesweeperApp {
    state: Arc<Mutex<Option<AppState>>>
}

impl MinesweeperApp {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(None)),
        }
    }
}

impl UwpApp for MinesweeperApp {
    fn initialize(&mut self, window: &CoreApplicationView) -> winrt::Result<()> {
        Ok(())
    }

    fn set_window(&mut self, window: &CoreWindow) -> winrt::Result<()> {
        Ok(())
    }

    fn load(&mut self, entry_point: &winrt::HString) -> winrt::Result<()> {
        Ok(())
    }

    fn run(&mut self) -> winrt::Result<()> {
        let window = CoreWindow::get_for_current_thread()?;

        // Init Composition
        let compositor = Compositor::new()?;
        let root = compositor.create_container_visual()?;
        root.set_relative_size_adjustment(Vector2 { x:1.0, y:1.0 })?;
        let target = compositor.create_target_for_current_view()?;
        target.set_root(&root)?;

        // Init minesweeper
        let window_size = get_window_size(&window)?;
        let game = Minesweeper::new(&root, &window_size)?;

        // Initialize our internal state
        let state = AppState {
            window: window.clone(),
            compositor,
            target,

            game,
        };
        self.state.lock().unwrap().replace(state);

        // Hook events
        type SizeChangedHandler = TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>;
        type PointerMovedHandler = TypedEventHandler<CoreWindow, PointerEventArgs>;
        type PointerPressedHandler = TypedEventHandler<CoreWindow, PointerEventArgs>;

        let size_changed_handler = SizeChangedHandler::new({
            let mut state = self.state.clone(); 
            move |sender, args| {
                let size = args.size()?;
                let size = Vector2{ x: size.width as f32, y: size.height as f32 };
                let mut state = state.lock().unwrap();
                let state = state.as_mut().unwrap();
                let game = &mut state.game;
                game.on_parent_size_changed(&size)?;
                Ok(())
            }
        });
        let pointer_moved_handler = PointerMovedHandler::new({
            let mut state = self.state.clone(); 
            move |sender, args| {
                let point = args.current_point()?.position()?;
                let point = Vector2{ x: point.x as f32, y: point.y as f32 };
                let mut state = state.lock().unwrap();
                let state = state.as_mut().unwrap();
                let game = &mut state.game;
                game.on_pointer_moved(&point)?;
                Ok(())
            }
        });
        let pointer_pressed_handler = PointerPressedHandler::new({
            let mut state = self.state.clone(); 
            move |sender, args| {
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