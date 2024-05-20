use crate::imp::Box;
use crate::{AsImpl, IUnknown, IUnknownImpl, Interface, InterfaceRef};
use core::borrow::Borrow;
use core::ops::Deref;
use core::ptr::NonNull;

/// Identifies types that can be placed in [`ComObject`].
///
/// This trait links types that can be placed in `ComObject` with the types generated by the
/// `#[implement]` macro. The `#[implement]` macro generates implementations of this trait.
/// The generated types contain the vtable layouts and refcount-related fields for the COM
/// object implementation.
///
/// This trait is an implementation detail of the Windows crates.
/// User code should not deal directly with this trait.
///
/// This trait is sort of the reverse of [`IUnknownImpl`]. This trait allows user code to use
/// `ComObject<T>] instead of `ComObject<T_Impl>`.
pub trait ComObjectInner {
    /// The generated `<foo>_Impl` type (aka the "boxed" type or "outer" type).
    type Outer: IUnknownImpl<Impl = Self>;
}

/// Describes the COM interfaces implemented by a specific COM object.
///
/// The `#[implement]` macro generates implementations of this trait. Implementations are attached
/// to the "outer" types generated by `#[implement]`, e.g. the `MyApp_Impl` type. Each
/// implementation knows how to locate the interface-specific field within `MyApp_Impl`.
///
/// This trait is an implementation detail of the Windows crates.
/// User code should not deal directly with this trait.
pub trait ComObjectInterface<I: Interface> {
    /// Gets a borrowed interface that is implemented by `T`.
    fn as_interface_ref(&self) -> InterfaceRef<'_, I>;
}

/// A counted pointer to a type that implements COM interfaces, where the object has been
/// placed in the heap (boxed).
///
/// This type exists so that you can place an object into the heap and query for COM interfaces,
/// without losing the safe reference to the implementation object.
///
/// Because the pointer inside this type is known to be non-null, `Option<ComObject<T>>` should
/// always have the same size as a single pointer.
///
/// # Safety
///
/// The contained `ptr` field is an owned, reference-counted pointer to a _pinned_ `Pin<Box<T::Outer>>`.
/// Although this code does not currently use `Pin<T>`, it takes care not to expose any unsafe semantics
/// to safe code. However, code that calls unsafe functions on [`ComObject`] must, like all unsafe code,
/// understand and preserve invariants.
#[repr(transparent)]
pub struct ComObject<T: ComObjectInner> {
    ptr: NonNull<T::Outer>,
}

impl<T: ComObjectInner> ComObject<T> {
    /// Allocates a heap cell (box) and moves `value` into it. Returns a counted pointer to `value`.
    pub fn new(value: T) -> Self {
        unsafe {
            let box_ = T::Outer::new_box(value);
            Self { ptr: NonNull::new_unchecked(Box::into_raw(box_)) }
        }
    }

    /// Gets a reference to the shared object stored in the box.
    ///
    /// [`ComObject`] also implements [`Deref`], so you can often deref directly into the object.
    /// For those situations where using the [`Deref`] impl is inconvenient, you can use
    /// this method to explicitly get a reference to the contents.
    #[inline(always)]
    pub fn get(&self) -> &T {
        self.get_box().get_impl()
    }

    /// Gets a reference to the shared object's heap box.
    #[inline(always)]
    fn get_box(&self) -> &T::Outer {
        unsafe { self.ptr.as_ref() }
    }

    // Note that we _do not_ provide a way to get a mutable reference to the outer box.
    // It's ok to return `&mut T`, but not `&mut T::Outer`. That would allow someone to replace the
    // contents of the entire object (box and reference count), which could lead to UB.
    // This could maybe be solved by returning `Pin<&mut T::Outer>`, but that requires some
    // additional thinking.

    /// Gets a mutable reference to the object stored in the box, if the reference count
    /// is exactly 1. If there are multiple references to this object then this returns `None`.
    #[inline(always)]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_reference_count_one() {
            // SAFETY: We must only return &mut T, *NOT* &mut T::Outer.
            // Returning T::Outer would allow swapping the contents of the object, which would
            // allow (incorrectly) modifying the reference count.
            unsafe { Some(self.ptr.as_mut().get_impl_mut()) }
        } else {
            None
        }
    }

    /// If this object has only a single object reference (i.e. this [`ComObject`] is the only
    /// reference to the heap allocation), then this method will extract the inner `T`
    /// (and return it in an `Ok`) and then free the heap allocation.
    ///
    /// If there is more than one reference to this object, then this returns `Err(self)`.
    #[inline(always)]
    pub fn take(self) -> Result<T, Self> {
        if self.is_reference_count_one() {
            let outer_box: Box<T::Outer> = unsafe { core::mem::transmute(self) };
            Ok(outer_box.into_inner())
        } else {
            Err(self)
        }
    }

    /// Casts to a given interface type.
    ///
    /// This always performs a `QueryInterface`, even if `T` is known to implement `I`.
    /// If you know that `T` implements `I`, then use [`Self::as_interface`] or [`Self::to_interface`] because
    /// those functions do not require a dynamic `QueryInterface` call.
    #[inline(always)]
    pub fn cast<I: Interface>(&self) -> windows_core::Result<I>
    where
        T::Outer: ComObjectInterface<IUnknown>,
    {
        let unknown = self.as_interface::<IUnknown>();
        unknown.cast()
    }

    /// Gets a borrowed reference to an interface that is implemented by `T`.
    ///
    /// The returned reference does not have an additional reference count.
    /// You can AddRef it by calling [`Self::to_owned`].
    #[inline(always)]
    pub fn as_interface<I: Interface>(&self) -> InterfaceRef<'_, I>
    where
        T::Outer: ComObjectInterface<I>,
    {
        self.get_box().as_interface_ref()
    }

    /// Gets an owned (counted) reference to an interface that is implemented by this [`ComObject`].
    #[inline(always)]
    pub fn to_interface<I: Interface>(&self) -> I
    where
        T::Outer: ComObjectInterface<I>,
    {
        self.as_interface::<I>().to_owned()
    }

    /// Converts `self` into an interface that it implements.
    ///
    /// This does not need to adjust reference counts because `self` is consumed.
    #[inline(always)]
    pub fn into_interface<I: Interface>(self) -> I
    where
        T::Outer: ComObjectInterface<I>,
    {
        unsafe {
            let raw = self.get_box().as_interface_ref().as_raw();
            core::mem::forget(self);
            I::from_raw(raw)
        }
    }
}

impl<T: ComObjectInner + Default> Default for ComObject<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: ComObjectInner> Drop for ComObject<T> {
    fn drop(&mut self) {
        unsafe {
            T::Outer::Release(self.ptr.as_ptr());
        }
    }
}

impl<T: ComObjectInner> Clone for ComObject<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe {
            self.ptr.as_ref().AddRef();
            Self { ptr: self.ptr }
        }
    }
}

impl<T: ComObjectInner> AsRef<T> for ComObject<T>
where
    IUnknown: From<T> + AsImpl<T>,
{
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T: ComObjectInner> Deref for ComObject<T> {
    type Target = T::Outer;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.get_box()
    }
}

// There is no DerefMut implementation because we cannot statically guarantee
// that the reference count is 1, which is a requirement for getting exclusive
// access to the contents of the object. Use get_mut() for dynamically-checked
// exclusive access.

impl<T: ComObjectInner> From<T> for ComObject<T> {
    fn from(value: T) -> ComObject<T> {
        ComObject::new(value)
    }
}

// Delegate hashing, if implemented.
impl<T: ComObjectInner + core::hash::Hash> core::hash::Hash for ComObject<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

// If T is Send (or Sync) then the ComObject<T> is also Send (or Sync).
// Since the actual object storage is in the heap, the object is never moved.
unsafe impl<T: ComObjectInner + Send> Send for ComObject<T> {}
unsafe impl<T: ComObjectInner + Sync> Sync for ComObject<T> {}

impl<T: ComObjectInner + PartialEq> PartialEq for ComObject<T> {
    fn eq(&self, other: &ComObject<T>) -> bool {
        let inner_self: &T = self.get();
        let other_self: &T = other.get();
        inner_self == other_self
    }
}

impl<T: ComObjectInner + Eq> Eq for ComObject<T> {}

impl<T: ComObjectInner + PartialOrd> PartialOrd for ComObject<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let inner_self: &T = self.get();
        let other_self: &T = other.get();
        <T as PartialOrd>::partial_cmp(inner_self, other_self)
    }
}

impl<T: ComObjectInner + Ord> Ord for ComObject<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let inner_self: &T = self.get();
        let other_self: &T = other.get();
        <T as Ord>::cmp(inner_self, other_self)
    }
}

impl<T: ComObjectInner + core::fmt::Debug> core::fmt::Debug for ComObject<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <T as core::fmt::Debug>::fmt(self.get(), f)
    }
}

impl<T: ComObjectInner + core::fmt::Display> core::fmt::Display for ComObject<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <T as core::fmt::Display>::fmt(self.get(), f)
    }
}

impl<T: ComObjectInner> Borrow<T> for ComObject<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}