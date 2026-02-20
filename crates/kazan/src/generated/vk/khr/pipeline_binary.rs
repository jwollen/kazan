#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    create_pipeline_binaries_khr: PFN_vkCreatePipelineBinariesKHR,
    destroy_pipeline_binary_khr: PFN_vkDestroyPipelineBinaryKHR,
    get_pipeline_key_khr: PFN_vkGetPipelineKeyKHR,
    get_pipeline_binary_data_khr: PFN_vkGetPipelineBinaryDataKHR,
    release_captured_pipeline_data_khr: PFN_vkReleaseCapturedPipelineDataKHR,
}
impl DeviceFn {
    pub unsafe fn create_pipeline_binaries_khr(
        &self,
        device: Device,
        create_info: &PipelineBinaryCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
        binaries: &mut PipelineBinaryHandlesInfoKHR,
    ) -> Result {
        unsafe {
            (self.create_pipeline_binaries_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                binaries,
            )
        }
    }
    pub unsafe fn destroy_pipeline_binary_khr(
        &self,
        device: Device,
        pipeline_binary: PipelineBinaryKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_pipeline_binary_khr)(device, pipeline_binary, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn get_pipeline_key_khr(
        &self,
        device: Device,
        pipeline_create_info: Option<&PipelineCreateInfoKHR>,
        pipeline_key: &mut PipelineBinaryKeyKHR,
    ) -> Result {
        unsafe {
            (self.get_pipeline_key_khr)(device, pipeline_create_info.to_raw_ptr(), pipeline_key)
        }
    }
    pub unsafe fn get_pipeline_binary_data_khr(
        &self,
        device: Device,
        info: &PipelineBinaryDataInfoKHR,
        pipeline_binary_key: &mut PipelineBinaryKeyKHR,
        pipeline_binary_data: impl ExtendUninit<u8>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                pipeline_binary_data,
                |pipeline_binary_data_size, pipeline_binary_data| {
                    (self.get_pipeline_binary_data_khr)(
                        device,
                        info,
                        pipeline_binary_key,
                        pipeline_binary_data_size,
                        pipeline_binary_data as _,
                    )
                },
            )
        }
    }
    pub unsafe fn release_captured_pipeline_data_khr(
        &self,
        device: Device,
        info: &ReleaseCapturedPipelineDataInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result {
        unsafe { (self.release_captured_pipeline_data_khr)(device, info, allocator.to_raw_ptr()) }
    }
}
