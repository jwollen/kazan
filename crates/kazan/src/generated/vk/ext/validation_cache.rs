#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    create_validation_cache_ext: PFN_vkCreateValidationCacheEXT,
    destroy_validation_cache_ext: PFN_vkDestroyValidationCacheEXT,
    merge_validation_caches_ext: PFN_vkMergeValidationCachesEXT,
    get_validation_cache_data_ext: PFN_vkGetValidationCacheDataEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_validation_cache_ext: transmute(
                    load(c"vkCreateValidationCacheEXT").ok_or(LoadingError)?,
                ),
                destroy_validation_cache_ext: transmute(
                    load(c"vkDestroyValidationCacheEXT").ok_or(LoadingError)?,
                ),
                merge_validation_caches_ext: transmute(
                    load(c"vkMergeValidationCachesEXT").ok_or(LoadingError)?,
                ),
                get_validation_cache_data_ext: transmute(
                    load(c"vkGetValidationCacheDataEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_validation_cache_ext(
        &self,
        device: Device,
        create_info: &ValidationCacheCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<ValidationCacheEXT> {
        unsafe {
            let mut validation_cache = core::mem::MaybeUninit::uninit();
            let result = (self.create_validation_cache_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                validation_cache.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(validation_cache.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_validation_cache_ext(
        &self,
        device: Device,
        validation_cache: ValidationCacheEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_validation_cache_ext)(device, validation_cache, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn merge_validation_caches_ext(
        &self,
        device: Device,
        dst_cache: ValidationCacheEXT,
        src_caches: &[ValidationCacheEXT],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.merge_validation_caches_ext)(
                device,
                dst_cache,
                src_caches.len().try_into().unwrap(),
                src_caches.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        device: Device,
        validation_cache: ValidationCacheEXT,
        data: impl ExtendUninit<u8>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(data, |data_size, data| {
                let result = (self.get_validation_cache_data_ext)(
                    device,
                    validation_cache,
                    data_size,
                    data as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
}
