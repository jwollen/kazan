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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFormatPackFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format_pack: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceFormatPackFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM,
                p_next: core::ptr::null_mut(),
                format_pack: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFormatPackFeaturesARM<'a> {
        pub fn format_pack(mut self, format_pack: Bool32) -> Self {
            self.format_pack = format_pack;
            self
        }
    }
}
