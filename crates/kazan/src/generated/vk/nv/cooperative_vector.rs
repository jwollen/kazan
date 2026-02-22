#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_cooperative_vector_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_cooperative_vector_properties_nv: transmute(
                    load(c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_cooperative_vector_properties_nv(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<CooperativeVectorPropertiesNV>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_physical_device_cooperative_vector_properties_nv)(
                    physical_device,
                    property_count,
                    properties as _,
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
pub struct DeviceFn {
    convert_cooperative_vector_matrix_nv: PFN_vkConvertCooperativeVectorMatrixNV,
    cmd_convert_cooperative_vector_matrix_nv: PFN_vkCmdConvertCooperativeVectorMatrixNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                convert_cooperative_vector_matrix_nv: transmute(
                    load(c"vkConvertCooperativeVectorMatrixNV").ok_or(LoadingError)?,
                ),
                cmd_convert_cooperative_vector_matrix_nv: transmute(
                    load(c"vkCmdConvertCooperativeVectorMatrixNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn convert_cooperative_vector_matrix_nv(
        &self,
        device: Device,
        info: &ConvertCooperativeVectorMatrixInfoNV,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.convert_cooperative_vector_matrix_nv)(device, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::INCOMPLETE => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_convert_cooperative_vector_matrix_nv(
        &self,
        command_buffer: CommandBuffer,
        infos: &[ConvertCooperativeVectorMatrixInfoNV],
    ) {
        unsafe {
            (self.cmd_convert_cooperative_vector_matrix_nv)(
                command_buffer,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
            )
        }
    }
}
