use crate::uwp::app::MinesweeperAppSource;
use bindings::windows::application_model::core::{CoreApplication, IFrameworkViewSource};

pub fn run() -> winrt::Result<()> {
    let app_source = MinesweeperAppSource {};
    let view_source: IFrameworkViewSource = app_source.into();
    CoreApplication::run(&view_source)?;
    Ok(())
}
