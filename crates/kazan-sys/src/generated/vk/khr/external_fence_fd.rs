#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportFenceFdInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub fd: c_int,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportFenceFdInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_FENCE_FD_INFO_KHR,
            p_next: core::ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ImportFenceFdInfoKHR<'a> {
    pub fn fence(mut self, fence: Fence) -> Self {
        self.fence = fence;
        self
    }
    pub fn flags(mut self, flags: FenceImportFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
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
pub struct FenceGetFdInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for FenceGetFdInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::FENCE_GET_FD_INFO_KHR,
            p_next: core::ptr::null(),
            fence: Default::default(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> FenceGetFdInfoKHR<'a> {
    pub fn fence(mut self, fence: Fence) -> Self {
        self.fence = fence;
        self
    }
    pub fn handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const FenceGetFdInfoKHR<'_>,
    p_fd: *mut c_int,
) -> Result;
pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(
    device: Device,
    p_import_fence_fd_info: *const ImportFenceFdInfoKHR<'_>,
) -> Result;
