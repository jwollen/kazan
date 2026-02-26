#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_device_buffer_memory_requirements_khr: PFN_vkGetDeviceBufferMemoryRequirements,
    get_device_image_memory_requirements_khr: PFN_vkGetDeviceImageMemoryRequirements,
    get_device_image_sparse_memory_requirements_khr: PFN_vkGetDeviceImageSparseMemoryRequirements,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_device_buffer_memory_requirements_khr: transmute(
                    load(c"vkGetDeviceBufferMemoryRequirementsKHR").ok_or(LoadingError)?,
                ),
                get_device_image_memory_requirements_khr: transmute(
                    load(c"vkGetDeviceImageMemoryRequirementsKHR").ok_or(LoadingError)?,
                ),
                get_device_image_sparse_memory_requirements_khr: transmute(
                    load(c"vkGetDeviceImageSparseMemoryRequirementsKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_device_buffer_memory_requirements_khr(
        &self,
        device: Device,
        info: &DeviceBufferMemoryRequirements<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_device_buffer_memory_requirements_khr)(
                device,
                info,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn get_device_image_memory_requirements_khr(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_device_image_memory_requirements_khr)(
                device,
                info,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn get_device_image_sparse_memory_requirements_khr<'a>(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements<'_>,
        sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2<'a>>,
    ) {
        unsafe {
            extend_uninit(
                sparse_memory_requirements,
                |sparse_memory_requirement_count, sparse_memory_requirements| {
                    (self.get_device_image_sparse_memory_requirements_khr)(
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
