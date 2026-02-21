#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_image_memory_requirements2_khr: PFN_vkGetImageMemoryRequirements2,
    get_buffer_memory_requirements2_khr: PFN_vkGetBufferMemoryRequirements2,
    get_image_sparse_memory_requirements2_khr: PFN_vkGetImageSparseMemoryRequirements2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_image_memory_requirements2_khr: transmute(
                    load(c"vkGetImageMemoryRequirements2KHR").ok_or(LoadingError)?,
                ),
                get_buffer_memory_requirements2_khr: transmute(
                    load(c"vkGetBufferMemoryRequirements2KHR").ok_or(LoadingError)?,
                ),
                get_image_sparse_memory_requirements2_khr: transmute(
                    load(c"vkGetImageSparseMemoryRequirements2KHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_image_memory_requirements2_khr(
        &self,
        device: Device,
        info: &ImageMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_image_memory_requirements2_khr)(device, info, memory_requirements) }
    }
    pub unsafe fn get_buffer_memory_requirements2_khr(
        &self,
        device: Device,
        info: &BufferMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_buffer_memory_requirements2_khr)(device, info, memory_requirements) }
    }
    pub unsafe fn get_image_sparse_memory_requirements2_khr(
        &self,
        device: Device,
        info: &ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2>,
    ) {
        unsafe {
            extend_uninit(
                sparse_memory_requirements,
                |sparse_memory_requirement_count, sparse_memory_requirements| {
                    (self.get_image_sparse_memory_requirements2_khr)(
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
