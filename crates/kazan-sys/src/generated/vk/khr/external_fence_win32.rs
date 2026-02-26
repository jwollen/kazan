#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportFenceWin32HandleInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportFenceWin32HandleInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: core::ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            handle: Default::default(),
            name: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportFenceWin32HandleInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportFenceWin32HandleInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: core::ptr::null(),
            p_attributes: core::ptr::null(),
            dw_access: Default::default(),
            name: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FenceGetWin32HandleInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for FenceGetWin32HandleInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR,
            p_next: core::ptr::null(),
            fence: Default::default(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR<'_>,
    p_handle: *mut HANDLE,
) -> Result;
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR<'_>,
) -> Result;
