#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportFenceWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportFenceWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FenceGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR,
    p_handle: *mut HANDLE,
) -> Result;
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR,
) -> Result;
