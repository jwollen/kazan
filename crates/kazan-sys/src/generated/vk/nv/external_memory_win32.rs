#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryWin32HandleInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagsNV,
    pub handle: HANDLE,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportMemoryWin32HandleInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            p_next: core::ptr::null(),
            handle_type: Default::default(),
            handle: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMemoryWin32HandleInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportMemoryWin32HandleInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            p_next: core::ptr::null(),
            p_attributes: core::ptr::null(),
            dw_access: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    p_handle: *mut HANDLE,
) -> Result;
