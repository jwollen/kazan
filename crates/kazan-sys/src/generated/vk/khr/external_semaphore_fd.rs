#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportSemaphoreFdInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub fd: c_int,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportSemaphoreFdInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_SEMAPHORE_FD_INFO_KHR,
            p_next: core::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ImportSemaphoreFdInfoKHR<'a> {
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
    pub fn fd(mut self, fd: c_int) -> Self {
        self.fd = fd;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreGetFdInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SemaphoreGetFdInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SEMAPHORE_GET_FD_INFO_KHR,
            p_next: core::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SemaphoreGetFdInfoKHR<'a> {
    pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
        self.semaphore = semaphore;
        self
    }
    pub fn handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const SemaphoreGetFdInfoKHR<'_>,
    p_fd: *mut c_int,
) -> Result;
pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR<'_>,
) -> Result;
