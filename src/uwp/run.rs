use crate::uwp::app::MinesweeperAppSource;
use bindings::Windows::ApplicationModel::Core::{CoreApplication, IFrameworkViewSource};

pub fn run() -> windows::Result<()> {
    let app_source = MinesweeperAppSource {};
    let view_source: IFrameworkViewSource = app_source.into();
    CoreApplication::Run(&view_source)?;
    Ok(())
}
