//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_tooling_info.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_tooling_info";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceToolPropertiesEXT.html>
    pub type PhysicalDeviceToolPropertiesEXT<'a> = PhysicalDeviceToolProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkToolPurposeFlagsEXT.html>
    pub type ToolPurposeFlagsEXT = ToolPurposeFlags;
    pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = PFN_vkGetPhysicalDeviceToolProperties;
}

pub struct InstanceFn {
    get_physical_device_tool_properties_ext: PFN_vkGetPhysicalDeviceToolProperties,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_tool_properties_ext: transmute(
                    load(c"vkGetPhysicalDeviceToolPropertiesEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html>
    #[inline]
    pub unsafe fn get_physical_device_tool_properties_ext<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut tool_properties: impl ExtendUninit<PhysicalDeviceToolProperties<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |tool_count, tool_properties| {
                let result = (self.get_physical_device_tool_properties_ext)(
                    physical_device,
                    tool_count,
                    tool_properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let tool_properties_buf = tool_properties.reserve(capacity);
            len = tool_properties_buf.len().try_into().unwrap();
            let result = call(&mut len, tool_properties_buf.as_mut_ptr() as *mut _)?;
            tool_properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}
