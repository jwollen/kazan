#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryWin32HandleInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportMemoryWin32HandleInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            p_next: core::ptr::null(),
            handle_type: Default::default(),
            handle: Default::default(),
            name: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMemoryWin32HandleInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportMemoryWin32HandleInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
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
pub struct MemoryWin32HandlePropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryWin32HandlePropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            memory_type_bits: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryGetWin32HandleInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryGetWin32HandleInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_GET_WIN32_HANDLE_INFO_KHR,
            p_next: core::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR<'_>,
    p_handle: *mut HANDLE,
) -> Result;
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR<'_>,
) -> Result;
