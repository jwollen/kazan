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
    pub struct PhysicalDeviceLayeredDriverPropertiesMSFT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub underlying_api: LayeredDriverUnderlyingApiMSFT,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceLayeredDriverPropertiesMSFT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT,
                p_next: core::ptr::null_mut(),
                underlying_api: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceLayeredDriverPropertiesMSFT<'a> {
        pub fn underlying_api(mut self, underlying_api: LayeredDriverUnderlyingApiMSFT) -> Self {
            self.underlying_api = underlying_api;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LayeredDriverUnderlyingApiMSFT(i32);
    impl LayeredDriverUnderlyingApiMSFT {
        pub const NONE_MSFT: Self = Self(0);
        pub const D3D12_MSFT: Self = Self(1);
    }
}
