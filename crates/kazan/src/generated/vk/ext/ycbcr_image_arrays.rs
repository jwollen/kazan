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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ycbcr_image_arrays: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceYcbcrImageArraysFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceYcbcrImageArraysFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceYcbcrImageArraysFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceYcbcrImageArraysFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                ycbcr_image_arrays: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceYcbcrImageArraysFeaturesEXT<'a> {
        pub fn ycbcr_image_arrays(mut self, ycbcr_image_arrays: bool) -> Self {
            self.ycbcr_image_arrays = ycbcr_image_arrays.into();
            self
        }
    }
}
