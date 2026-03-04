#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    pub type PhysicalDeviceMaintenance3PropertiesKHR<'a> = PhysicalDeviceMaintenance3Properties<'a>;
    pub type DescriptorSetLayoutSupportKHR<'a> = DescriptorSetLayoutSupport<'a>;
    pub type PFN_vkGetDescriptorSetLayoutSupportKHR = PFN_vkGetDescriptorSetLayoutSupport;
}
pub struct DeviceFn {
    get_descriptor_set_layout_support_khr: PFN_vkGetDescriptorSetLayoutSupport,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_descriptor_set_layout_support_khr: transmute(
                    load(c"vkGetDescriptorSetLayoutSupportKHR").ok_or(MissingEntryPointError)?,
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
