#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXT<'a> =
        PhysicalDeviceTexelBufferAlignmentProperties<'a>;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub texel_buffer_alignment: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT,
                p_next: core::ptr::null_mut(),
                texel_buffer_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'a> {
        pub fn texel_buffer_alignment(mut self, texel_buffer_alignment: Bool32) -> Self {
            self.texel_buffer_alignment = texel_buffer_alignment;
            self
        }
    }
}
