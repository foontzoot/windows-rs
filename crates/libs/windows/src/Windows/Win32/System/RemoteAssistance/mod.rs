#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DRendezvousSessionEvents, DRendezvousSessionEvents_Vtbl, 0x3fa19cf8_64c4_4f53_ae60_635b3806eca6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DRendezvousSessionEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DRendezvousSessionEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DRendezvousSessionEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DRendezvousSessionEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DRendezvousSessionEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DRendezvousSessionEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DRendezvousSessionEvents_Vtbl {
    pub const fn new<Identity: DRendezvousSessionEvents_Impl, const OFFSET: isize>() -> DRendezvousSessionEvents_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DRendezvousSessionEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IRendezvousApplication, IRendezvousApplication_Vtbl, 0x4f4d070b_a275_49fb_b10d_8ec26387b50d);
impl core::ops::Deref for IRendezvousApplication {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRendezvousApplication, windows_core::IUnknown);
impl IRendezvousApplication {
    pub unsafe fn SetRendezvousSession<P0>(&self, prendezvoussession: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetRendezvousSession)(windows_core::Interface::as_raw(self), prendezvoussession.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRendezvousApplication_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetRendezvousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRendezvousApplication_Impl: Sized + windows_core::IUnknownImpl {
    fn SetRendezvousSession(&self, prendezvoussession: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRendezvousApplication {}
impl IRendezvousApplication_Vtbl {
    pub const fn new<Identity: IRendezvousApplication_Impl, const OFFSET: isize>() -> IRendezvousApplication_Vtbl {
        unsafe extern "system" fn SetRendezvousSession<Identity: IRendezvousApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendezvoussession: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRendezvousApplication_Impl::SetRendezvousSession(this, windows_core::from_raw_borrowed(&prendezvoussession)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRendezvousSession: SetRendezvousSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRendezvousApplication as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IRendezvousSession, IRendezvousSession_Vtbl, 0x9ba4b1dd_8b0c_48b7_9e7c_2f25857c8df5);
impl core::ops::Deref for IRendezvousSession {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRendezvousSession, windows_core::IUnknown);
impl IRendezvousSession {
    pub unsafe fn State(&self) -> windows_core::Result<RENDEZVOUS_SESSION_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoteUser(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteUser)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SendContextData<P0>(&self, bstrdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SendContextData)(windows_core::Interface::as_raw(self), bstrdata.param().abi()).ok()
    }
    pub unsafe fn Terminate<P0>(&self, hr: windows_core::HRESULT, bstrappdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), hr, bstrappdata.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRendezvousSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RENDEZVOUS_SESSION_STATE) -> windows_core::HRESULT,
    pub RemoteUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SendContextData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
pub trait IRendezvousSession_Impl: Sized + windows_core::IUnknownImpl {
    fn State(&self) -> windows_core::Result<RENDEZVOUS_SESSION_STATE>;
    fn RemoteUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn SendContextData(&self, bstrdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Terminate(&self, hr: windows_core::HRESULT, bstrappdata: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRendezvousSession {}
impl IRendezvousSession_Vtbl {
    pub const fn new<Identity: IRendezvousSession_Impl, const OFFSET: isize>() -> IRendezvousSession_Vtbl {
        unsafe extern "system" fn State<Identity: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRendezvousSession_Impl::State(this) {
                Ok(ok__) => {
                    psessionstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteUser<Identity: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRendezvousSession_Impl::RemoteUser(this) {
                Ok(ok__) => {
                    bstrusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRendezvousSession_Impl::Flags(this) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendContextData<Identity: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRendezvousSession_Impl::SendContextData(this, core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn Terminate<Identity: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, bstrappdata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRendezvousSession_Impl::Terminate(this, core::mem::transmute_copy(&hr), core::mem::transmute(&bstrappdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            State: State::<Identity, OFFSET>,
            RemoteUser: RemoteUser::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            SendContextData: SendContextData::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRendezvousSession as windows_core::Interface>::IID
    }
}
pub const DISPID_EVENT_ON_CONTEXT_DATA: u32 = 7u32;
pub const DISPID_EVENT_ON_SEND_ERROR: u32 = 8u32;
pub const DISPID_EVENT_ON_STATE_CHANGED: u32 = 5u32;
pub const DISPID_EVENT_ON_TERMINATION: u32 = 6u32;
pub const RSF_INVITEE: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(2i32);
pub const RSF_INVITER: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(1i32);
pub const RSF_NONE: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(0i32);
pub const RSF_ORIGINAL_INVITER: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(4i32);
pub const RSF_REMOTE_LEGACYSESSION: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(8i32);
pub const RSF_REMOTE_WIN7SESSION: RENDEZVOUS_SESSION_FLAGS = RENDEZVOUS_SESSION_FLAGS(16i32);
pub const RSS_ACCEPTED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(3i32);
pub const RSS_CANCELLED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(5i32);
pub const RSS_CONNECTED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(4i32);
pub const RSS_DECLINED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(6i32);
pub const RSS_INVITATION: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(2i32);
pub const RSS_READY: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(1i32);
pub const RSS_TERMINATED: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(7i32);
pub const RSS_UNKNOWN: RENDEZVOUS_SESSION_STATE = RENDEZVOUS_SESSION_STATE(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RENDEZVOUS_SESSION_FLAGS(pub i32);
impl windows_core::TypeKind for RENDEZVOUS_SESSION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RENDEZVOUS_SESSION_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RENDEZVOUS_SESSION_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RENDEZVOUS_SESSION_STATE(pub i32);
impl windows_core::TypeKind for RENDEZVOUS_SESSION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RENDEZVOUS_SESSION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RENDEZVOUS_SESSION_STATE").field(&self.0).finish()
    }
}
pub const RendezvousApplication: windows_core::GUID = windows_core::GUID::from_u128(0x0b7e019a_b5de_47fa_8966_9082f82fb192);
