#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    map_memory2: PFN_vkMapMemory2,
    unmap_memory2: PFN_vkUnmapMemory2,
}
impl DeviceFn {
    pub unsafe fn map_memory2_khr(
        &self,
        device: Device,
        memory_map_info: &MemoryMapInfo,
        data: &mut *mut c_void,
    ) -> Result {
        unsafe { (self.map_memory2)(device, memory_map_info, data) }
    }
    pub unsafe fn unmap_memory2_khr(
        &self,
        device: Device,
        memory_unmap_info: &MemoryUnmapInfo,
    ) -> Result {
        unsafe { (self.unmap_memory2)(device, memory_unmap_info) }
    }
}
