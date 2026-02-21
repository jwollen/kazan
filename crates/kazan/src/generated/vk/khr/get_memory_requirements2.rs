#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
    get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
    get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
}
impl DeviceFn {
    pub unsafe fn get_image_memory_requirements2(
        &self,
        device: Device,
        info: &ImageMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_image_memory_requirements2)(device, info, memory_requirements) }
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        device: Device,
        info: &BufferMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_buffer_memory_requirements2)(device, info, memory_requirements) }
    }
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        device: Device,
        info: &ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2>,
    ) {
        unsafe {
            extend_uninit(
                sparse_memory_requirements,
                |sparse_memory_requirement_count, sparse_memory_requirements| {
                    (self.get_image_sparse_memory_requirements2)(
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
