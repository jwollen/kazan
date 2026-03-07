#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_texture_gather_bias_lod";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTextureLODGatherFormatPropertiesAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct TextureLODGatherFormatPropertiesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supports_texture_gather_lod_bias_amd: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for TextureLODGatherFormatPropertiesAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TextureLODGatherFormatPropertiesAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "supports_texture_gather_lod_bias_amd",
                    &self.supports_texture_gather_lod_bias_amd,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TextureLODGatherFormatPropertiesAMD<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD;
    }

    unsafe impl<'a> Extends<ImageFormatProperties2<'a>> for TextureLODGatherFormatPropertiesAMD<'a> {}

    impl Default for TextureLODGatherFormatPropertiesAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                supports_texture_gather_lod_bias_amd: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TextureLODGatherFormatPropertiesAMD<'a> {
        pub fn supports_texture_gather_lod_bias_amd(
            mut self,
            supports_texture_gather_lod_bias_amd: bool,
        ) -> Self {
            self.supports_texture_gather_lod_bias_amd = supports_texture_gather_lod_bias_amd.into();
            self
        }
    }
}
