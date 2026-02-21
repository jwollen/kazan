#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_device_buffer_memory_requirements: PFN_vkGetDeviceBufferMemoryRequirements,
    get_device_image_memory_requirements: PFN_vkGetDeviceImageMemoryRequirements,
    get_device_image_sparse_memory_requirements: PFN_vkGetDeviceImageSparseMemoryRequirements,
}
impl DeviceFn {
    pub unsafe fn get_device_buffer_memory_requirements_khr(
        &self,
        device: Device,
        info: &DeviceBufferMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_device_buffer_memory_requirements)(device, info, memory_requirements) }
    }
    pub unsafe fn get_device_image_memory_requirements_khr(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_device_image_memory_requirements)(device, info, memory_requirements) }
    }
    pub unsafe fn get_device_image_sparse_memory_requirements_khr(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements,
        sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2>,
    ) {
        unsafe {
            extend_uninit(
                sparse_memory_requirements,
                |sparse_memory_requirement_count, sparse_memory_requirements| {
                    (self.get_device_image_sparse_memory_requirements)(
                        device,
                        info,
                        sparse_memory_requirement_count,
                        sparse_memory_requirements as _,
                    )
                },
            )
        }
    }
}
