#![allow(non_camel_case_types)]
mod generated;
pub use generated::*;

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
