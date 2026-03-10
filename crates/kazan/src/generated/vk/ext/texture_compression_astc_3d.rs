//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_texture_compression_astc_3d.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_texture_compression_astc_3d";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub texture_compression_astc_3d: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceTextureCompressionASTC3DFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "texture_compression_astc_3d",
                    &self.texture_compression_astc_3d,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                texture_compression_astc_3d: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'a> {
        #[inline]
        pub fn texture_compression_astc_3d(mut self, texture_compression_astc_3d: bool) -> Self {
            self.texture_compression_astc_3d = texture_compression_astc_3d.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT =
        PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'static>;
    impl PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
