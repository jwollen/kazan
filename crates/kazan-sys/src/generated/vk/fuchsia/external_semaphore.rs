#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportSemaphoreZirconHandleInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub zircon_handle: zx_handle_t,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportSemaphoreZirconHandleInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            zircon_handle: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreGetZirconHandleInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SemaphoreGetZirconHandleInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA<'_>,
    p_zircon_handle: *mut zx_handle_t,
) -> Result;
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
) -> Result;
