//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance3.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_maintenance3";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance3PropertiesKHR.html>
    pub type PhysicalDeviceMaintenance3PropertiesKHR<'a> = PhysicalDeviceMaintenance3Properties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorSetLayoutSupportKHR.html>
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetLayoutSupportKHR.html>
    #[inline]
    pub unsafe fn get_descriptor_set_layout_support_khr(
        &self,
        device: Device,
        create_info: &DescriptorSetLayoutCreateInfo<'_>,
        support: &mut DescriptorSetLayoutSupport<'_>,
    ) {
        unsafe { (self.get_descriptor_set_layout_support_khr)(device, create_info, support) }
    }
}
