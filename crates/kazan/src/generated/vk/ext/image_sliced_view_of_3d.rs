#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub const REMAINING_3D_SLICES_EXT: u32 = !0;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageViewSlicedCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub slice_offset: u32,
        pub slice_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageViewSlicedCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_SLICED_CREATE_INFO_EXT;
    }
    unsafe impl<'a> Extends<ImageViewCreateInfo<'a>> for ImageViewSlicedCreateInfoEXT<'a> {}
    impl Default for ImageViewSlicedCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                slice_offset: Default::default(),
                slice_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageViewSlicedCreateInfoEXT<'a> {
        pub fn slice_offset(mut self, slice_offset: u32) -> Self {
            self.slice_offset = slice_offset;
            self
        }
        pub fn slice_count(mut self, slice_count: u32) -> Self {
            self.slice_count = slice_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_sliced_view_of3_d: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                image_sliced_view_of3_d: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'a> {
        pub fn image_sliced_view_of3_d(mut self, image_sliced_view_of3_d: Bool32) -> Self {
            self.image_sliced_view_of3_d = image_sliced_view_of3_d;
            self
        }
    }
}
