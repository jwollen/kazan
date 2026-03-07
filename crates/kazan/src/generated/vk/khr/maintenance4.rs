#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_maintenance4";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceBufferMemoryRequirementsKHR.html>
    pub type DeviceBufferMemoryRequirementsKHR<'a> = DeviceBufferMemoryRequirements<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceImageMemoryRequirementsKHR.html>
    pub type DeviceImageMemoryRequirementsKHR<'a> = DeviceImageMemoryRequirements<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance4FeaturesKHR.html>
    pub type PhysicalDeviceMaintenance4FeaturesKHR<'a> = PhysicalDeviceMaintenance4Features<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance4PropertiesKHR.html>
    pub type PhysicalDeviceMaintenance4PropertiesKHR<'a> = PhysicalDeviceMaintenance4Properties<'a>;
    pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = PFN_vkGetDeviceBufferMemoryRequirements;
    pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = PFN_vkGetDeviceImageMemoryRequirements;
    pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR =
        PFN_vkGetDeviceImageSparseMemoryRequirements;
}

pub struct DeviceFn {
    get_device_buffer_memory_requirements_khr: PFN_vkGetDeviceBufferMemoryRequirements,
    get_device_image_memory_requirements_khr: PFN_vkGetDeviceImageMemoryRequirements,
    get_device_image_sparse_memory_requirements_khr: PFN_vkGetDeviceImageSparseMemoryRequirements,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_device_buffer_memory_requirements_khr: transmute(
                    load(c"vkGetDeviceBufferMemoryRequirementsKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_device_image_memory_requirements_khr: transmute(
                    load(c"vkGetDeviceImageMemoryRequirementsKHR").ok_or(MissingEntryPointError)?,
                ),
                get_device_image_sparse_memory_requirements_khr: transmute(
                    load(c"vkGetDeviceImageSparseMemoryRequirementsKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceBufferMemoryRequirementsKHR.html>
    #[inline]
    pub unsafe fn get_device_buffer_memory_requirements_khr(
        &self,
        device: Device,
        info: &DeviceBufferMemoryRequirements<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.get_device_buffer_memory_requirements_khr)(device, info, memory_requirements)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceImageMemoryRequirementsKHR.html>
    #[inline]
    pub unsafe fn get_device_image_memory_requirements_khr(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.get_device_image_memory_requirements_khr)(device, info, memory_requirements)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceImageSparseMemoryRequirementsKHR.html>
    #[inline]
    pub unsafe fn get_device_image_sparse_memory_requirements_khr<'a>(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements<'a>,
        mut sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2<'a>>,
    ) {
        unsafe {
            let call = |sparse_memory_requirement_count, sparse_memory_requirements| {
                (self.get_device_image_sparse_memory_requirements_khr)(
                    device,
                    info,
                    sparse_memory_requirement_count,
                    sparse_memory_requirements as _,
                )
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut());
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let sparse_memory_requirements_buf = sparse_memory_requirements.reserve(capacity);
            len = sparse_memory_requirements_buf.len().try_into().unwrap();
            call(
                &mut len,
                sparse_memory_requirements_buf.as_mut_ptr() as *mut _,
            );
            sparse_memory_requirements.set_len(len.try_into().unwrap());
        }
    }
}
