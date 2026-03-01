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
    pub type VertexInputBindingDivisorDescriptionEXT = VertexInputBindingDivisorDescription;
    pub type PipelineVertexInputDivisorStateCreateInfoEXT<'a> =
        PipelineVertexInputDivisorStateCreateInfo<'a>;
    pub type PhysicalDeviceVertexAttributeDivisorFeaturesEXT<'a> =
        PhysicalDeviceVertexAttributeDivisorFeatures<'a>;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_vertex_attrib_divisor: u32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                max_vertex_attrib_divisor: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'a> {
        pub fn max_vertex_attrib_divisor(mut self, max_vertex_attrib_divisor: u32) -> Self {
            self.max_vertex_attrib_divisor = max_vertex_attrib_divisor;
            self
        }
    }
}
