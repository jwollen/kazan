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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceImageViewMinLodFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_lod: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageViewMinLodFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceImageViewMinLodFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceImageViewMinLodFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceImageViewMinLodFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                min_lod: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceImageViewMinLodFeaturesEXT<'a> {
        pub fn min_lod(mut self, min_lod: bool) -> Self {
            self.min_lod = min_lod.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageViewMinLodCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageViewMinLodCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub min_lod: f32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageViewMinLodCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT;
    }
    unsafe impl<'a> Extends<ImageViewCreateInfo<'a>> for ImageViewMinLodCreateInfoEXT<'a> {}
    impl Default for ImageViewMinLodCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                min_lod: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageViewMinLodCreateInfoEXT<'a> {
        pub fn min_lod(mut self, min_lod: f32) -> Self {
            self.min_lod = min_lod;
            self
        }
    }
}
