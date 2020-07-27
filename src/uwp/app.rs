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

#[repr(C)]
pub struct abi_IUnknown {
    pub unknown_query_interface:
        unsafe extern "system" fn(winrt::NonNullRawComPtr<winrt::IUnknown>, &winrt::Guid, *mut winrt::RawPtr) -> winrt::ErrorCode,
    pub unknown_add_ref: extern "system" fn(winrt::NonNullRawComPtr<winrt::IUnknown>) -> u32,
    pub unknown_release: extern "system" fn(winrt::NonNullRawComPtr<winrt::IUnknown>) -> u32,
}

#[repr(C)]
pub struct abi_IInspectable {
    iunknown: abi_IUnknown,

    pub inspectable_iids:
        unsafe extern "system" fn(winrt::NonNullRawComPtr<winrt::Object>, *mut u32, *mut *mut winrt::Guid) -> winrt::ErrorCode,
    pub inspectable_type_name: unsafe extern "system" fn(
        winrt::NonNullRawComPtr<winrt::Object>,
        *mut <winrt::HString as winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode,
    pub inspectable_trust_level:
        unsafe extern "system" fn(winrt::NonNullRawComPtr<winrt::Object>, *mut i32) -> winrt::ErrorCode,
}

#[derive(Default, Clone, PartialEq)]
#[repr(transparent)]
pub struct App {
    ptr: winrt::ComPtr<App>,
}

impl App {
    pub fn new() -> winrt::Result<Self> {
        unsafe {
            let raw = AppRc::new();

            let mut result: App = std::mem::zeroed();
            let mut ptr: std::ptr::NonNull<Self> = std::ptr::NonNull::new_unchecked(raw.as_ref().unwrap().get_app_abi() as _);
            *<App as AbiTransferable>::set_abi(&mut result) =
                Some(winrt::NonNullRawComPtr::new(ptr.cast()));

            Ok(result)
        }
    }
}

unsafe impl winrt::ComInterface for App {
    type VTable = abi_IApp;
    fn iid() -> winrt::Guid {
        winrt_guid!(DF36D812-3CF5-4B3D-BFA6-AC1A74F3C5C0)
    }
}

unsafe impl winrt::AbiTransferable for App {
    type Abi = winrt::RawComPtr<Self>;
    fn get_abi(&self) -> Self::Abi {
        <winrt::ComPtr<Self> as winrt::AbiTransferable>::get_abi(&self.ptr)
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        <winrt::ComPtr<Self> as winrt::AbiTransferable>::set_abi(&mut self.ptr)
    }
}

#[repr(C)]
pub struct abi_IApp {
    iinspectable: abi_IInspectable,
}

#[repr(C)]
pub struct abi_IFrameworkView_Full {
    iinspectable: abi_IInspectable,
    pub initialize: unsafe extern "system" fn(
        ::winrt::NonNullRawComPtr<IFrameworkView>,
        application_view: <CoreApplicationView as ::winrt::AbiTransferable>::Abi,
    ) -> ::winrt::ErrorCode,
    pub set_window: unsafe extern "system" fn(
        ::winrt::NonNullRawComPtr<IFrameworkView>,
        window: <CoreWindow as ::winrt::AbiTransferable>::Abi,
    ) -> ::winrt::ErrorCode,
    pub load: unsafe extern "system" fn(
        ::winrt::NonNullRawComPtr<IFrameworkView>,
        entry_point: <winrt::HString as ::winrt::AbiTransferable>::Abi,
    ) -> ::winrt::ErrorCode,
    pub run: unsafe extern "system" fn(
        ::winrt::NonNullRawComPtr<IFrameworkView>,
    ) -> ::winrt::ErrorCode,
    pub uninitialize: unsafe extern "system" fn(
        ::winrt::NonNullRawComPtr<IFrameworkView>,
    ) -> ::winrt::ErrorCode,
}

#[repr(C)]
pub struct abi_IFrameworkViewSource_Full {
    iinspectable: abi_IInspectable,
    pub create_view: unsafe extern "system" fn(
        ::winrt::NonNullRawComPtr<IFrameworkViewSource>,
        result__: *mut <IFrameworkView as ::winrt::AbiTransferable>::Abi,
    ) -> ::winrt::ErrorCode,
}

struct AppState {
    compositor: Compositor,
    target: CompositionTarget,
}

struct AppInner {
    state: Option<AppState>
}

impl AppInner {
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }

    pub fn initialize(&self, window: &CoreApplicationView) -> winrt::Result<()> {
        Ok(())
    }

    pub fn set_window(&self, window: &CoreWindow) -> winrt::Result<()> {
        Ok(())
    }

    pub fn load(&self, entry_point: &winrt::HString) -> winrt::Result<()> {
        Ok(())
    }

    pub fn run(&self) -> winrt::Result<()> {
        let compositor = Compositor::new()?;
        let root = compositor.create_container_visual()?;
        root.set_relative_size_adjustment(Vector2 { x:1.0, y:1.0 })?;
        let target = compositor.create_target_for_current_view()?;
        target.set_root(&root)?;

        // TODO: Remove
        let test = compositor.create_sprite_visual()?;
        test.set_size(Vector2 { x: 200.0, y: 200.0 })?;
        test.set_brush(compositor.create_color_brush_with_color(Colors::red()?)?)?;
        root.children()?.insert_at_top(&test)?;

        // TODO: Init minesweeper

        // TODO: Hook events

        let window = CoreWindow::get_for_current_thread()?;
        window.activate()?;

        let dispatcher = window.dispatcher()?;
        dispatcher.process_events(CoreProcessEventsOption::ProcessUntilQuit)?;

        Ok(())
    }

    pub fn uninitialize(&self) -> winrt::Result<()> {
        Ok(())
    }
}

struct AppRc {
    count: winrt::RefCount,
    this: *mut AppRc,
    app_abi: *mut impl_App, 
    view_abi: *mut impl_App_IFrameworkView,
    view_source_abi: *mut impl_App_IFrameworkViewSource,
    inner: AppInner,
}

impl AppRc {
    pub fn new() -> *mut Self {
        let count = winrt::RefCount::new();
        let inner = AppInner::new();
        unsafe {
            let base = Self {
                count,
                this: std::ptr::null_mut::<AppRc>(),
                app_abi: std::ptr::null_mut::<impl_App>(),
                view_abi: std::ptr::null_mut::<impl_App_IFrameworkView>(),
                view_source_abi: std::ptr::null_mut::<impl_App_IFrameworkViewSource>(),
                inner
            };
            let base = Box::into_raw(Box::new(base));

            let app_abi = impl_App::from_base(base);
            let view_abi = impl_App_IFrameworkView::from_base(base);
            let view_source_abi = impl_App_IFrameworkViewSource::from_base(base);

            (*base).this = base;
            (*base).app_abi = app_abi;
            (*base).view_abi = view_abi;
            (*base).view_source_abi = view_source_abi;

            base
        }
    }

    pub fn add_ref(&self) -> u32 {
        self.count.add_ref()
    }

    pub unsafe fn release(&self) -> u32 {
        let remaining = self.count.release();
        if remaining == 0 {
            Box::from_raw(self.this);
        }
        remaining
    }

    pub unsafe fn get_inner(&self) -> &AppInner {
        &self.inner
    }

    pub fn get_app_abi(&self) -> *mut impl_App {
        self.app_abi
    }

    pub fn get_view_abi(&self) -> *mut impl_App_IFrameworkView {
        self.view_abi
    }

    pub fn get_view_source_abi(&self) -> *mut impl_App_IFrameworkViewSource {
        self.view_source_abi
    }
}

impl Drop for AppRc {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.app_abi);
            Box::from_raw(self.view_abi);
            Box::from_raw(self.view_source_abi);
        }
    }
}

#[repr(C)]
struct impl_App_IFrameworkView {
    vtable: *const abi_IFrameworkView_Full,
    base: *mut AppRc,
}

#[repr(C)]
struct impl_App_IFrameworkViewSource {
    vtable: *const abi_IFrameworkViewSource_Full,
    base: *mut AppRc,
}

#[repr(C)]
struct impl_App {
    vtable: *const abi_IApp,
    base: *mut AppRc,
}

impl impl_App {
    const VTABLE: abi_IApp = abi_IApp {
        iinspectable: abi_IInspectable {
            iunknown: abi_IUnknown {
                unknown_query_interface: impl_App::unknown_query_interface,
                unknown_add_ref: impl_App::unknown_add_ref,
                unknown_release: impl_App::unknown_release,
            },
            inspectable_iids: impl_App::inspectable_iids,
            inspectable_type_name: impl_App::inspectable_type_name,
            inspectable_trust_level: impl_App::inspectable_trust_level,
        },
    };

    unsafe fn from_base(base: *mut AppRc) -> *mut Self {
        Box::into_raw(Box::new(Self {
            vtable: &Self::VTABLE,
            base
        }))
    } 

    extern "system" fn unknown_query_interface(
        this: winrt::NonNullRawComPtr<winrt::IUnknown>,
        iid: &winrt::Guid,
        interface: *mut winrt::RawPtr,
    ) -> winrt::ErrorCode {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            if iid == &<App as winrt::ComInterface>::iid()
                || iid == &<winrt::IUnknown as winrt::ComInterface>::iid()
                || iid == &<winrt::Object as winrt::ComInterface>::iid()
                || iid == &<winrt::IAgileObject as winrt::ComInterface>::iid()
            {
                *interface = this as winrt::RawPtr;
                (*this).base.as_ref().unwrap().add_ref();
                return winrt::ErrorCode(0);
            } else if iid == &<IFrameworkView as winrt::ComInterface>::iid() {
                (*this).base.as_ref().unwrap().add_ref();
                let base = (*this).base;
                *interface = (*base).get_view_abi() as winrt::RawPtr;
                return winrt::ErrorCode(0);
            } else if iid == &<IFrameworkViewSource as winrt::ComInterface>::iid() {
                (*this).base.as_ref().unwrap().add_ref();
                let base = (*this).base;
                *interface = (*base).get_view_source_abi() as winrt::RawPtr;
                return winrt::ErrorCode(0);
            }
            *interface = std::ptr::null_mut();
            winrt::ErrorCode(0x80004002)
        }
    }
    extern "system" fn unknown_add_ref(this: winrt::NonNullRawComPtr<winrt::IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            (*this).base.as_ref().unwrap().add_ref()
        }
    }
    extern "system" fn unknown_release(this: winrt::NonNullRawComPtr<winrt::IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            (*this).base.as_ref().unwrap().release()
        }
    }
    extern "system" fn inspectable_iids(
        _this: winrt::NonNullRawComPtr<winrt::Object>,
        _iidcount: *mut u32,
        _iids: *mut *mut winrt::Guid,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
    extern "system" fn inspectable_type_name(
        _this: winrt::NonNullRawComPtr<winrt::Object>,
        _class_name: *mut <winrt::HString as winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
    extern "system" fn inspectable_trust_level(
        _this: winrt::NonNullRawComPtr<winrt::Object>,
        _trust_level: *mut i32,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
}

impl impl_App_IFrameworkView {
    const VTABLE: abi_IFrameworkView_Full = abi_IFrameworkView_Full {
        iinspectable: abi_IInspectable {
            iunknown: abi_IUnknown {
                unknown_query_interface: impl_App_IFrameworkView::unknown_query_interface,
                unknown_add_ref: impl_App_IFrameworkView::unknown_add_ref,
                unknown_release: impl_App_IFrameworkView::unknown_release,
            },
            inspectable_iids: impl_App_IFrameworkView::inspectable_iids,
            inspectable_type_name: impl_App_IFrameworkView::inspectable_type_name,
            inspectable_trust_level: impl_App_IFrameworkView::inspectable_trust_level,
        },
        initialize: impl_App_IFrameworkView::initialize,
        set_window: impl_App_IFrameworkView::set_window,
        load: impl_App_IFrameworkView::load,
        run: impl_App_IFrameworkView::run,
        uninitialize: impl_App_IFrameworkView::uninitialize,
    };

    unsafe fn from_base(base: *mut AppRc) -> *mut Self {
        Box::into_raw(Box::new(Self {
            vtable: &Self::VTABLE,
            base
        }))
    } 

    extern "system" fn unknown_query_interface(
        this: winrt::NonNullRawComPtr<winrt::IUnknown>,
        iid: &winrt::Guid,
        interface: *mut winrt::RawPtr,
    ) -> winrt::ErrorCode {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            if iid == &<IFrameworkView as winrt::ComInterface>::iid()
                || iid == &<winrt::IUnknown as winrt::ComInterface>::iid()
                || iid == &<winrt::Object as winrt::ComInterface>::iid()
                || iid == &<winrt::IAgileObject as winrt::ComInterface>::iid()
            {
                *interface = this as winrt::RawPtr;
                (*this).base.as_ref().unwrap().add_ref();
                return winrt::ErrorCode(0);
            } else if iid == &<App as winrt::ComInterface>::iid() {
                (*this).base.as_ref().unwrap().add_ref();
                let base = (*this).base;
                *interface = (*base).get_app_abi() as winrt::RawPtr;
                return winrt::ErrorCode(0);
            } else if iid == &<IFrameworkViewSource as winrt::ComInterface>::iid() {
                (*this).base.as_ref().unwrap().add_ref();
                let base = (*this).base;
                *interface = (*base).get_view_source_abi() as winrt::RawPtr;
                return winrt::ErrorCode(0);
            }
            *interface = std::ptr::null_mut();
            winrt::ErrorCode(0x80004002)
        }
    }
    extern "system" fn unknown_add_ref(this: winrt::NonNullRawComPtr<winrt::IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            (*this).base.as_ref().unwrap().add_ref()
        }
    }
    extern "system" fn unknown_release(this: winrt::NonNullRawComPtr<winrt::IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            (*this).base.as_ref().unwrap().release()
        }
    }
    extern "system" fn inspectable_iids(
        _this: winrt::NonNullRawComPtr<winrt::Object>,
        _iidcount: *mut u32,
        _iids: *mut *mut winrt::Guid,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
    extern "system" fn inspectable_type_name(
        _this: winrt::NonNullRawComPtr<winrt::Object>,
        _class_name: *mut <winrt::HString as winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
    extern "system" fn inspectable_trust_level(
        _this: winrt::NonNullRawComPtr<winrt::Object>,
        _trust_level: *mut i32,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
    unsafe extern "system" fn initialize(
        this: winrt::NonNullRawComPtr<IFrameworkView>,
        application_view: <CoreApplicationView as ::winrt::AbiTransferable>::Abi,
    ) -> ::winrt::ErrorCode {
        let this: *mut Self = this.as_raw() as _;
        let application_view = CoreApplicationView::from_abi(&application_view);
        let result = (*this).base.as_ref().unwrap().get_inner()
            .initialize(&application_view)
            .map_or_else(|e| e.code(), |()| winrt::ErrorCode(0));
        std::mem::forget(application_view);
        result
    }
    unsafe extern "system" fn set_window(
        this: ::winrt::NonNullRawComPtr<IFrameworkView>,
        window: <CoreWindow as ::winrt::AbiTransferable>::Abi,
    ) -> ::winrt::ErrorCode {
        let this: *mut Self = this.as_raw() as _;
        let window = CoreWindow::from_abi(&window);
        let result = (*this).base.as_ref().unwrap().get_inner()
            .set_window(&window)
            .map_or_else(|e| e.code(), |()| winrt::ErrorCode(0));
        std::mem::forget(window);
        result
    }
    unsafe extern "system" fn load(
        this: ::winrt::NonNullRawComPtr<IFrameworkView>,
        entry_point: <winrt::HString as ::winrt::AbiTransferable>::Abi,
    ) -> ::winrt::ErrorCode {
        let this: *mut Self = this.as_raw() as _;
        let entry_point = winrt::HString::from_abi(&entry_point);
        let result = (*this).base.as_ref().unwrap().get_inner()
            .load(&entry_point)
            .map_or_else(|e| e.code(), |()| winrt::ErrorCode(0));
        std::mem::forget(entry_point);
        result
    }
    unsafe extern "system" fn run(
        this: ::winrt::NonNullRawComPtr<IFrameworkView>,
    ) -> ::winrt::ErrorCode {
        let this: *mut Self = this.as_raw() as _;
        (*this).base.as_ref().unwrap().get_inner()
            .run()
            .map_or_else(|e| e.code(), |()| winrt::ErrorCode(0))
    }
    unsafe extern "system" fn uninitialize(
        this: ::winrt::NonNullRawComPtr<IFrameworkView>,
    ) -> ::winrt::ErrorCode {
        let this: *mut Self = this.as_raw() as _;
        (*this).base.as_ref().unwrap().get_inner()
            .uninitialize()
            .map_or_else(|e| e.code(), |()| winrt::ErrorCode(0))
    }
}

impl impl_App_IFrameworkViewSource {
    const VTABLE: abi_IFrameworkViewSource_Full = abi_IFrameworkViewSource_Full {
        iinspectable: abi_IInspectable {
            iunknown: abi_IUnknown {
                unknown_query_interface: impl_App_IFrameworkViewSource::unknown_query_interface,
                unknown_add_ref: impl_App_IFrameworkViewSource::unknown_add_ref,
                unknown_release: impl_App_IFrameworkViewSource::unknown_release,
            },
            inspectable_iids: impl_App_IFrameworkViewSource::inspectable_iids,
            inspectable_type_name: impl_App_IFrameworkViewSource::inspectable_type_name,
            inspectable_trust_level: impl_App_IFrameworkViewSource::inspectable_trust_level,
        },
        create_view: impl_App_IFrameworkViewSource::create_view,
    };

    unsafe fn from_base(base: *mut AppRc) -> *mut Self {
        Box::into_raw(Box::new(Self {
            vtable: &Self::VTABLE,
            base
        }))
    } 

    extern "system" fn unknown_query_interface(
        this: winrt::NonNullRawComPtr<winrt::IUnknown>,
        iid: &winrt::Guid,
        interface: *mut winrt::RawPtr,
    ) -> winrt::ErrorCode {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            if iid == &<IFrameworkViewSource as winrt::ComInterface>::iid()
                || iid == &<winrt::IUnknown as winrt::ComInterface>::iid()
                || iid == &<winrt::Object as winrt::ComInterface>::iid()
                || iid == &<winrt::IAgileObject as winrt::ComInterface>::iid()
            {
                *interface = this as winrt::RawPtr;
                (*this).base.as_ref().unwrap().add_ref();
                return winrt::ErrorCode(0);
            } else if iid == &<IFrameworkView as winrt::ComInterface>::iid() {
                (*this).base.as_ref().unwrap().add_ref();
                let base = (*this).base;
                *interface = (*base).get_view_abi() as winrt::RawPtr;
                return winrt::ErrorCode(0);
            } else if iid == &<App as winrt::ComInterface>::iid() {
                (*this).base.as_ref().unwrap().add_ref();
                let base = (*this).base;
                *interface = (*base).get_app_abi() as winrt::RawPtr;
                return winrt::ErrorCode(0);
            }
            *interface = std::ptr::null_mut();
            winrt::ErrorCode(0x80004002)
        }
    }
    extern "system" fn unknown_add_ref(this: winrt::NonNullRawComPtr<winrt::IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            (*this).base.as_ref().unwrap().add_ref()
        }
    }
    extern "system" fn unknown_release(this: winrt::NonNullRawComPtr<winrt::IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            (*this).base.as_ref().unwrap().release()
        }
    }
    extern "system" fn inspectable_iids(
        _this: winrt::NonNullRawComPtr<winrt::Object>,
        _iidcount: *mut u32,
        _iids: *mut *mut winrt::Guid,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
    extern "system" fn inspectable_type_name(
        _this: winrt::NonNullRawComPtr<winrt::Object>,
        _class_name: *mut <winrt::HString as winrt::AbiTransferable>::Abi,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
    extern "system" fn inspectable_trust_level(
        _this: winrt::NonNullRawComPtr<winrt::Object>,
        _trust_level: *mut i32,
    ) -> winrt::ErrorCode {
        winrt::ErrorCode(0x80004001)
    }
    unsafe extern "system" fn create_view(
        this: ::winrt::NonNullRawComPtr<IFrameworkViewSource>,
        result__: *mut <IFrameworkView as ::winrt::AbiTransferable>::Abi,
    ) -> ::winrt::ErrorCode {
        let this: *mut Self = this.as_raw() as _;
        (*this).base.as_ref().unwrap().add_ref();
        let base = (*this).base;
        *result__ = std::mem::transmute((*base).get_view_abi());
        winrt::ErrorCode(0)
    }
}