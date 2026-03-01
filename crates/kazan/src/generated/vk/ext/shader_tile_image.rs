#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderTileImageFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_tile_image_color_read_access: Bool32,
        pub shader_tile_image_depth_read_access: Bool32,
        pub shader_tile_image_stencil_read_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderTileImageFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderTileImageFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderTileImageFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceShaderTileImageFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_tile_image_color_read_access: Default::default(),
                shader_tile_image_depth_read_access: Default::default(),
                shader_tile_image_stencil_read_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderTileImageFeaturesEXT<'a> {
        pub fn shader_tile_image_color_read_access(
            mut self,
            shader_tile_image_color_read_access: Bool32,
        ) -> Self {
            self.shader_tile_image_color_read_access = shader_tile_image_color_read_access;
            self
        }
        pub fn shader_tile_image_depth_read_access(
            mut self,
            shader_tile_image_depth_read_access: Bool32,
        ) -> Self {
            self.shader_tile_image_depth_read_access = shader_tile_image_depth_read_access;
            self
        }
        pub fn shader_tile_image_stencil_read_access(
            mut self,
            shader_tile_image_stencil_read_access: Bool32,
        ) -> Self {
            self.shader_tile_image_stencil_read_access = shader_tile_image_stencil_read_access;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderTileImagePropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_tile_image_coherent_read_accelerated: Bool32,
        pub shader_tile_image_read_sample_from_pixel_rate_invocation: Bool32,
        pub shader_tile_image_read_from_helper_invocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderTileImagePropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShaderTileImagePropertiesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceShaderTileImagePropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_tile_image_coherent_read_accelerated: Default::default(),
                shader_tile_image_read_sample_from_pixel_rate_invocation: Default::default(),
                shader_tile_image_read_from_helper_invocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderTileImagePropertiesEXT<'a> {
        pub fn shader_tile_image_coherent_read_accelerated(
            mut self,
            shader_tile_image_coherent_read_accelerated: Bool32,
        ) -> Self {
            self.shader_tile_image_coherent_read_accelerated =
                shader_tile_image_coherent_read_accelerated;
            self
        }
        pub fn shader_tile_image_read_sample_from_pixel_rate_invocation(
            mut self,
            shader_tile_image_read_sample_from_pixel_rate_invocation: Bool32,
        ) -> Self {
            self.shader_tile_image_read_sample_from_pixel_rate_invocation =
                shader_tile_image_read_sample_from_pixel_rate_invocation;
            self
        }
        pub fn shader_tile_image_read_from_helper_invocation(
            mut self,
            shader_tile_image_read_from_helper_invocation: Bool32,
        ) -> Self {
            self.shader_tile_image_read_from_helper_invocation =
                shader_tile_image_read_from_helper_invocation;
            self
        }
    }
}
