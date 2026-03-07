#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_nested_command_buffer";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceNestedCommandBufferFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceNestedCommandBufferFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub nested_command_buffer: Bool32,
        pub nested_command_buffer_rendering: Bool32,
        pub nested_command_buffer_simultaneous_use: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceNestedCommandBufferFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceNestedCommandBufferFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("nested_command_buffer", &self.nested_command_buffer)
                .field(
                    "nested_command_buffer_rendering",
                    &self.nested_command_buffer_rendering,
                )
                .field(
                    "nested_command_buffer_simultaneous_use",
                    &self.nested_command_buffer_simultaneous_use,
                )
                .finish()
        }
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
        #[inline]
        pub fn nested_command_buffer(mut self, nested_command_buffer: bool) -> Self {
            self.nested_command_buffer = nested_command_buffer.into();
            self
        }

        #[inline]
        pub fn nested_command_buffer_rendering(
            mut self,
            nested_command_buffer_rendering: bool,
        ) -> Self {
            self.nested_command_buffer_rendering = nested_command_buffer_rendering.into();
            self
        }

        #[inline]
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
    #[must_use]
    pub struct PhysicalDeviceNestedCommandBufferPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_command_buffer_nesting_level: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceNestedCommandBufferPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceNestedCommandBufferPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_command_buffer_nesting_level",
                    &self.max_command_buffer_nesting_level,
                )
                .finish()
        }
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
        #[inline]
        pub fn max_command_buffer_nesting_level(
            mut self,
            max_command_buffer_nesting_level: u32,
        ) -> Self {
            self.max_command_buffer_nesting_level = max_command_buffer_nesting_level;
            self
        }
    }
}
