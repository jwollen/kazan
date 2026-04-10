//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_shader_abort.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_shader_abort";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderAbortFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderAbortFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_abort: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderAbortFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderAbortFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_abort", &self.shader_abort)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderAbortFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_ABORT_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceShaderAbortFeaturesKHR<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceShaderAbortFeaturesKHR<'_> {}

    impl Default for PhysicalDeviceShaderAbortFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_abort: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderAbortFeaturesKHR<'a> {
        #[inline]
        pub fn shader_abort(mut self, shader_abort: bool) -> Self {
            self.shader_abort = shader_abort.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderAbortPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderAbortPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_shader_abort_message_size: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderAbortPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderAbortPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_shader_abort_message_size",
                    &self.max_shader_abort_message_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderAbortPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_ABORT_PROPERTIES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>> for PhysicalDeviceShaderAbortPropertiesKHR<'_> {}

    impl Default for PhysicalDeviceShaderAbortPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_shader_abort_message_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderAbortPropertiesKHR<'a> {
        #[inline]
        pub fn max_shader_abort_message_size(mut self, max_shader_abort_message_size: u64) -> Self {
            self.max_shader_abort_message_size = max_shader_abort_message_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultShaderAbortMessageInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceFaultShaderAbortMessageInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub message_data_size: u64,
        pub p_message_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceFaultShaderAbortMessageInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceFaultShaderAbortMessageInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("message_data_size", &self.message_data_size)
                .field("p_message_data", &self.p_message_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceFaultShaderAbortMessageInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEVICE_FAULT_SHADER_ABORT_MESSAGE_INFO_KHR;
    }

    unsafe impl Extends<DeviceFaultDebugInfoKHR<'_>> for DeviceFaultShaderAbortMessageInfoKHR<'_> {}

    impl Default for DeviceFaultShaderAbortMessageInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                message_data_size: Default::default(),
                p_message_data: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceFaultShaderAbortMessageInfoKHR<'a> {
        #[inline]
        pub fn message_data(mut self, message_data: &'a mut [u8]) -> Self {
            self.message_data_size = message_data.len().try_into().unwrap();
            self.p_message_data = message_data.as_mut_ptr() as _;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceShaderAbortFeaturesKHR = PhysicalDeviceShaderAbortFeaturesKHR<'static>;
    pub type VkPhysicalDeviceShaderAbortPropertiesKHR =
        PhysicalDeviceShaderAbortPropertiesKHR<'static>;
    pub type VkDeviceFaultShaderAbortMessageInfoKHR = DeviceFaultShaderAbortMessageInfoKHR<'static>;
    impl PhysicalDeviceShaderAbortFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceShaderAbortFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceShaderAbortPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceShaderAbortPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceFaultShaderAbortMessageInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceFaultShaderAbortMessageInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
