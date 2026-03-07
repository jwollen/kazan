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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCommandBufferInheritanceFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub command_buffer_inheritance: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                command_buffer_inheritance: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {
        pub fn command_buffer_inheritance(mut self, command_buffer_inheritance: bool) -> Self {
            self.command_buffer_inheritance = command_buffer_inheritance.into();
            self
        }
    }
}
