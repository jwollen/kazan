#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportSemaphoreWin32HandleInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportSemaphoreWin32HandleInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            p_next: core::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            handle: Default::default(),
            name: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ImportSemaphoreWin32HandleInfoKHR<'a> {
    pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
        self.semaphore = semaphore;
        self
    }
    pub fn flags(mut self, flags: SemaphoreImportFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: HANDLE) -> Self {
        self.handle = handle;
        self
    }
    pub fn name(mut self, name: LPCWSTR) -> Self {
        self.name = name;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportSemaphoreWin32HandleInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportSemaphoreWin32HandleInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            p_next: core::ptr::null(),
            p_attributes: core::ptr::null(),
            dw_access: Default::default(),
            name: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ExportSemaphoreWin32HandleInfoKHR<'a> {
    pub fn attributes(mut self, attributes: *const SECURITY_ATTRIBUTES) -> Self {
        self.p_attributes = attributes;
        self
    }
    pub fn dw_access(mut self, dw_access: DWORD) -> Self {
        self.dw_access = dw_access;
        self
    }
    pub fn name(mut self, name: LPCWSTR) -> Self {
        self.name = name;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct D3D12FenceSubmitInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_values_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_values_count: u32,
    pub p_signal_semaphore_values: *const u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for D3D12FenceSubmitInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::D3D12_FENCE_SUBMIT_INFO_KHR,
            p_next: core::ptr::null(),
            wait_semaphore_values_count: Default::default(),
            p_wait_semaphore_values: core::ptr::null(),
            signal_semaphore_values_count: Default::default(),
            p_signal_semaphore_values: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> D3D12FenceSubmitInfoKHR<'a> {
    pub fn wait_semaphore_values(mut self, wait_semaphore_values: &'a [u64]) -> Self {
        self.wait_semaphore_values_count = wait_semaphore_values.len().try_into().unwrap();
        self.p_wait_semaphore_values = wait_semaphore_values.as_ptr();
        self
    }
    pub fn signal_semaphore_values(mut self, signal_semaphore_values: &'a [u64]) -> Self {
        self.signal_semaphore_values_count = signal_semaphore_values.len().try_into().unwrap();
        self.p_signal_semaphore_values = signal_semaphore_values.as_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreGetWin32HandleInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SemaphoreGetWin32HandleInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
            p_next: core::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SemaphoreGetWin32HandleInfoKHR<'a> {
    pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
        self.semaphore = semaphore;
        self
    }
    pub fn handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR<'_>,
    p_handle: *mut HANDLE,
) -> Result;
pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR<'_>,
) -> Result;
