mod generated;
pub use generated::*;

//mod loading;

use core::{
    ffi::{CStr, c_char},
    mem::MaybeUninit,
    ptr,
};

pub trait ExtendUninit<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>];

    unsafe fn set_len(&mut self, len: usize);
}

impl<T> ExtendUninit<T> for Vec<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>] {
        self.reserve(capacity.saturating_div(self.capacity()));
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

pub(crate) fn try_extend_uninit<T, N, E, F, R>(mut e: E, mut f: F) -> Result<R>
where
    N: Copy + Default + TryInto<usize> + TryFrom<usize>,
    <N as TryInto<usize>>::Error: core::fmt::Debug,
    <N as TryFrom<usize>>::Error: core::fmt::Debug,
    E: ExtendUninit<T>,
    F: FnMut(&mut N, *mut T) -> Result<R>,
{
    let mut len = N::default();
    f(&mut len, std::ptr::null_mut())?;

    let capacity = len.try_into().expect("failed to convert `N` to usize");
    let data = unsafe { e.reserve(capacity) };
    len = data.len().try_into().unwrap();
    let result = f(&mut len, data.as_mut_ptr() as *mut T)?;

    unsafe { e.set_len(len.try_into().unwrap()) };
    Ok(result)
}

pub(crate) fn try_extend_uninit2<T1, T2, N, E1, E2, F, R>(
    mut e1: E1,
    mut e2: E2,
    mut f: F,
) -> Result<R>
where
    N: Copy + Default + TryInto<usize> + TryFrom<usize>,
    <N as TryInto<usize>>::Error: core::fmt::Debug,
    <N as TryFrom<usize>>::Error: core::fmt::Debug,
    E1: ExtendUninit<T1>,
    E2: ExtendUninit<T2>,
    F: FnMut(&mut N, *mut T1, *mut T2) -> Result<R>,
{
    let mut len = N::default();
    f(&mut len, std::ptr::null_mut(), std::ptr::null_mut())?;

    let capacity = len.try_into().expect("failed to convert `N` to usize");
    let data1 = unsafe { e1.reserve(capacity) };
    let data2 = unsafe { e2.reserve(capacity) };

    len = data1.len().try_into().unwrap();
    assert_eq!(data1.len(), data2.len());

    let result = f(
        &mut len,
        data1.as_mut_ptr() as *mut T1,
        data2.as_mut_ptr() as *mut T2,
    )?;
    unsafe {
        e1.set_len(len.try_into().unwrap());
        e2.set_len(len.try_into().unwrap())
    };
    Ok(result)
}

pub(crate) fn extend_uninit<T, N, E, F>(mut e: E, mut f: F)
where
    N: Copy + Default + TryInto<usize> + TryFrom<usize>,
    <N as TryInto<usize>>::Error: core::fmt::Debug,
    <N as TryFrom<usize>>::Error: core::fmt::Debug,
    E: ExtendUninit<T>,
    F: FnMut(&mut N, *mut T),
{
    let mut len = N::default();
    f(&mut len, std::ptr::null_mut());

    let capacity = len.try_into().expect("failed to convert `N` to usize");
    let data = unsafe { e.reserve(capacity) };
    len = data.len().try_into().unwrap();
    f(&mut len, data.as_mut_ptr() as *mut T);
    unsafe { e.set_len(len.try_into().unwrap()) };
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

pub struct LoadingError;

pub type Result<T> = core::result::Result<T, vk::Result>;

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
