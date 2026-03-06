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
    pub struct PhysicalDeviceYcbcrDegammaFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ycbcr_degamma: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceYcbcrDegammaFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceYcbcrDegammaFeaturesQCOM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceYcbcrDegammaFeaturesQCOM<'a> {}
    impl Default for PhysicalDeviceYcbcrDegammaFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                ycbcr_degamma: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceYcbcrDegammaFeaturesQCOM<'a> {
        pub fn ycbcr_degamma(mut self, ycbcr_degamma: bool) -> Self {
            self.ycbcr_degamma = ycbcr_degamma.into();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub enable_y_degamma: Bool32,
        pub enable_cb_cr_degamma: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM;
    }
    unsafe impl<'a> Extends<SamplerYcbcrConversionCreateInfo<'a>>
        for SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'a>
    {
    }
    impl Default for SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                enable_y_degamma: Default::default(),
                enable_cb_cr_degamma: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'a> {
        pub fn enable_y_degamma(mut self, enable_y_degamma: bool) -> Self {
            self.enable_y_degamma = enable_y_degamma.into();
            self
        }
        pub fn enable_cb_cr_degamma(mut self, enable_cb_cr_degamma: bool) -> Self {
            self.enable_cb_cr_degamma = enable_cb_cr_degamma.into();
            self
        }
    }
}
