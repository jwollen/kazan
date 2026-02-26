#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_descriptor_set_layout_support_khr: PFN_vkGetDescriptorSetLayoutSupport,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_descriptor_set_layout_support_khr: transmute(
                    load(c"vkGetDescriptorSetLayoutSupportKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_descriptor_set_layout_support_khr(
        &self,
        device: Device,
        create_info: &DescriptorSetLayoutCreateInfo<'_>,
    ) -> DescriptorSetLayoutSupport<'_> {
        unsafe {
            let mut support = core::mem::MaybeUninit::uninit();
            (self.get_descriptor_set_layout_support_khr)(device, create_info, support.as_mut_ptr());
            support.assume_init()
        }
    }
}
