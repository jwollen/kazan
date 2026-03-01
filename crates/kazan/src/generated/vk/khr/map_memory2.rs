#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    pub type MemoryMapInfoKHR<'a> = MemoryMapInfo<'a>;
    pub type MemoryUnmapInfoKHR<'a> = MemoryUnmapInfo<'a>;
    pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;
    pub type PFN_vkMapMemory2KHR = PFN_vkMapMemory2;
    pub type PFN_vkUnmapMemory2KHR = PFN_vkUnmapMemory2;
}
pub struct DeviceFn {
    map_memory2_khr: PFN_vkMapMemory2,
    unmap_memory2_khr: PFN_vkUnmapMemory2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                map_memory2_khr: transmute(load(c"vkMapMemory2KHR").ok_or(LoadingError)?),
                unmap_memory2_khr: transmute(load(c"vkUnmapMemory2KHR").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn map_memory2_khr(
        &self,
        device: Device,
        memory_map_info: &MemoryMapInfo<'_>,
        data: &mut *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.map_memory2_khr)(device, memory_map_info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn unmap_memory2_khr(
        &self,
        device: Device,
        memory_unmap_info: &MemoryUnmapInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.unmap_memory2_khr)(device, memory_unmap_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
