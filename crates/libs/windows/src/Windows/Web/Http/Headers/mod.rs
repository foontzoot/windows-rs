#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpCacheDirectiveHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpCacheDirectiveHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpCacheDirectiveHeaderValueCollection, windows_collections::IIterable<HttpNameValueHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpNameValueHeaderValue>);
impl HttpCacheDirectiveHeaderValueCollection {
    pub fn MaxAge(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMaxAge<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxAge)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn MaxStale(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxStale)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMaxStale<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxStale)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn MinFresh(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinFresh)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMinFresh<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMinFresh)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SharedMaxAge(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharedMaxAge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSharedMaxAge<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSharedMaxAge)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpNameValueHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpNameValueHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpNameValueHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpNameValueHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpNameValueHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpNameValueHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpNameValueHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpNameValueHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpNameValueHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpCacheDirectiveHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpCacheDirectiveHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpCacheDirectiveHeaderValueCollection {
    type Vtable = <IHttpCacheDirectiveHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpCacheDirectiveHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpCacheDirectiveHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCacheDirectiveHeaderValueCollection";
}
unsafe impl Send for HttpCacheDirectiveHeaderValueCollection {}
unsafe impl Sync for HttpCacheDirectiveHeaderValueCollection {}
impl IntoIterator for HttpCacheDirectiveHeaderValueCollection {
    type Item = HttpNameValueHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpCacheDirectiveHeaderValueCollection {
    type Item = HttpNameValueHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpChallengeHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpChallengeHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpChallengeHeaderValue, super::super::super::Foundation::IStringable);
impl HttpChallengeHeaderValue {
    pub fn Parameters(&self) -> windows_core::Result<windows_collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Scheme(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scheme)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Token(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Token)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateFromScheme(scheme: &windows_core::HSTRING) -> windows_core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromScheme)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(scheme), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromSchemeWithToken(scheme: &windows_core::HSTRING, token: &windows_core::HSTRING) -> windows_core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromSchemeWithToken)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(scheme), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, challengeheadervalue: &mut Option<HttpChallengeHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpChallengeHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), challengeheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpChallengeHeaderValueFactory<R, F: FnOnce(&IHttpChallengeHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpChallengeHeaderValue, IHttpChallengeHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpChallengeHeaderValueStatics<R, F: FnOnce(&IHttpChallengeHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpChallengeHeaderValue, IHttpChallengeHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpChallengeHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpChallengeHeaderValue>();
}
unsafe impl windows_core::Interface for HttpChallengeHeaderValue {
    type Vtable = <IHttpChallengeHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpChallengeHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpChallengeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpChallengeHeaderValue";
}
unsafe impl Send for HttpChallengeHeaderValue {}
unsafe impl Sync for HttpChallengeHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpChallengeHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpChallengeHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpChallengeHeaderValueCollection, windows_collections::IIterable<HttpChallengeHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpChallengeHeaderValue>);
impl HttpChallengeHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpChallengeHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpChallengeHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpChallengeHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpChallengeHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpChallengeHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpChallengeHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpChallengeHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpChallengeHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpChallengeHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpChallengeHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpChallengeHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpChallengeHeaderValueCollection {
    type Vtable = <IHttpChallengeHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpChallengeHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpChallengeHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpChallengeHeaderValueCollection";
}
unsafe impl Send for HttpChallengeHeaderValueCollection {}
unsafe impl Sync for HttpChallengeHeaderValueCollection {}
impl IntoIterator for HttpChallengeHeaderValueCollection {
    type Item = HttpChallengeHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpChallengeHeaderValueCollection {
    type Item = HttpChallengeHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpConnectionOptionHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpConnectionOptionHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpConnectionOptionHeaderValue, super::super::super::Foundation::IStringable);
impl HttpConnectionOptionHeaderValue {
    pub fn Token(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Token)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Create(token: &windows_core::HSTRING) -> windows_core::Result<HttpConnectionOptionHeaderValue> {
        Self::IHttpConnectionOptionHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpConnectionOptionHeaderValue> {
        Self::IHttpConnectionOptionHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, connectionoptionheadervalue: &mut Option<HttpConnectionOptionHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpConnectionOptionHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), connectionoptionheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpConnectionOptionHeaderValueFactory<R, F: FnOnce(&IHttpConnectionOptionHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpConnectionOptionHeaderValueStatics<R, F: FnOnce(&IHttpConnectionOptionHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpConnectionOptionHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpConnectionOptionHeaderValue>();
}
unsafe impl windows_core::Interface for HttpConnectionOptionHeaderValue {
    type Vtable = <IHttpConnectionOptionHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpConnectionOptionHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpConnectionOptionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpConnectionOptionHeaderValue";
}
unsafe impl Send for HttpConnectionOptionHeaderValue {}
unsafe impl Sync for HttpConnectionOptionHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpConnectionOptionHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpConnectionOptionHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpConnectionOptionHeaderValueCollection, windows_collections::IIterable<HttpConnectionOptionHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpConnectionOptionHeaderValue>);
impl HttpConnectionOptionHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpConnectionOptionHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpConnectionOptionHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpConnectionOptionHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpConnectionOptionHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpConnectionOptionHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpConnectionOptionHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpConnectionOptionHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpConnectionOptionHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpConnectionOptionHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpConnectionOptionHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpConnectionOptionHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpConnectionOptionHeaderValueCollection {
    type Vtable = <IHttpConnectionOptionHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpConnectionOptionHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpConnectionOptionHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpConnectionOptionHeaderValueCollection";
}
unsafe impl Send for HttpConnectionOptionHeaderValueCollection {}
unsafe impl Sync for HttpConnectionOptionHeaderValueCollection {}
impl IntoIterator for HttpConnectionOptionHeaderValueCollection {
    type Item = HttpConnectionOptionHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpConnectionOptionHeaderValueCollection {
    type Item = HttpConnectionOptionHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpContentCodingHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpContentCodingHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpContentCodingHeaderValue, super::super::super::Foundation::IStringable);
impl HttpContentCodingHeaderValue {
    pub fn ContentCoding(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentCoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Create(contentcoding: &windows_core::HSTRING) -> windows_core::Result<HttpContentCodingHeaderValue> {
        Self::IHttpContentCodingHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contentcoding), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpContentCodingHeaderValue> {
        Self::IHttpContentCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, contentcodingheadervalue: &mut Option<HttpContentCodingHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpContentCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), contentcodingheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpContentCodingHeaderValueFactory<R, F: FnOnce(&IHttpContentCodingHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpContentCodingHeaderValue, IHttpContentCodingHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpContentCodingHeaderValueStatics<R, F: FnOnce(&IHttpContentCodingHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpContentCodingHeaderValue, IHttpContentCodingHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpContentCodingHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpContentCodingHeaderValue>();
}
unsafe impl windows_core::Interface for HttpContentCodingHeaderValue {
    type Vtable = <IHttpContentCodingHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpContentCodingHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpContentCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingHeaderValue";
}
unsafe impl Send for HttpContentCodingHeaderValue {}
unsafe impl Sync for HttpContentCodingHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpContentCodingHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpContentCodingHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpContentCodingHeaderValueCollection, windows_collections::IIterable<HttpContentCodingHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpContentCodingHeaderValue>);
impl HttpContentCodingHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpContentCodingHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpContentCodingHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpContentCodingHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpContentCodingHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpContentCodingHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpContentCodingHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpContentCodingHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpContentCodingHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpContentCodingHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpContentCodingHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpContentCodingHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpContentCodingHeaderValueCollection {
    type Vtable = <IHttpContentCodingHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpContentCodingHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpContentCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingHeaderValueCollection";
}
unsafe impl Send for HttpContentCodingHeaderValueCollection {}
unsafe impl Sync for HttpContentCodingHeaderValueCollection {}
impl IntoIterator for HttpContentCodingHeaderValueCollection {
    type Item = HttpContentCodingHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpContentCodingHeaderValueCollection {
    type Item = HttpContentCodingHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpContentCodingWithQualityHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpContentCodingWithQualityHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpContentCodingWithQualityHeaderValue, super::super::super::Foundation::IStringable);
impl HttpContentCodingWithQualityHeaderValue {
    pub fn ContentCoding(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentCoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Quality(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Quality)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromValue(contentcoding: &windows_core::HSTRING) -> windows_core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contentcoding), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromValueWithQuality(contentcoding: &windows_core::HSTRING, quality: f64) -> windows_core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromValueWithQuality)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contentcoding), quality, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, contentcodingwithqualityheadervalue: &mut Option<HttpContentCodingWithQualityHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpContentCodingWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), contentcodingwithqualityheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpContentCodingWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpContentCodingWithQualityHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpContentCodingWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpContentCodingWithQualityHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpContentCodingWithQualityHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpContentCodingWithQualityHeaderValue>();
}
unsafe impl windows_core::Interface for HttpContentCodingWithQualityHeaderValue {
    type Vtable = <IHttpContentCodingWithQualityHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpContentCodingWithQualityHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpContentCodingWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValue";
}
unsafe impl Send for HttpContentCodingWithQualityHeaderValue {}
unsafe impl Sync for HttpContentCodingWithQualityHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpContentCodingWithQualityHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpContentCodingWithQualityHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpContentCodingWithQualityHeaderValueCollection, windows_collections::IIterable<HttpContentCodingWithQualityHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>);
impl HttpContentCodingWithQualityHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpContentCodingWithQualityHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpContentCodingWithQualityHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpContentCodingWithQualityHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpContentCodingWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpContentCodingWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpContentCodingWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpContentCodingWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpContentCodingWithQualityHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpContentCodingWithQualityHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpContentCodingWithQualityHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpContentCodingWithQualityHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpContentCodingWithQualityHeaderValueCollection {
    type Vtable = <IHttpContentCodingWithQualityHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpContentCodingWithQualityHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpContentCodingWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValueCollection";
}
unsafe impl Send for HttpContentCodingWithQualityHeaderValueCollection {}
unsafe impl Sync for HttpContentCodingWithQualityHeaderValueCollection {}
impl IntoIterator for HttpContentCodingWithQualityHeaderValueCollection {
    type Item = HttpContentCodingWithQualityHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpContentCodingWithQualityHeaderValueCollection {
    type Item = HttpContentCodingWithQualityHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpContentDispositionHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpContentDispositionHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpContentDispositionHeaderValue, super::super::super::Foundation::IStringable);
impl HttpContentDispositionHeaderValue {
    pub fn DispositionType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispositionType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDispositionType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDispositionType)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetFileName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFileName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FileNameStar(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileNameStar)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetFileNameStar(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFileNameStar)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Parameters(&self) -> windows_core::Result<windows_collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSize<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u64>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create(dispositiontype: &windows_core::HSTRING) -> windows_core::Result<HttpContentDispositionHeaderValue> {
        Self::IHttpContentDispositionHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(dispositiontype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpContentDispositionHeaderValue> {
        Self::IHttpContentDispositionHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, contentdispositionheadervalue: &mut Option<HttpContentDispositionHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpContentDispositionHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), contentdispositionheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpContentDispositionHeaderValueFactory<R, F: FnOnce(&IHttpContentDispositionHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpContentDispositionHeaderValueStatics<R, F: FnOnce(&IHttpContentDispositionHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpContentDispositionHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpContentDispositionHeaderValue>();
}
unsafe impl windows_core::Interface for HttpContentDispositionHeaderValue {
    type Vtable = <IHttpContentDispositionHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpContentDispositionHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpContentDispositionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentDispositionHeaderValue";
}
unsafe impl Send for HttpContentDispositionHeaderValue {}
unsafe impl Sync for HttpContentDispositionHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpContentHeaderCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpContentHeaderCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy ! ( HttpContentHeaderCollection , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::HSTRING , windows_core::HSTRING > > , windows_collections:: IMap < windows_core::HSTRING , windows_core::HSTRING > , super::super::super::Foundation:: IStringable );
impl HttpContentHeaderCollection {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpContentHeaderCollection, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ContentDisposition(&self) -> windows_core::Result<HttpContentDispositionHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentDisposition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentDisposition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpContentDispositionHeaderValue>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentDisposition)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ContentEncoding(&self) -> windows_core::Result<HttpContentCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentEncoding)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Globalization")]
    pub fn ContentLanguage(&self) -> windows_core::Result<HttpLanguageHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContentLength(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentLength)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentLength<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u64>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentLength)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ContentLocation(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentLocation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentLocation<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentLocation)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentMD5(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentMD5)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetContentMD5<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentMD5)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ContentRange(&self) -> windows_core::Result<HttpContentRangeHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentRange)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentRange<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpContentRangeHeaderValue>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentRange)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ContentType(&self) -> windows_core::Result<HttpMediaTypeHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentType<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpMediaTypeHeaderValue>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentType)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Expires(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Expires)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExpires<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExpires)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LastModified(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastModified)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLastModified<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLastModified)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Append(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TryAppendWithoutValidation(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAppendWithoutValidation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert(&self, key: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for HttpContentHeaderCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpContentHeaderCollection>();
}
unsafe impl windows_core::Interface for HttpContentHeaderCollection {
    type Vtable = <IHttpContentHeaderCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpContentHeaderCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpContentHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentHeaderCollection";
}
unsafe impl Send for HttpContentHeaderCollection {}
unsafe impl Sync for HttpContentHeaderCollection {}
impl IntoIterator for HttpContentHeaderCollection {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpContentHeaderCollection {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpContentRangeHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpContentRangeHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpContentRangeHeaderValue, super::super::super::Foundation::IStringable);
impl HttpContentRangeHeaderValue {
    pub fn FirstBytePosition(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstBytePosition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastBytePosition(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastBytePosition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Length(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Unit(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unit)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetUnit(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnit)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateFromLength(length: u64) -> windows_core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromLength)(windows_core::Interface::as_raw(this), length, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromRange(from: u64, to: u64) -> windows_core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromRange)(windows_core::Interface::as_raw(this), from, to, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromRangeWithLength(from: u64, to: u64, length: u64) -> windows_core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromRangeWithLength)(windows_core::Interface::as_raw(this), from, to, length, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, contentrangeheadervalue: &mut Option<HttpContentRangeHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpContentRangeHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), contentrangeheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpContentRangeHeaderValueFactory<R, F: FnOnce(&IHttpContentRangeHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpContentRangeHeaderValue, IHttpContentRangeHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpContentRangeHeaderValueStatics<R, F: FnOnce(&IHttpContentRangeHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpContentRangeHeaderValue, IHttpContentRangeHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpContentRangeHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpContentRangeHeaderValue>();
}
unsafe impl windows_core::Interface for HttpContentRangeHeaderValue {
    type Vtable = <IHttpContentRangeHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpContentRangeHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpContentRangeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentRangeHeaderValue";
}
unsafe impl Send for HttpContentRangeHeaderValue {}
unsafe impl Sync for HttpContentRangeHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpCookiePairHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpCookiePairHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpCookiePairHeaderValue, super::super::super::Foundation::IStringable);
impl HttpCookiePairHeaderValue {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateFromName(name: &windows_core::HSTRING) -> windows_core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromNameWithValue(name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNameWithValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, cookiepairheadervalue: &mut Option<HttpCookiePairHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpCookiePairHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), cookiepairheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpCookiePairHeaderValueFactory<R, F: FnOnce(&IHttpCookiePairHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpCookiePairHeaderValue, IHttpCookiePairHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpCookiePairHeaderValueStatics<R, F: FnOnce(&IHttpCookiePairHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpCookiePairHeaderValue, IHttpCookiePairHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpCookiePairHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpCookiePairHeaderValue>();
}
unsafe impl windows_core::Interface for HttpCookiePairHeaderValue {
    type Vtable = <IHttpCookiePairHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpCookiePairHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpCookiePairHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCookiePairHeaderValue";
}
unsafe impl Send for HttpCookiePairHeaderValue {}
unsafe impl Sync for HttpCookiePairHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpCookiePairHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpCookiePairHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpCookiePairHeaderValueCollection, windows_collections::IIterable<HttpCookiePairHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpCookiePairHeaderValue>);
impl HttpCookiePairHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpCookiePairHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpCookiePairHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpCookiePairHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpCookiePairHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpCookiePairHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpCookiePairHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpCookiePairHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpCookiePairHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpCookiePairHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpCookiePairHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpCookiePairHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpCookiePairHeaderValueCollection {
    type Vtable = <IHttpCookiePairHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpCookiePairHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpCookiePairHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCookiePairHeaderValueCollection";
}
unsafe impl Send for HttpCookiePairHeaderValueCollection {}
unsafe impl Sync for HttpCookiePairHeaderValueCollection {}
impl IntoIterator for HttpCookiePairHeaderValueCollection {
    type Item = HttpCookiePairHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpCookiePairHeaderValueCollection {
    type Item = HttpCookiePairHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpCredentialsHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpCredentialsHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpCredentialsHeaderValue, super::super::super::Foundation::IStringable);
impl HttpCredentialsHeaderValue {
    pub fn Parameters(&self) -> windows_core::Result<windows_collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Scheme(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scheme)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Token(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Token)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateFromScheme(scheme: &windows_core::HSTRING) -> windows_core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromScheme)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(scheme), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromSchemeWithToken(scheme: &windows_core::HSTRING, token: &windows_core::HSTRING) -> windows_core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromSchemeWithToken)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(scheme), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, credentialsheadervalue: &mut Option<HttpCredentialsHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpCredentialsHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), credentialsheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpCredentialsHeaderValueFactory<R, F: FnOnce(&IHttpCredentialsHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpCredentialsHeaderValue, IHttpCredentialsHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpCredentialsHeaderValueStatics<R, F: FnOnce(&IHttpCredentialsHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpCredentialsHeaderValue, IHttpCredentialsHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpCredentialsHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpCredentialsHeaderValue>();
}
unsafe impl windows_core::Interface for HttpCredentialsHeaderValue {
    type Vtable = <IHttpCredentialsHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpCredentialsHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpCredentialsHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCredentialsHeaderValue";
}
unsafe impl Send for HttpCredentialsHeaderValue {}
unsafe impl Sync for HttpCredentialsHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpDateOrDeltaHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpDateOrDeltaHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpDateOrDeltaHeaderValue, super::super::super::Foundation::IStringable);
impl HttpDateOrDeltaHeaderValue {
    pub fn Date(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Date)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Delta(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delta)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpDateOrDeltaHeaderValue> {
        Self::IHttpDateOrDeltaHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, dateordeltaheadervalue: &mut Option<HttpDateOrDeltaHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpDateOrDeltaHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), dateordeltaheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpDateOrDeltaHeaderValueStatics<R, F: FnOnce(&IHttpDateOrDeltaHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpDateOrDeltaHeaderValue, IHttpDateOrDeltaHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpDateOrDeltaHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpDateOrDeltaHeaderValue>();
}
unsafe impl windows_core::Interface for HttpDateOrDeltaHeaderValue {
    type Vtable = <IHttpDateOrDeltaHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpDateOrDeltaHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpDateOrDeltaHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpDateOrDeltaHeaderValue";
}
unsafe impl Send for HttpDateOrDeltaHeaderValue {}
unsafe impl Sync for HttpDateOrDeltaHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpExpectationHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpExpectationHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpExpectationHeaderValue, super::super::super::Foundation::IStringable);
impl HttpExpectationHeaderValue {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Parameters(&self) -> windows_core::Result<windows_collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromName(name: &windows_core::HSTRING) -> windows_core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromNameWithValue(name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNameWithValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, expectationheadervalue: &mut Option<HttpExpectationHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpExpectationHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), expectationheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpExpectationHeaderValueFactory<R, F: FnOnce(&IHttpExpectationHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpExpectationHeaderValue, IHttpExpectationHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpExpectationHeaderValueStatics<R, F: FnOnce(&IHttpExpectationHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpExpectationHeaderValue, IHttpExpectationHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpExpectationHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpExpectationHeaderValue>();
}
unsafe impl windows_core::Interface for HttpExpectationHeaderValue {
    type Vtable = <IHttpExpectationHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpExpectationHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpExpectationHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpExpectationHeaderValue";
}
unsafe impl Send for HttpExpectationHeaderValue {}
unsafe impl Sync for HttpExpectationHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpExpectationHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpExpectationHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpExpectationHeaderValueCollection, windows_collections::IIterable<HttpExpectationHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpExpectationHeaderValue>);
impl HttpExpectationHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpExpectationHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpExpectationHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpExpectationHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpExpectationHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpExpectationHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpExpectationHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpExpectationHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpExpectationHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpExpectationHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpExpectationHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpExpectationHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpExpectationHeaderValueCollection {
    type Vtable = <IHttpExpectationHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpExpectationHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpExpectationHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpExpectationHeaderValueCollection";
}
unsafe impl Send for HttpExpectationHeaderValueCollection {}
unsafe impl Sync for HttpExpectationHeaderValueCollection {}
impl IntoIterator for HttpExpectationHeaderValueCollection {
    type Item = HttpExpectationHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpExpectationHeaderValueCollection {
    type Item = HttpExpectationHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Globalization")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpLanguageHeaderValueCollection(windows_core::IUnknown);
#[cfg(feature = "Globalization")]
windows_core::imp::interface_hierarchy!(HttpLanguageHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Globalization")]
windows_core::imp::required_hierarchy!(HttpLanguageHeaderValueCollection, windows_collections::IIterable<super::super::super::Globalization::Language>, super::super::super::Foundation::IStringable, windows_collections::IVector<super::super::super::Globalization::Language>);
#[cfg(feature = "Globalization")]
impl HttpLanguageHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<super::super::super::Globalization::Language>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<super::super::super::Globalization::Language> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::super::Globalization::Language>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::super::Globalization::Language>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::super::Globalization::Language>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::super::Globalization::Language>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Globalization::Language>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<super::super::super::Globalization::Language>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<super::super::super::Globalization::Language>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Globalization")]
impl windows_core::RuntimeType for HttpLanguageHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpLanguageHeaderValueCollection>();
}
#[cfg(feature = "Globalization")]
unsafe impl windows_core::Interface for HttpLanguageHeaderValueCollection {
    type Vtable = <IHttpLanguageHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpLanguageHeaderValueCollection as windows_core::Interface>::IID;
}
#[cfg(feature = "Globalization")]
impl windows_core::RuntimeName for HttpLanguageHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageHeaderValueCollection";
}
#[cfg(feature = "Globalization")]
unsafe impl Send for HttpLanguageHeaderValueCollection {}
#[cfg(feature = "Globalization")]
unsafe impl Sync for HttpLanguageHeaderValueCollection {}
#[cfg(feature = "Globalization")]
impl IntoIterator for HttpLanguageHeaderValueCollection {
    type Item = super::super::super::Globalization::Language;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Globalization")]
impl IntoIterator for &HttpLanguageHeaderValueCollection {
    type Item = super::super::super::Globalization::Language;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpLanguageRangeWithQualityHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpLanguageRangeWithQualityHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpLanguageRangeWithQualityHeaderValue, super::super::super::Foundation::IStringable);
impl HttpLanguageRangeWithQualityHeaderValue {
    pub fn LanguageRange(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LanguageRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Quality(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Quality)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromLanguageRange(languagerange: &windows_core::HSTRING) -> windows_core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromLanguageRange)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(languagerange), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromLanguageRangeWithQuality(languagerange: &windows_core::HSTRING, quality: f64) -> windows_core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromLanguageRangeWithQuality)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(languagerange), quality, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, languagerangewithqualityheadervalue: &mut Option<HttpLanguageRangeWithQualityHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpLanguageRangeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), languagerangewithqualityheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpLanguageRangeWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpLanguageRangeWithQualityHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpLanguageRangeWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpLanguageRangeWithQualityHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpLanguageRangeWithQualityHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpLanguageRangeWithQualityHeaderValue>();
}
unsafe impl windows_core::Interface for HttpLanguageRangeWithQualityHeaderValue {
    type Vtable = <IHttpLanguageRangeWithQualityHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpLanguageRangeWithQualityHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpLanguageRangeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValue";
}
unsafe impl Send for HttpLanguageRangeWithQualityHeaderValue {}
unsafe impl Sync for HttpLanguageRangeWithQualityHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpLanguageRangeWithQualityHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpLanguageRangeWithQualityHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpLanguageRangeWithQualityHeaderValueCollection, windows_collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>);
impl HttpLanguageRangeWithQualityHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpLanguageRangeWithQualityHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpLanguageRangeWithQualityHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpLanguageRangeWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpLanguageRangeWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpLanguageRangeWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpLanguageRangeWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpLanguageRangeWithQualityHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpLanguageRangeWithQualityHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpLanguageRangeWithQualityHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpLanguageRangeWithQualityHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpLanguageRangeWithQualityHeaderValueCollection {
    type Vtable = <IHttpLanguageRangeWithQualityHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpLanguageRangeWithQualityHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpLanguageRangeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValueCollection";
}
unsafe impl Send for HttpLanguageRangeWithQualityHeaderValueCollection {}
unsafe impl Sync for HttpLanguageRangeWithQualityHeaderValueCollection {}
impl IntoIterator for HttpLanguageRangeWithQualityHeaderValueCollection {
    type Item = HttpLanguageRangeWithQualityHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpLanguageRangeWithQualityHeaderValueCollection {
    type Item = HttpLanguageRangeWithQualityHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpMediaTypeHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpMediaTypeHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpMediaTypeHeaderValue, super::super::super::Foundation::IStringable);
impl HttpMediaTypeHeaderValue {
    pub fn CharSet(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharSet)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCharSet(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharSet)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn MediaType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetMediaType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMediaType)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Parameters(&self) -> windows_core::Result<windows_collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(mediatype: &windows_core::HSTRING) -> windows_core::Result<HttpMediaTypeHeaderValue> {
        Self::IHttpMediaTypeHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(mediatype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpMediaTypeHeaderValue> {
        Self::IHttpMediaTypeHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, mediatypeheadervalue: &mut Option<HttpMediaTypeHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpMediaTypeHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), mediatypeheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpMediaTypeHeaderValueFactory<R, F: FnOnce(&IHttpMediaTypeHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpMediaTypeHeaderValueStatics<R, F: FnOnce(&IHttpMediaTypeHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpMediaTypeHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpMediaTypeHeaderValue>();
}
unsafe impl windows_core::Interface for HttpMediaTypeHeaderValue {
    type Vtable = <IHttpMediaTypeHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpMediaTypeHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpMediaTypeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeHeaderValue";
}
unsafe impl Send for HttpMediaTypeHeaderValue {}
unsafe impl Sync for HttpMediaTypeHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpMediaTypeWithQualityHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpMediaTypeWithQualityHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpMediaTypeWithQualityHeaderValue, super::super::super::Foundation::IStringable);
impl HttpMediaTypeWithQualityHeaderValue {
    pub fn CharSet(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharSet)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCharSet(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharSet)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn MediaType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetMediaType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMediaType)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Parameters(&self) -> windows_core::Result<windows_collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Quality(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Quality)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetQuality<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<f64>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuality)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CreateFromMediaType(mediatype: &windows_core::HSTRING) -> windows_core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromMediaType)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(mediatype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromMediaTypeWithQuality(mediatype: &windows_core::HSTRING, quality: f64) -> windows_core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromMediaTypeWithQuality)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(mediatype), quality, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, mediatypewithqualityheadervalue: &mut Option<HttpMediaTypeWithQualityHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpMediaTypeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), mediatypewithqualityheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpMediaTypeWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpMediaTypeWithQualityHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpMediaTypeWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpMediaTypeWithQualityHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpMediaTypeWithQualityHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpMediaTypeWithQualityHeaderValue>();
}
unsafe impl windows_core::Interface for HttpMediaTypeWithQualityHeaderValue {
    type Vtable = <IHttpMediaTypeWithQualityHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpMediaTypeWithQualityHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpMediaTypeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValue";
}
unsafe impl Send for HttpMediaTypeWithQualityHeaderValue {}
unsafe impl Sync for HttpMediaTypeWithQualityHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpMediaTypeWithQualityHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpMediaTypeWithQualityHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpMediaTypeWithQualityHeaderValueCollection, windows_collections::IIterable<HttpMediaTypeWithQualityHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>);
impl HttpMediaTypeWithQualityHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpMediaTypeWithQualityHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpMediaTypeWithQualityHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpMediaTypeWithQualityHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpMediaTypeWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpMediaTypeWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpMediaTypeWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpMediaTypeWithQualityHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpMediaTypeWithQualityHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpMediaTypeWithQualityHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpMediaTypeWithQualityHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpMediaTypeWithQualityHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpMediaTypeWithQualityHeaderValueCollection {
    type Vtable = <IHttpMediaTypeWithQualityHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpMediaTypeWithQualityHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpMediaTypeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValueCollection";
}
unsafe impl Send for HttpMediaTypeWithQualityHeaderValueCollection {}
unsafe impl Sync for HttpMediaTypeWithQualityHeaderValueCollection {}
impl IntoIterator for HttpMediaTypeWithQualityHeaderValueCollection {
    type Item = HttpMediaTypeWithQualityHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpMediaTypeWithQualityHeaderValueCollection {
    type Item = HttpMediaTypeWithQualityHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpMethodHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpMethodHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpMethodHeaderValueCollection, windows_collections::IIterable<super::HttpMethod>, super::super::super::Foundation::IStringable, windows_collections::IVector<super::HttpMethod>);
impl HttpMethodHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<super::HttpMethod>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<super::HttpMethod> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<super::HttpMethod>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::HttpMethod>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::HttpMethod>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::HttpMethod>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::HttpMethod>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<super::HttpMethod>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<super::HttpMethod>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpMethodHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpMethodHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpMethodHeaderValueCollection {
    type Vtable = <IHttpMethodHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpMethodHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpMethodHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMethodHeaderValueCollection";
}
unsafe impl Send for HttpMethodHeaderValueCollection {}
unsafe impl Sync for HttpMethodHeaderValueCollection {}
impl IntoIterator for HttpMethodHeaderValueCollection {
    type Item = super::HttpMethod;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpMethodHeaderValueCollection {
    type Item = super::HttpMethod;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpNameValueHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpNameValueHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpNameValueHeaderValue, super::super::super::Foundation::IStringable);
impl HttpNameValueHeaderValue {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateFromName(name: &windows_core::HSTRING) -> windows_core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromNameWithValue(name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNameWithValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, namevalueheadervalue: &mut Option<HttpNameValueHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpNameValueHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), namevalueheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpNameValueHeaderValueFactory<R, F: FnOnce(&IHttpNameValueHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpNameValueHeaderValue, IHttpNameValueHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpNameValueHeaderValueStatics<R, F: FnOnce(&IHttpNameValueHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpNameValueHeaderValue, IHttpNameValueHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpNameValueHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpNameValueHeaderValue>();
}
unsafe impl windows_core::Interface for HttpNameValueHeaderValue {
    type Vtable = <IHttpNameValueHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpNameValueHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpNameValueHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpNameValueHeaderValue";
}
unsafe impl Send for HttpNameValueHeaderValue {}
unsafe impl Sync for HttpNameValueHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpProductHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpProductHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpProductHeaderValue, super::super::super::Foundation::IStringable);
impl HttpProductHeaderValue {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Version(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateFromName(productname: &windows_core::HSTRING) -> windows_core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromNameWithVersion(productname: &windows_core::HSTRING, productversion: &windows_core::HSTRING) -> windows_core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNameWithVersion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productname), core::mem::transmute_copy(productversion), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, productheadervalue: &mut Option<HttpProductHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpProductHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), productheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpProductHeaderValueFactory<R, F: FnOnce(&IHttpProductHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpProductHeaderValue, IHttpProductHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpProductHeaderValueStatics<R, F: FnOnce(&IHttpProductHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpProductHeaderValue, IHttpProductHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpProductHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpProductHeaderValue>();
}
unsafe impl windows_core::Interface for HttpProductHeaderValue {
    type Vtable = <IHttpProductHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpProductHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpProductHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductHeaderValue";
}
unsafe impl Send for HttpProductHeaderValue {}
unsafe impl Sync for HttpProductHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpProductInfoHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpProductInfoHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpProductInfoHeaderValue, super::super::super::Foundation::IStringable);
impl HttpProductInfoHeaderValue {
    pub fn Product(&self) -> windows_core::Result<HttpProductHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Product)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateFromComment(productcomment: &windows_core::HSTRING) -> windows_core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productcomment), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromNameWithVersion(productname: &windows_core::HSTRING, productversion: &windows_core::HSTRING) -> windows_core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNameWithVersion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productname), core::mem::transmute_copy(productversion), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, productinfoheadervalue: &mut Option<HttpProductInfoHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpProductInfoHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), productinfoheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpProductInfoHeaderValueFactory<R, F: FnOnce(&IHttpProductInfoHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpProductInfoHeaderValue, IHttpProductInfoHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpProductInfoHeaderValueStatics<R, F: FnOnce(&IHttpProductInfoHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpProductInfoHeaderValue, IHttpProductInfoHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpProductInfoHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpProductInfoHeaderValue>();
}
unsafe impl windows_core::Interface for HttpProductInfoHeaderValue {
    type Vtable = <IHttpProductInfoHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpProductInfoHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpProductInfoHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductInfoHeaderValue";
}
unsafe impl Send for HttpProductInfoHeaderValue {}
unsafe impl Sync for HttpProductInfoHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpProductInfoHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpProductInfoHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpProductInfoHeaderValueCollection, windows_collections::IIterable<HttpProductInfoHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpProductInfoHeaderValue>);
impl HttpProductInfoHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpProductInfoHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpProductInfoHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpProductInfoHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpProductInfoHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpProductInfoHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpProductInfoHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpProductInfoHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpProductInfoHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpProductInfoHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpProductInfoHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpProductInfoHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpProductInfoHeaderValueCollection {
    type Vtable = <IHttpProductInfoHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpProductInfoHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpProductInfoHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductInfoHeaderValueCollection";
}
unsafe impl Send for HttpProductInfoHeaderValueCollection {}
unsafe impl Sync for HttpProductInfoHeaderValueCollection {}
impl IntoIterator for HttpProductInfoHeaderValueCollection {
    type Item = HttpProductInfoHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpProductInfoHeaderValueCollection {
    type Item = HttpProductInfoHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpRequestHeaderCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpRequestHeaderCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy ! ( HttpRequestHeaderCollection , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::HSTRING , windows_core::HSTRING > > , windows_collections:: IMap < windows_core::HSTRING , windows_core::HSTRING > , super::super::super::Foundation:: IStringable );
impl HttpRequestHeaderCollection {
    pub fn Accept(&self) -> windows_core::Result<HttpMediaTypeWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Accept)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AcceptEncoding(&self) -> windows_core::Result<HttpContentCodingWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceptEncoding)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AcceptLanguage(&self) -> windows_core::Result<HttpLanguageRangeWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceptLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Authorization(&self) -> windows_core::Result<HttpCredentialsHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Authorization)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAuthorization<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpCredentialsHeaderValue>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAuthorization)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CacheControl(&self) -> windows_core::Result<HttpCacheDirectiveHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Connection(&self) -> windows_core::Result<HttpConnectionOptionHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Cookie(&self) -> windows_core::Result<HttpCookiePairHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cookie)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Date(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Date)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Expect(&self) -> windows_core::Result<HttpExpectationHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Expect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn From(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).From)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetFrom(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFrom)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Networking")]
    pub fn Host(&self) -> windows_core::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Host)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking")]
    pub fn SetHost<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Networking::HostName>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHost)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IfModifiedSince(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IfModifiedSince)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIfModifiedSince<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIfModifiedSince)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IfUnmodifiedSince(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IfUnmodifiedSince)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIfUnmodifiedSince<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIfUnmodifiedSince)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn MaxForwards(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxForwards)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMaxForwards<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxForwards)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProxyAuthorization(&self) -> windows_core::Result<HttpCredentialsHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyAuthorization)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProxyAuthorization<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpCredentialsHeaderValue>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProxyAuthorization)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Referer(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Referer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetReferer<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReferer)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn TransferEncoding(&self) -> windows_core::Result<HttpTransferCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransferEncoding)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UserAgent(&self) -> windows_core::Result<HttpProductInfoHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserAgent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Append(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TryAppendWithoutValidation(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAppendWithoutValidation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert(&self, key: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for HttpRequestHeaderCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpRequestHeaderCollection>();
}
unsafe impl windows_core::Interface for HttpRequestHeaderCollection {
    type Vtable = <IHttpRequestHeaderCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpRequestHeaderCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpRequestHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpRequestHeaderCollection";
}
unsafe impl Send for HttpRequestHeaderCollection {}
unsafe impl Sync for HttpRequestHeaderCollection {}
impl IntoIterator for HttpRequestHeaderCollection {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpRequestHeaderCollection {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpResponseHeaderCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpResponseHeaderCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy ! ( HttpResponseHeaderCollection , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::HSTRING , windows_core::HSTRING > > , windows_collections:: IMap < windows_core::HSTRING , windows_core::HSTRING > , super::super::super::Foundation:: IStringable );
impl HttpResponseHeaderCollection {
    pub fn Age(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Age)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAge<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAge)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Allow(&self) -> windows_core::Result<HttpMethodHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Allow)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CacheControl(&self) -> windows_core::Result<HttpCacheDirectiveHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Connection(&self) -> windows_core::Result<HttpConnectionOptionHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Date(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Date)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Location(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Location)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLocation<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLocation)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProxyAuthenticate(&self) -> windows_core::Result<HttpChallengeHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyAuthenticate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RetryAfter(&self) -> windows_core::Result<HttpDateOrDeltaHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetryAfter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRetryAfter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpDateOrDeltaHeaderValue>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRetryAfter)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn TransferEncoding(&self) -> windows_core::Result<HttpTransferCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransferEncoding)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WwwAuthenticate(&self) -> windows_core::Result<HttpChallengeHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WwwAuthenticate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Append(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TryAppendWithoutValidation(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAppendWithoutValidation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert(&self, key: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for HttpResponseHeaderCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpResponseHeaderCollection>();
}
unsafe impl windows_core::Interface for HttpResponseHeaderCollection {
    type Vtable = <IHttpResponseHeaderCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpResponseHeaderCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpResponseHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpResponseHeaderCollection";
}
unsafe impl Send for HttpResponseHeaderCollection {}
unsafe impl Sync for HttpResponseHeaderCollection {}
impl IntoIterator for HttpResponseHeaderCollection {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpResponseHeaderCollection {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpTransferCodingHeaderValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpTransferCodingHeaderValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpTransferCodingHeaderValue, super::super::super::Foundation::IStringable);
impl HttpTransferCodingHeaderValue {
    pub fn Parameters(&self) -> windows_core::Result<windows_collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Create(input: &windows_core::HSTRING) -> windows_core::Result<HttpTransferCodingHeaderValue> {
        Self::IHttpTransferCodingHeaderValueFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<HttpTransferCodingHeaderValue> {
        Self::IHttpTransferCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, transfercodingheadervalue: &mut Option<HttpTransferCodingHeaderValue>) -> windows_core::Result<bool> {
        Self::IHttpTransferCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), transfercodingheadervalue as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IHttpTransferCodingHeaderValueFactory<R, F: FnOnce(&IHttpTransferCodingHeaderValueFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValueFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHttpTransferCodingHeaderValueStatics<R, F: FnOnce(&IHttpTransferCodingHeaderValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpTransferCodingHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpTransferCodingHeaderValue>();
}
unsafe impl windows_core::Interface for HttpTransferCodingHeaderValue {
    type Vtable = <IHttpTransferCodingHeaderValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpTransferCodingHeaderValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpTransferCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpTransferCodingHeaderValue";
}
unsafe impl Send for HttpTransferCodingHeaderValue {}
unsafe impl Sync for HttpTransferCodingHeaderValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpTransferCodingHeaderValueCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpTransferCodingHeaderValueCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpTransferCodingHeaderValueCollection, windows_collections::IIterable<HttpTransferCodingHeaderValue>, super::super::super::Foundation::IStringable, windows_collections::IVector<HttpTransferCodingHeaderValue>);
impl HttpTransferCodingHeaderValueCollection {
    pub fn ParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseAdd)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<HttpTransferCodingHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<HttpTransferCodingHeaderValue> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<HttpTransferCodingHeaderValue>> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HttpTransferCodingHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpTransferCodingHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<HttpTransferCodingHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HttpTransferCodingHeaderValue>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<HttpTransferCodingHeaderValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<HttpTransferCodingHeaderValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for HttpTransferCodingHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpTransferCodingHeaderValueCollection>();
}
unsafe impl windows_core::Interface for HttpTransferCodingHeaderValueCollection {
    type Vtable = <IHttpTransferCodingHeaderValueCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHttpTransferCodingHeaderValueCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpTransferCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpTransferCodingHeaderValueCollection";
}
unsafe impl Send for HttpTransferCodingHeaderValueCollection {}
unsafe impl Sync for HttpTransferCodingHeaderValueCollection {}
impl IntoIterator for HttpTransferCodingHeaderValueCollection {
    type Item = HttpTransferCodingHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &HttpTransferCodingHeaderValueCollection {
    type Item = HttpTransferCodingHeaderValue;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
windows_core::imp::define_interface!(IHttpCacheDirectiveHeaderValueCollection, IHttpCacheDirectiveHeaderValueCollection_Vtbl, 0x9a586b89_d5d0_4fbe_bd9d_b5b3636811b4);
impl windows_core::RuntimeType for IHttpCacheDirectiveHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCacheDirectiveHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaxAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxStale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaxStale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinFresh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMinFresh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SharedMaxAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSharedMaxAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpChallengeHeaderValue, IHttpChallengeHeaderValue_Vtbl, 0x393361af_0f7d_4820_9fdd_a2b956eeaeab);
impl windows_core::RuntimeType for IHttpChallengeHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Scheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Token: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpChallengeHeaderValueCollection, IHttpChallengeHeaderValueCollection_Vtbl, 0xca9e5f81_aee0_4353_a10b_e625babd64c2);
impl windows_core::RuntimeType for IHttpChallengeHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpChallengeHeaderValueFactory, IHttpChallengeHeaderValueFactory_Vtbl, 0xc452c451_d99c_40aa_9399_90eeb98fc613);
impl windows_core::RuntimeType for IHttpChallengeHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromScheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromSchemeWithToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpChallengeHeaderValueStatics, IHttpChallengeHeaderValueStatics_Vtbl, 0xf3d38a72_fc01_4d01_a008_fcb7c459d635);
impl windows_core::RuntimeType for IHttpChallengeHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValue_Vtbl, 0xcb4af27a_4e90_45eb_8dcd_fd1408f4c44f);
impl windows_core::RuntimeType for IHttpConnectionOptionHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Token: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpConnectionOptionHeaderValueCollection, IHttpConnectionOptionHeaderValueCollection_Vtbl, 0xe4f56c1d_5142_4e00_8e0f_019509337629);
impl windows_core::RuntimeType for IHttpConnectionOptionHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpConnectionOptionHeaderValueFactory, IHttpConnectionOptionHeaderValueFactory_Vtbl, 0xd93ccc1e_0b7d_4c3f_a58d_a2a1bdeabc0a);
impl windows_core::RuntimeType for IHttpConnectionOptionHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpConnectionOptionHeaderValueStatics, IHttpConnectionOptionHeaderValueStatics_Vtbl, 0xaaa75d37_a946_4b1f_85af_48b68b3c50bd);
impl windows_core::RuntimeType for IHttpConnectionOptionHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentCodingHeaderValue, IHttpContentCodingHeaderValue_Vtbl, 0xbcf7f92a_9376_4d85_bccc_9f4f9acab434);
impl windows_core::RuntimeType for IHttpContentCodingHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentCoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentCodingHeaderValueCollection, IHttpContentCodingHeaderValueCollection_Vtbl, 0x7d221721_a6db_436e_8e83_91596192819c);
impl windows_core::RuntimeType for IHttpContentCodingHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentCodingHeaderValueFactory, IHttpContentCodingHeaderValueFactory_Vtbl, 0xc53d2bd7_332b_4350_8510_2e67a2289a5a);
impl windows_core::RuntimeType for IHttpContentCodingHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentCodingHeaderValueStatics, IHttpContentCodingHeaderValueStatics_Vtbl, 0x94d8602e_f9bf_42f7_aa46_ed272a41e212);
impl windows_core::RuntimeType for IHttpContentCodingHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValue_Vtbl, 0x94531cd5_8b13_4d73_8651_f76b38f88495);
impl windows_core::RuntimeType for IHttpContentCodingWithQualityHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentCoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Quality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentCodingWithQualityHeaderValueCollection, IHttpContentCodingWithQualityHeaderValueCollection_Vtbl, 0x7c0d753e_e899_4378_b5c8_412d820711cc);
impl windows_core::RuntimeType for IHttpContentCodingWithQualityHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentCodingWithQualityHeaderValueFactory, IHttpContentCodingWithQualityHeaderValueFactory_Vtbl, 0xc45eee1a_c553_46fc_ade2_d75c1d53df7b);
impl windows_core::RuntimeType for IHttpContentCodingWithQualityHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromValueWithQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentCodingWithQualityHeaderValueStatics, IHttpContentCodingWithQualityHeaderValueStatics_Vtbl, 0xe8c9357c_8f89_4801_8e75_4c9abfc3de71);
impl windows_core::RuntimeType for IHttpContentCodingWithQualityHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValue_Vtbl, 0xf2a2eedc_2629_4b49_9908_96a168e9365e);
impl windows_core::RuntimeType for IHttpContentDispositionHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DispositionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDispositionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileNameStar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileNameStar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentDispositionHeaderValueFactory, IHttpContentDispositionHeaderValueFactory_Vtbl, 0x9915bbc4_456c_4e81_8295_b2ab3cbcf545);
impl windows_core::RuntimeType for IHttpContentDispositionHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentDispositionHeaderValueStatics, IHttpContentDispositionHeaderValueStatics_Vtbl, 0x29c56067_5a37_46e4_b074_c5177d69ca66);
impl windows_core::RuntimeType for IHttpContentDispositionHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentHeaderCollection, IHttpContentHeaderCollection_Vtbl, 0x40612a44_47ae_4b7e_9124_69628b64aa18);
impl windows_core::RuntimeType for IHttpContentHeaderCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentHeaderCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentDisposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentDisposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub ContentLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    ContentLanguage: usize,
    pub ContentLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ContentMD5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentMD5: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetContentMD5: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetContentMD5: usize,
    pub ContentRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Expires: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExpires: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentRangeHeaderValue, IHttpContentRangeHeaderValue_Vtbl, 0x04d967d3_a4f6_495c_9530_8579fcba8aa9);
impl windows_core::RuntimeType for IHttpContentRangeHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FirstBytePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastBytePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentRangeHeaderValueFactory, IHttpContentRangeHeaderValueFactory_Vtbl, 0x3f5bd691_a03c_4456_9a6f_ef27ecd03cae);
impl windows_core::RuntimeType for IHttpContentRangeHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromLength: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromRange: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromRangeWithLength: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpContentRangeHeaderValueStatics, IHttpContentRangeHeaderValueStatics_Vtbl, 0x80a346ca_174c_4fae_821c_134cd294aa38);
impl windows_core::RuntimeType for IHttpContentRangeHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpCookiePairHeaderValue, IHttpCookiePairHeaderValue_Vtbl, 0xcbd46217_4b29_412b_bd90_b3d814ab8e1b);
impl windows_core::RuntimeType for IHttpCookiePairHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpCookiePairHeaderValueCollection, IHttpCookiePairHeaderValueCollection_Vtbl, 0xf3f44350_581e_4ecc_9f59_e507d04f06e6);
impl windows_core::RuntimeType for IHttpCookiePairHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpCookiePairHeaderValueFactory, IHttpCookiePairHeaderValueFactory_Vtbl, 0x635e326f_146f_4f56_aa21_2cb7d6d58b1e);
impl windows_core::RuntimeType for IHttpCookiePairHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpCookiePairHeaderValueStatics, IHttpCookiePairHeaderValueStatics_Vtbl, 0x6e866d48_06af_4462_8158_99388d5dca81);
impl windows_core::RuntimeType for IHttpCookiePairHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpCredentialsHeaderValue, IHttpCredentialsHeaderValue_Vtbl, 0xc34cc3cb_542e_4177_a6c7_b674ce193fbf);
impl windows_core::RuntimeType for IHttpCredentialsHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Scheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Token: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpCredentialsHeaderValueFactory, IHttpCredentialsHeaderValueFactory_Vtbl, 0xf21d9e91_4d1c_4182_bfd1_34470a62f950);
impl windows_core::RuntimeType for IHttpCredentialsHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromScheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromSchemeWithToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpCredentialsHeaderValueStatics, IHttpCredentialsHeaderValueStatics_Vtbl, 0xa69b2be6_ce8c_4443_a35a_1b727b131036);
impl windows_core::RuntimeType for IHttpCredentialsHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpDateOrDeltaHeaderValue, IHttpDateOrDeltaHeaderValue_Vtbl, 0xeafcaa6a_c4dc_49e2_a27d_043adf5867a3);
impl windows_core::RuntimeType for IHttpDateOrDeltaHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpDateOrDeltaHeaderValueStatics, IHttpDateOrDeltaHeaderValueStatics_Vtbl, 0x7c2659a8_6672_4e90_9a9a_f39766f7f576);
impl windows_core::RuntimeType for IHttpDateOrDeltaHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpExpectationHeaderValue, IHttpExpectationHeaderValue_Vtbl, 0x4ce585cd_3a99_43af_a2e6_ec232fea9658);
impl windows_core::RuntimeType for IHttpExpectationHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpExpectationHeaderValueCollection, IHttpExpectationHeaderValueCollection_Vtbl, 0xe78521b3_a0e2_4ac4_9e66_79706cb9fd58);
impl windows_core::RuntimeType for IHttpExpectationHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpExpectationHeaderValueFactory, IHttpExpectationHeaderValueFactory_Vtbl, 0x4ea275cb_d53e_4868_8856_1e21a5030dc0);
impl windows_core::RuntimeType for IHttpExpectationHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpExpectationHeaderValueStatics, IHttpExpectationHeaderValueStatics_Vtbl, 0x3019abe2_cfe5_473b_a57f_fba5b14eb257);
impl windows_core::RuntimeType for IHttpExpectationHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpLanguageHeaderValueCollection, IHttpLanguageHeaderValueCollection_Vtbl, 0x9ebd7ca3_8219_44f6_9902_8c56dfd3340c);
impl windows_core::RuntimeType for IHttpLanguageHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValue_Vtbl, 0x7256e102_0080_4db4_a083_7de7b2e5ba4c);
impl windows_core::RuntimeType for IHttpLanguageRangeWithQualityHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LanguageRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Quality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpLanguageRangeWithQualityHeaderValueCollection, IHttpLanguageRangeWithQualityHeaderValueCollection_Vtbl, 0x885d5abd_4b4f_480a_89ce_8aedcee6e3a0);
impl windows_core::RuntimeType for IHttpLanguageRangeWithQualityHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpLanguageRangeWithQualityHeaderValueFactory, IHttpLanguageRangeWithQualityHeaderValueFactory_Vtbl, 0x7bb83970_780f_4c83_9fe4_dc3087f6bd55);
impl windows_core::RuntimeType for IHttpLanguageRangeWithQualityHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromLanguageRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromLanguageRangeWithQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpLanguageRangeWithQualityHeaderValueStatics, IHttpLanguageRangeWithQualityHeaderValueStatics_Vtbl, 0x2541e146_f308_46f5_b695_42f54024ec68);
impl windows_core::RuntimeType for IHttpLanguageRangeWithQualityHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValue_Vtbl, 0x16b28533_e728_4fcb_bdb0_08a431a14844);
impl windows_core::RuntimeType for IHttpMediaTypeHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CharSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCharSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpMediaTypeHeaderValueFactory, IHttpMediaTypeHeaderValueFactory_Vtbl, 0xbed747a8_cd17_42dd_9367_ab9c5b56dd7d);
impl windows_core::RuntimeType for IHttpMediaTypeHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpMediaTypeHeaderValueStatics, IHttpMediaTypeHeaderValueStatics_Vtbl, 0xe04d83df_1d41_4d8c_a2de_6fd2ed87399b);
impl windows_core::RuntimeType for IHttpMediaTypeHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValue_Vtbl, 0x188d5e32_76be_44a0_b1cd_2074bded2dde);
impl windows_core::RuntimeType for IHttpMediaTypeWithQualityHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CharSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCharSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Quality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpMediaTypeWithQualityHeaderValueCollection, IHttpMediaTypeWithQualityHeaderValueCollection_Vtbl, 0x3c0c6b73_1342_4587_a056_18d02ff67165);
impl windows_core::RuntimeType for IHttpMediaTypeWithQualityHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpMediaTypeWithQualityHeaderValueFactory, IHttpMediaTypeWithQualityHeaderValueFactory_Vtbl, 0x4c6d20f4_9457_44e6_a323_d122b958780b);
impl windows_core::RuntimeType for IHttpMediaTypeWithQualityHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromMediaTypeWithQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpMediaTypeWithQualityHeaderValueStatics, IHttpMediaTypeWithQualityHeaderValueStatics_Vtbl, 0x5b070cd9_b560_4fc8_9835_7e6c0a657b24);
impl windows_core::RuntimeType for IHttpMediaTypeWithQualityHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpMethodHeaderValueCollection, IHttpMethodHeaderValueCollection_Vtbl, 0x43bc3ff4_6119_4adf_938c_34bfffcf92ed);
impl windows_core::RuntimeType for IHttpMethodHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpNameValueHeaderValue, IHttpNameValueHeaderValue_Vtbl, 0xd8ba7463_5b9a_4d1b_93f9_aa5b44ecfddf);
impl windows_core::RuntimeType for IHttpNameValueHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpNameValueHeaderValueFactory, IHttpNameValueHeaderValueFactory_Vtbl, 0x770e2267_cbf8_4736_a925_93fbe10c7ca8);
impl windows_core::RuntimeType for IHttpNameValueHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpNameValueHeaderValueStatics, IHttpNameValueHeaderValueStatics_Vtbl, 0xffd4030f_1130_4152_8659_256909a9d115);
impl windows_core::RuntimeType for IHttpNameValueHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpProductHeaderValue, IHttpProductHeaderValue_Vtbl, 0xf4feee03_ebd4_4160_b9ff_807c5183b6e6);
impl windows_core::RuntimeType for IHttpProductHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpProductHeaderValueFactory, IHttpProductHeaderValueFactory_Vtbl, 0x611aa4f5_82bc_42fb_977b_dc00536e5e86);
impl windows_core::RuntimeType for IHttpProductHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromNameWithVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpProductHeaderValueStatics, IHttpProductHeaderValueStatics_Vtbl, 0x90c33e29_befc_4337_be62_49f097975f53);
impl windows_core::RuntimeType for IHttpProductHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpProductInfoHeaderValue, IHttpProductInfoHeaderValue_Vtbl, 0x1b1a8732_4c35_486a_966f_646489198e4d);
impl windows_core::RuntimeType for IHttpProductInfoHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Product: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpProductInfoHeaderValueCollection, IHttpProductInfoHeaderValueCollection_Vtbl, 0x877df74a_d69b_44f8_ad4f_453af9c42ed0);
impl windows_core::RuntimeType for IHttpProductInfoHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpProductInfoHeaderValueFactory, IHttpProductInfoHeaderValueFactory_Vtbl, 0x24220fbe_eabe_4464_b460_ec010b7c41e2);
impl windows_core::RuntimeType for IHttpProductInfoHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromComment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromNameWithVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpProductInfoHeaderValueStatics, IHttpProductInfoHeaderValueStatics_Vtbl, 0xdb7fd857_327a_4e73_81e5_7059a302b042);
impl windows_core::RuntimeType for IHttpProductInfoHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpRequestHeaderCollection, IHttpRequestHeaderCollection_Vtbl, 0xaf40329b_b544_469b_86b9_ac3d466fea36);
impl windows_core::RuntimeType for IHttpRequestHeaderCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestHeaderCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Accept: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Authorization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAuthorization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Expect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Networking")]
    pub Host: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    Host: usize,
    #[cfg(feature = "Networking")]
    pub SetHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetHost: usize,
    pub IfModifiedSince: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIfModifiedSince: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IfUnmodifiedSince: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIfUnmodifiedSince: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxForwards: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaxForwards: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProxyAuthorization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProxyAuthorization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Referer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReferer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransferEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserAgent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpResponseHeaderCollection, IHttpResponseHeaderCollection_Vtbl, 0x7a990969_fa3f_41ed_aac6_bf957975c16b);
impl windows_core::RuntimeType for IHttpResponseHeaderCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseHeaderCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Age: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Allow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProxyAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetryAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRetryAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransferEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WwwAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValue_Vtbl, 0x436f32f9_3ded_42bd_b38a_5496a2511ce6);
impl windows_core::RuntimeType for IHttpTransferCodingHeaderValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpTransferCodingHeaderValueCollection, IHttpTransferCodingHeaderValueCollection_Vtbl, 0x202c8c34_2c03_49b8_9665_73e27cb2fc79);
impl windows_core::RuntimeType for IHttpTransferCodingHeaderValueCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpTransferCodingHeaderValueFactory, IHttpTransferCodingHeaderValueFactory_Vtbl, 0xbb62dffc_e361_4f08_8e4f_c9e723de703b);
impl windows_core::RuntimeType for IHttpTransferCodingHeaderValueFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpTransferCodingHeaderValueStatics, IHttpTransferCodingHeaderValueStatics_Vtbl, 0x6ab8892a_1a98_4d32_a906_7470a9875ce5);
impl windows_core::RuntimeType for IHttpTransferCodingHeaderValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
