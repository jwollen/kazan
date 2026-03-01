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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image2_d_view_of3_d: Bool32,
        pub sampler2_d_view_of3_d: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                image2_d_view_of3_d: Default::default(),
                sampler2_d_view_of3_d: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a> {
        pub fn image2_d_view_of3_d(mut self, image2_d_view_of3_d: Bool32) -> Self {
            self.image2_d_view_of3_d = image2_d_view_of3_d;
            self
        }
        pub fn sampler2_d_view_of3_d(mut self, sampler2_d_view_of3_d: Bool32) -> Self {
            self.sampler2_d_view_of3_d = sampler2_d_view_of3_d;
            self
        }
    }
}
