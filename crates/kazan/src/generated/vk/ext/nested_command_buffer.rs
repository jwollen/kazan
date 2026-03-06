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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceNestedCommandBufferFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceNestedCommandBufferFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub nested_command_buffer: Bool32,
        pub nested_command_buffer_rendering: Bool32,
        pub nested_command_buffer_simultaneous_use: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceNestedCommandBufferFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceNestedCommandBufferFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceNestedCommandBufferFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceNestedCommandBufferFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                nested_command_buffer: Default::default(),
                nested_command_buffer_rendering: Default::default(),
                nested_command_buffer_simultaneous_use: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceNestedCommandBufferFeaturesEXT<'a> {
        pub fn nested_command_buffer(mut self, nested_command_buffer: bool) -> Self {
            self.nested_command_buffer = nested_command_buffer.into();
            self
        }
        pub fn nested_command_buffer_rendering(
            mut self,
            nested_command_buffer_rendering: bool,
        ) -> Self {
            self.nested_command_buffer_rendering = nested_command_buffer_rendering.into();
            self
        }
        pub fn nested_command_buffer_simultaneous_use(
            mut self,
            nested_command_buffer_simultaneous_use: bool,
        ) -> Self {
            self.nested_command_buffer_simultaneous_use =
                nested_command_buffer_simultaneous_use.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceNestedCommandBufferPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceNestedCommandBufferPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_command_buffer_nesting_level: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceNestedCommandBufferPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceNestedCommandBufferPropertiesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceNestedCommandBufferPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_command_buffer_nesting_level: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceNestedCommandBufferPropertiesEXT<'a> {
        pub fn max_command_buffer_nesting_level(
            mut self,
            max_command_buffer_nesting_level: u32,
        ) -> Self {
            self.max_command_buffer_nesting_level = max_command_buffer_nesting_level;
            self
        }
    }
}
