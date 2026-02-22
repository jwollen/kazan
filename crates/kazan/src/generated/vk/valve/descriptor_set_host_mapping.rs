#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_descriptor_set_layout_host_mapping_info_valve:
        PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,
    get_descriptor_set_host_mapping_valve: PFN_vkGetDescriptorSetHostMappingVALVE,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_descriptor_set_layout_host_mapping_info_valve: transmute(
                    load(c"vkGetDescriptorSetLayoutHostMappingInfoVALVE").ok_or(LoadingError)?,
                ),
                get_descriptor_set_host_mapping_valve: transmute(
                    load(c"vkGetDescriptorSetHostMappingVALVE").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve(
        &self,
        device: Device,
        binding_reference: &DescriptorSetBindingReferenceVALVE,
    ) -> DescriptorSetLayoutHostMappingInfoVALVE {
        unsafe {
            let mut host_mapping = core::mem::MaybeUninit::uninit();
            (self.get_descriptor_set_layout_host_mapping_info_valve)(
                device,
                binding_reference,
                host_mapping.as_mut_ptr(),
            );
            host_mapping.assume_init()
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
