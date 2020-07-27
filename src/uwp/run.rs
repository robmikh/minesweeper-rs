use crate::uwp::app::MinesweeperApp;
use crate::uwp::app_adapter::AppView;
use bindings::windows::application_model::core::CoreApplication;

pub fn run() -> winrt::Result<()> {
    let view_source = AppView::create_view_source(Box::new(MinesweeperApp::new()))?;
    CoreApplication::run(&view_source)?;
    Ok(())
}
