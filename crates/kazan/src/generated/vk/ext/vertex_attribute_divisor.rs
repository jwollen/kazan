//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_vertex_attribute_divisor.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_vertex_attribute_divisor";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVertexInputBindingDivisorDescriptionEXT.html>
    pub type VertexInputBindingDivisorDescriptionEXT = VertexInputBindingDivisorDescription;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineVertexInputDivisorStateCreateInfoEXT.html>
    pub type PipelineVertexInputDivisorStateCreateInfoEXT<'a> =
        PipelineVertexInputDivisorStateCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html>
    pub type PhysicalDeviceVertexAttributeDivisorFeaturesEXT<'a> =
        PhysicalDeviceVertexAttributeDivisorFeatures<'a>;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_vertex_attrib_divisor: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVertexAttributeDivisorPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_vertex_attrib_divisor", &self.max_vertex_attrib_divisor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_vertex_attrib_divisor: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'a> {
        #[inline]
        pub fn max_vertex_attrib_divisor(mut self, max_vertex_attrib_divisor: u32) -> Self {
            self.max_vertex_attrib_divisor = max_vertex_attrib_divisor;
            self
        }
    }
}
