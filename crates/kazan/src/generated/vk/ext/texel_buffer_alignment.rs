//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_texel_buffer_alignment.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_texel_buffer_alignment";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT.html>
    pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXT<'a> =
        PhysicalDeviceTexelBufferAlignmentProperties<'a>;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub texel_buffer_alignment: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceTexelBufferAlignmentFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("texel_buffer_alignment", &self.texel_buffer_alignment)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                texel_buffer_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'a> {
        #[inline]
        pub fn texel_buffer_alignment(mut self, texel_buffer_alignment: bool) -> Self {
            self.texel_buffer_alignment = texel_buffer_alignment.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT =
        PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'static>;
    pub type VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT =
        PhysicalDeviceTexelBufferAlignmentPropertiesEXT<'static>;
    impl PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
