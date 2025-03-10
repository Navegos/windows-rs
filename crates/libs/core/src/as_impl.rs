/// A trait for retrieving the implementation behind a COM or WinRT interface.
///
/// This trait is automatically implemented when using the `implement` macro.
pub trait AsImpl<T> {
    /// # Safety
    ///
    /// The caller needs to ensure that `self` is actually implemented by the
    /// implementation `T`.
    unsafe fn as_impl(&self) -> &T {
        unsafe { self.as_impl_ptr().as_ref() }
    }

    /// Returns a pointer to the implementation object.
    ///
    /// # Safety
    ///
    /// The caller needs to ensure that `self` is actually implemented by the
    /// implementation `T`.
    unsafe fn as_impl_ptr(&self) -> core::ptr::NonNull<T>;
}
