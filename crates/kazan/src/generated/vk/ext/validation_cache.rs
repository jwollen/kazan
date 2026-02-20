#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    create_validation_cache_ext: PFN_vkCreateValidationCacheEXT,
    destroy_validation_cache_ext: PFN_vkDestroyValidationCacheEXT,
    get_validation_cache_data_ext: PFN_vkGetValidationCacheDataEXT,
    merge_validation_caches_ext: PFN_vkMergeValidationCachesEXT,
}
impl DeviceFn {
    pub unsafe fn create_validation_cache_ext(
        &self,
        device: Device,
        create_info: &ValidationCacheCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        validation_cache: &mut ValidationCacheEXT,
    ) -> Result {
        unsafe {
            (self.create_validation_cache_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                validation_cache,
            )
        }
    }
    pub unsafe fn destroy_validation_cache_ext(
        &self,
        device: Device,
        validation_cache: ValidationCacheEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_validation_cache_ext)(device, validation_cache, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        device: Device,
        validation_cache: ValidationCacheEXT,
        data: impl ExtendUninit<u8>,
    ) -> Result {
        unsafe {
            try_extend_uninit(data, |data_size, data| {
                (self.get_validation_cache_data_ext)(device, validation_cache, data_size, data as _)
            })
        }
    }
    pub unsafe fn merge_validation_caches_ext(
        &self,
        device: Device,
        dst_cache: ValidationCacheEXT,
        src_caches: &[ValidationCacheEXT],
    ) -> Result {
        unsafe {
            (self.merge_validation_caches_ext)(
                device,
                dst_cache,
                src_caches.len().try_into().unwrap(),
                src_caches.as_ptr() as _,
            )
        }
    }
}
