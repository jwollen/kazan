#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_descriptor_set_layout_host_mapping_info_valve:
        PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,
    get_descriptor_set_host_mapping_valve: PFN_vkGetDescriptorSetHostMappingVALVE,
}
impl DeviceFn {
    pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve(
        &self,
        device: Device,
        binding_reference: &DescriptorSetBindingReferenceVALVE,
        host_mapping: &mut DescriptorSetLayoutHostMappingInfoVALVE,
    ) {
        unsafe {
            (self.get_descriptor_set_layout_host_mapping_info_valve)(
                device,
                binding_reference,
                host_mapping,
            )
        }
    }
    pub unsafe fn get_descriptor_set_host_mapping_valve(
        &self,
        device: Device,
        descriptor_set: DescriptorSet,
        data: &mut *mut c_void,
    ) {
        unsafe { (self.get_descriptor_set_host_mapping_valve)(device, descriptor_set, data) }
    }
}
