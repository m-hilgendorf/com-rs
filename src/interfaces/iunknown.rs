use crate::com_interface;
use crate::types::{c_void, HRESULT, REFIID};

#[com_interface(00000000-0000-0000-C000-000000000046)]
pub trait IUnknown {
    /// The COM [`QueryInterface` Method]
    ///
    /// This method normally should not be called directly. Interfaces that implement
    /// `IUnknown` also implement [`IUknown::get_interface`] which is a safe wrapper around
    /// `IUknown::query_interface`.
    ///
    /// [`QueryInterface` Method]: https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void)
    /// [`IUnknown::get_interface`]: trait.IUnknown.html#method.get_interface
    unsafe fn query_interface(&self, riid: REFIID, ppv: *mut *mut c_void) -> HRESULT;

    /// The COM [`AddRef` Method]
    ///
    /// This method normally should not be called directly. This method is used by
    /// [`InterfaceRc`] to implement the reference counting mechanism.
    ///
    /// [`AddRef` Method]: https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref
    /// [`InterfaceRc`]: struct.InterfaceRc.html
    fn add_ref(&self) -> u32;

    /// The COM [`Release` Method]
    ///
    /// This method normally should not be called directly. This method is used by
    /// [`InterfaceRc`] to implement the reference counting mechanism.
    ///
    /// [`Release` Method]: https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release
    /// [`InterfaceRc`]: struct.InterfaceRc.html
    unsafe fn release(&self) -> u32;
}
