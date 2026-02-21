#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_descriptor_set_layout_support: PFN_vkGetDescriptorSetLayoutSupport,
}
impl DeviceFn {
    pub unsafe fn get_descriptor_set_layout_support_khr(
        &self,
        device: Device,
        create_info: &DescriptorSetLayoutCreateInfo,
        support: &mut DescriptorSetLayoutSupport,
    ) {
        unsafe { (self.get_descriptor_set_layout_support)(device, create_info, support) }
    }
}
