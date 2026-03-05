mod generated;
pub use generated::*;

mod chain;
pub use chain::*;

pub(crate) mod macros;

mod loading;

use core::{
    ffi::{CStr, c_char}, fmt, mem::MaybeUninit, ptr
};

pub trait ExtendUninit<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>];

    unsafe fn set_len(&mut self, len: usize);
}

impl<T> ExtendUninit<T> for Vec<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>] {
        self.reserve(capacity.saturating_sub(self.capacity()));
        self.spare_capacity_mut()
    }

    unsafe fn set_len(&mut self, len: usize) {
        unsafe {
            self.set_len(self.len() + len);
        }
    }
}

impl<T> ExtendUninit<T> for &mut Vec<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>] {
        unsafe { ExtendUninit::reserve(*self, capacity) }
    }

    unsafe fn set_len(&mut self, len: usize) {
        unsafe { ExtendUninit::set_len(*self, len) }
    }
}

pub trait RawPtr<T> {
    fn to_raw_ptr(self) -> *const T;
}

impl<T> RawPtr<T> for Option<&T> {
    fn to_raw_ptr(self) -> *const T {
        match self {
            Some(inner) => inner,
            None => ptr::null(),
        }
    }
}

impl<T> RawPtr<T> for Option<&[T]> {
    fn to_raw_ptr(self) -> *const T {
        match self {
            Some(inner) => inner.as_ptr(),
            None => ptr::null(),
        }
    }
}

impl RawPtr<c_char> for Option<&CStr> {
    fn to_raw_ptr(self) -> *const c_char {
        match self {
            Some(inner) => inner.as_ptr(),
            None => ptr::null(),
        }
    }
}

pub trait RawMutPtr<T> {
    unsafe fn to_raw_mut_ptr(self) -> *mut T;
}

impl<T> RawMutPtr<T> for Option<&mut T> {
    unsafe fn to_raw_mut_ptr(self) -> *mut T {
        match self {
            Some(inner) => inner,
            None => ptr::null_mut(),
        }
    }
}

impl<T> RawMutPtr<T> for Option<&mut [T]> {
    unsafe fn to_raw_mut_ptr(self) -> *mut T {
        match self {
            Some(inner) => inner.as_mut_ptr(),
            None => ptr::null_mut(),
        }
    }
}

pub trait Handle: Sized {
    const TYPE: vk::ObjectType;
    fn as_raw(self) -> u64;
    fn from_raw(_: u64) -> Self;

    /// Returns whether the handle is a `NULL` value.
    ///
    /// # Example
    ///
    /// ```
    /// # use ash::vk::{Handle, Instance};
    /// let instance = Instance::null();
    /// assert!(instance.is_null());
    /// ```
    fn is_null(self) -> bool {
        self.as_raw() == 0
    }
}

pub use loading::MissingEntryPointError;

pub type Result<T> = core::result::Result<T, vk::Result>;

#[cfg(feature = "std")]
impl std::error::Error for vk::Result {}

impl fmt::Display for vk::Result {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match * self { Self :: SUCCESS => Some ("Command completed successfully") , Self :: NOT_READY => Some ("A fence or query has not yet completed") , Self :: TIMEOUT => Some ("A wait operation has not completed in the specified time") , Self :: EVENT_SET => Some ("An event is signaled") , Self :: EVENT_RESET => Some ("An event is unsignaled") , Self :: INCOMPLETE => Some ("A return array was too small for the result") , Self :: ERROR_OUT_OF_HOST_MEMORY => Some ("A host memory allocation has failed") , Self :: ERROR_OUT_OF_DEVICE_MEMORY => Some ("A device memory allocation has failed") , Self :: ERROR_INITIALIZATION_FAILED => Some ("Initialization of an object has failed") , Self :: ERROR_DEVICE_LOST => Some ("The logical device has been lost. See <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-lost-device>") , Self :: ERROR_MEMORY_MAP_FAILED => Some ("Mapping of a memory object has failed") , Self :: ERROR_LAYER_NOT_PRESENT => Some ("Layer specified does not exist") , Self :: ERROR_EXTENSION_NOT_PRESENT => Some ("Extension specified does not exist") , Self :: ERROR_FEATURE_NOT_PRESENT => Some ("Requested feature is not available on this device") , Self :: ERROR_INCOMPATIBLE_DRIVER => Some ("Unable to find a Vulkan driver") , Self :: ERROR_TOO_MANY_OBJECTS => Some ("Too many objects of the type have already been created") , Self :: ERROR_FORMAT_NOT_SUPPORTED => Some ("Requested format is not supported on this device") , Self :: ERROR_FRAGMENTED_POOL => Some ("A requested pool allocation has failed due to fragmentation of the pool's memory") , Self :: ERROR_UNKNOWN => Some ("An unknown error has occurred, due to an implementation or application bug") , _ => None , } ;
        if let Some(x) = name {
            fmt.write_str(x)
        } else {
            <Self as fmt::Debug>::fmt(self, fmt)
        }
    }
}

use std::ffi::{c_int, c_uint, c_ulong, c_void};
pub type RROutput = c_ulong;
pub type VisualID = c_uint;
pub type Display = c_void;
pub type Window = c_ulong;
pub type xcb_connection_t = c_void;
pub type xcb_window_t = u32;
pub type xcb_visualid_t = u32;
pub type MirConnection = *const c_void;
pub type MirSurface = *const c_void;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HANDLE.html>
pub type HANDLE = isize;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HINSTANCE.html>
pub type HINSTANCE = HANDLE;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HWND.html>
pub type HWND = HANDLE;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/struct.HMONITOR.html>
pub type HMONITOR = HANDLE;
pub type wl_display = c_void;
pub type wl_surface = c_void;
pub type DWORD = c_ulong;
pub type LPCWSTR = *const u16;
pub type zx_handle_t = u32;
pub type _screen_buffer = c_void;
pub type _screen_context = c_void;
pub type _screen_window = c_void;
pub type SECURITY_ATTRIBUTES = c_void;
pub type IDirectFB = c_void;
pub type IDirectFBSurface = c_void;
pub type GgpStreamDescriptor = c_int;
pub type GgpFrameToken = c_int;
