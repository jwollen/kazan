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
    pub struct PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format_rgba10x6_without_y_cb_cr_sampler: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                format_rgba10x6_without_y_cb_cr_sampler: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'a> {
        pub fn format_rgba10x6_without_y_cb_cr_sampler(
            mut self,
            format_rgba10x6_without_y_cb_cr_sampler: Bool32,
        ) -> Self {
            self.format_rgba10x6_without_y_cb_cr_sampler = format_rgba10x6_without_y_cb_cr_sampler;
            self
        }
    }
}
