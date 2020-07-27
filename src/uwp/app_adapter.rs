use bindings::windows::{
    application_model::core::{CoreApplicationView, IFrameworkView, IFrameworkViewSource},
    ui::core::CoreWindow,
};
use com::{co_class, com_interface, interfaces::iunknown::IUnknown};
use std::cell::RefCell;
use winrt::AbiTransferable;
use winrt::TryInto;

pub trait UwpApp {
    fn initialize(&mut self, window: &CoreApplicationView) -> winrt::Result<()>;
    fn set_window(&mut self, window: &CoreWindow) -> winrt::Result<()>;
    fn load(&mut self, entry_point: &winrt::HString) -> winrt::Result<()>;
    fn run(&mut self) -> winrt::Result<()>;
    fn uninitialize(&mut self) -> winrt::Result<()>;
}

#[com_interface("AF86E2E0-B12D-4c6a-9C5A-D7AA65101E90")]
pub trait IInspectable: IUnknown {
    unsafe fn get_iids(&self, iid_count: *mut u32, iids: *mut *mut winrt::Guid)
        -> winrt::ErrorCode;
    unsafe fn get_runtime_class_name(
        &self,
        class_name: *mut <winrt::HString as winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode;
    unsafe fn get_trust_level(&self, trust_level: *mut i32) -> winrt::ErrorCode;
}

#[com_interface("FAAB5CD0-8924-45AC-AD0F-A08FAE5D0324")]
pub trait IFrameworkViewImpl: IInspectable {
    unsafe fn initialize(
        &self,
        application_view: <CoreApplicationView as ::winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode;
    unsafe fn set_window(
        &self,
        window: <CoreWindow as ::winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode;
    unsafe fn load(
        &self,
        entry_point: <winrt::HString as ::winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode;
    unsafe fn run(&self) -> winrt::ErrorCode;
    unsafe fn uninitialize(&self) -> winrt::ErrorCode;
}

#[com_interface("CD770614-65C4-426C-9494-34FC43554862")]
pub trait IFrameworkViewSourceImpl: IInspectable {
    unsafe fn create_view(
        &self,
        result: *mut <IFrameworkView as ::winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode;
}

#[co_class(implements(IInspectable, IFrameworkViewImpl, IFrameworkViewSourceImpl))]
pub struct AppView {
    inner: RefCell<Box<dyn UwpApp>>,
}

impl AppView {
    pub(crate) fn new() -> Box<AppView> {
        panic!("Not supported!");
    }

    pub fn create_view_source(inner: Box<dyn UwpApp>) -> winrt::Result<IFrameworkViewSource> {
        let app_view = Box::into_raw(AppView::allocate(RefCell::new(inner)));
        let mut object = winrt::Object::default();
        unsafe {
            *object.set_abi() = std::mem::transmute(app_view);
        }
        let view_source: IFrameworkViewSource = object.try_into()?;
        std::mem::forget(object);
        Ok(view_source)
    }
}

impl IInspectable for AppView {
    unsafe fn get_iids(
        &self,
        _iid_count: *mut u32,
        _iids: *mut *mut winrt::Guid,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
    unsafe fn get_runtime_class_name(
        &self,
        _class_name: *mut <winrt::HString as winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
    unsafe fn get_trust_level(&self, _trust_level: *mut i32) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
}

impl IFrameworkViewImpl for AppView {
    unsafe fn initialize(
        &self,
        application_view: <CoreApplicationView as ::winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode {
        let application_view = CoreApplicationView::from_abi(&application_view);
        let result = self
            .inner
            .borrow_mut()
            .initialize(&application_view)
            .map_or_else(|e| e.code(), |()| winrt::ErrorCode(0));
        std::mem::forget(application_view);
        result
    }
    unsafe fn set_window(
        &self,
        window: <CoreWindow as ::winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode {
        let window = CoreWindow::from_abi(&window);
        let result = self
            .inner
            .borrow_mut()
            .set_window(&window)
            .map_or_else(|e| e.code(), |()| winrt::ErrorCode(0));
        std::mem::forget(window);
        result
    }
    unsafe fn load(
        &self,
        entry_point: <winrt::HString as ::winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode {
        let entry_point = winrt::HString::from_abi(&entry_point);
        let result = self
            .inner
            .borrow_mut()
            .load(&entry_point)
            .map_or_else(|e| e.code(), |()| winrt::ErrorCode(0));
        std::mem::forget(entry_point);
        result
    }
    unsafe fn run(&self) -> winrt::ErrorCode {
        self.inner
            .borrow_mut()
            .run()
            .map_or_else(|e| e.code(), |()| winrt::ErrorCode(0))
    }
    unsafe fn uninitialize(&self) -> winrt::ErrorCode {
        self.inner
            .borrow_mut()
            .uninitialize()
            .map_or_else(|e| e.code(), |()| winrt::ErrorCode(0))
    }
}

impl IFrameworkViewSourceImpl for AppView {
    unsafe fn create_view(
        &self,
        result: *mut <IFrameworkView as ::winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode {
        let guid = &<IFrameworkView as winrt::ComInterface>::iid();
        winrt::ErrorCode(self.query_interface(std::mem::transmute(guid), result as _) as u32)
    }
}
