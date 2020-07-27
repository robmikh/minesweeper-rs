use bindings::{
    windows::{
        application_model::core::{
            CoreApplication, CoreApplicationView, IFrameworkViewSource, IFrameworkView,
            abi_IFrameworkViewSource, abi_IFrameworkView,
        },
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
use std::rc::Rc;
use winrt::AbiTransferable;
use winrt_guid::winrt_guid;
use crate::uwp::app_adapter::UwpApp;
use crate::minesweeper::Minesweeper;

struct AppState {
    window: CoreWindow,
    compositor: Compositor,
    target: CompositionTarget,

    game: Minesweeper,
}

pub struct MinesweeperApp {
    state: Option<AppState>
}

impl MinesweeperApp {
    pub fn new() -> Self {
        Self {
            state: None,
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
        let mut game = Minesweeper::new(&root, &window_size)?;

        // TODO: Hook events

        let state = AppState {
            window,
            compositor,
            target,

            game,
        };
        self.state = Some(state);
        
        let window = &self.state.as_ref().unwrap().window;
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