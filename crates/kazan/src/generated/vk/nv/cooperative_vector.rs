#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_cooperative_vector_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_cooperative_vector_properties_nv(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<CooperativeVectorPropertiesNV>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_cooperative_vector_properties_nv)(
                    physical_device,
                    property_count,
                    properties as _,
                )
            })
        }
    }
}
pub struct DeviceFn {
    convert_cooperative_vector_matrix_nv: PFN_vkConvertCooperativeVectorMatrixNV,
    cmd_convert_cooperative_vector_matrix_nv: PFN_vkCmdConvertCooperativeVectorMatrixNV,
}
impl DeviceFn {
    pub unsafe fn convert_cooperative_vector_matrix_nv(
        &self,
        device: Device,
        info: &ConvertCooperativeVectorMatrixInfoNV,
    ) -> Result {
        unsafe { (self.convert_cooperative_vector_matrix_nv)(device, info) }
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
