//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_command_buffer_inheritance.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_command_buffer_inheritance";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCommandBufferInheritanceFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub command_buffer_inheritance: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCommandBufferInheritanceFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCommandBufferInheritanceFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "command_buffer_inheritance",
                    &self.command_buffer_inheritance,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceCommandBufferInheritanceFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceCommandBufferInheritanceFeaturesNV<'a>
    {
    }

    impl Default for PhysicalDeviceCommandBufferInheritanceFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                command_buffer_inheritance: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {
        #[inline]
        pub fn command_buffer_inheritance(mut self, command_buffer_inheritance: bool) -> Self {
            self.command_buffer_inheritance = command_buffer_inheritance.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceCommandBufferInheritanceFeaturesNV =
        PhysicalDeviceCommandBufferInheritanceFeaturesNV<'static>;
    impl PhysicalDeviceCommandBufferInheritanceFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCommandBufferInheritanceFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}
