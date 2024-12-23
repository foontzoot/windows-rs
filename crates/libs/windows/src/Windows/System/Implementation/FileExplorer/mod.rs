windows_core::imp::define_interface!(ISysStorageProviderEventReceivedEventArgs, ISysStorageProviderEventReceivedEventArgs_Vtbl, 0xe132d1b9_7b9d_5820_9728_4262b5289142);
impl windows_core::RuntimeType for ISysStorageProviderEventReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISysStorageProviderEventReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Json: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISysStorageProviderEventReceivedEventArgsFactory, ISysStorageProviderEventReceivedEventArgsFactory_Vtbl, 0xde1a780e_e975_5f68_bcc6_fb46281c6a61);
impl windows_core::RuntimeType for ISysStorageProviderEventReceivedEventArgsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISysStorageProviderEventSource, ISysStorageProviderEventSource_Vtbl, 0x1f36c476_9546_536a_8381_2f9a2c08cedd);
impl core::ops::Deref for ISysStorageProviderEventSource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISysStorageProviderEventSource, windows_core::IUnknown, windows_core::IInspectable);
impl ISysStorageProviderEventSource {
    pub fn EventReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EventReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEventReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEventReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for ISysStorageProviderEventSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISysStorageProviderEventSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EventReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEventReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
pub trait ISysStorageProviderEventSource_Impl: Sized + windows_core::IUnknownImpl {
    fn EventReceived(&self, handler: Option<&super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEventReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISysStorageProviderEventSource {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderEventSource";
}
impl ISysStorageProviderEventSource_Vtbl {
    pub const fn new<Identity: ISysStorageProviderEventSource_Impl, const OFFSET: isize>() -> ISysStorageProviderEventSource_Vtbl {
        unsafe extern "system" fn EventReceived<Identity: ISysStorageProviderEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISysStorageProviderEventSource_Impl::EventReceived(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventReceived<Identity: ISysStorageProviderEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISysStorageProviderEventSource_Impl::RemoveEventReceived(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISysStorageProviderEventSource, OFFSET>(),
            EventReceived: EventReceived::<Identity, OFFSET>,
            RemoveEventReceived: RemoveEventReceived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISysStorageProviderEventSource as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISysStorageProviderHandlerFactory, ISysStorageProviderHandlerFactory_Vtbl, 0xee798431_8213_5e89_a623_14d8c72b8a61);
impl core::ops::Deref for ISysStorageProviderHandlerFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISysStorageProviderHandlerFactory, windows_core::IUnknown, windows_core::IInspectable);
impl ISysStorageProviderHandlerFactory {
    pub fn GetHttpRequestProvider(&self, syncrootid: &windows_core::HSTRING) -> windows_core::Result<ISysStorageProviderHttpRequestProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetHttpRequestProvider)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(syncrootid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetEventSource(&self, syncrootid: &windows_core::HSTRING, eventname: &windows_core::HSTRING) -> windows_core::Result<ISysStorageProviderEventSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEventSource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(syncrootid), core::mem::transmute_copy(eventname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ISysStorageProviderHandlerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISysStorageProviderHandlerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetHttpRequestProvider: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEventSource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISysStorageProviderHandlerFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn GetHttpRequestProvider(&self, syncrootid: &windows_core::HSTRING) -> windows_core::Result<ISysStorageProviderHttpRequestProvider>;
    fn GetEventSource(&self, syncrootid: &windows_core::HSTRING, eventname: &windows_core::HSTRING) -> windows_core::Result<ISysStorageProviderEventSource>;
}
impl windows_core::RuntimeName for ISysStorageProviderHandlerFactory {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHandlerFactory";
}
impl ISysStorageProviderHandlerFactory_Vtbl {
    pub const fn new<Identity: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>() -> ISysStorageProviderHandlerFactory_Vtbl {
        unsafe extern "system" fn GetHttpRequestProvider<Identity: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncrootid: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISysStorageProviderHandlerFactory_Impl::GetHttpRequestProvider(this, core::mem::transmute(&syncrootid)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventSource<Identity: ISysStorageProviderHandlerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncrootid: core::mem::MaybeUninit<windows_core::HSTRING>, eventname: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISysStorageProviderHandlerFactory_Impl::GetEventSource(this, core::mem::transmute(&syncrootid), core::mem::transmute(&eventname)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISysStorageProviderHandlerFactory, OFFSET>(),
            GetHttpRequestProvider: GetHttpRequestProvider::<Identity, OFFSET>,
            GetEventSource: GetEventSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISysStorageProviderHandlerFactory as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISysStorageProviderHttpRequestProvider, ISysStorageProviderHttpRequestProvider_Vtbl, 0xcb6fefb6_e76a_5c25_a33e_3e78a6e0e0ce);
impl core::ops::Deref for ISysStorageProviderHttpRequestProvider {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISysStorageProviderHttpRequestProvider, windows_core::IUnknown, windows_core::IInspectable);
impl ISysStorageProviderHttpRequestProvider {
    #[cfg(feature = "Web_Http")]
    pub fn SendRequestAsync<P0>(&self, request: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>
    where
        P0: windows_core::Param<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendRequestAsync)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ISysStorageProviderHttpRequestProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISysStorageProviderHttpRequestProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Http")]
    pub SendRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    SendRequestAsync: usize,
}
#[cfg(feature = "Web_Http")]
pub trait ISysStorageProviderHttpRequestProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn SendRequestAsync(&self, request: Option<&super::super::super::Web::Http::HttpRequestMessage>) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>;
}
#[cfg(feature = "Web_Http")]
impl windows_core::RuntimeName for ISysStorageProviderHttpRequestProvider {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.ISysStorageProviderHttpRequestProvider";
}
#[cfg(feature = "Web_Http")]
impl ISysStorageProviderHttpRequestProvider_Vtbl {
    pub const fn new<Identity: ISysStorageProviderHttpRequestProvider_Impl, const OFFSET: isize>() -> ISysStorageProviderHttpRequestProvider_Vtbl {
        unsafe extern "system" fn SendRequestAsync<Identity: ISysStorageProviderHttpRequestProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISysStorageProviderHttpRequestProvider_Impl::SendRequestAsync(this, windows_core::from_raw_borrowed(&request)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISysStorageProviderHttpRequestProvider, OFFSET>(),
            SendRequestAsync: SendRequestAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISysStorageProviderHttpRequestProvider as windows_core::Interface>::IID
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SysStorageProviderEventReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SysStorageProviderEventReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SysStorageProviderEventReceivedEventArgs {
    pub fn Json(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Json)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(json: &windows_core::HSTRING) -> windows_core::Result<SysStorageProviderEventReceivedEventArgs> {
        Self::ISysStorageProviderEventReceivedEventArgsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(json), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISysStorageProviderEventReceivedEventArgsFactory<R, F: FnOnce(&ISysStorageProviderEventReceivedEventArgsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SysStorageProviderEventReceivedEventArgs, ISysStorageProviderEventReceivedEventArgsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SysStorageProviderEventReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISysStorageProviderEventReceivedEventArgs>();
}
unsafe impl windows_core::Interface for SysStorageProviderEventReceivedEventArgs {
    type Vtable = ISysStorageProviderEventReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISysStorageProviderEventReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SysStorageProviderEventReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs";
}
unsafe impl Send for SysStorageProviderEventReceivedEventArgs {}
unsafe impl Sync for SysStorageProviderEventReceivedEventArgs {}
