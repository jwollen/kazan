#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_HUAWEI_hdr_vivid";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHdrVividDynamicMetadataHUAWEI.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct HdrVividDynamicMetadataHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dynamic_metadata_size: usize,
        pub p_dynamic_metadata: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for HdrVividDynamicMetadataHUAWEI<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("HdrVividDynamicMetadataHUAWEI")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("dynamic_metadata_size", &self.dynamic_metadata_size)
                .field("p_dynamic_metadata", &self.p_dynamic_metadata)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for HdrVividDynamicMetadataHUAWEI<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::HDR_VIVID_DYNAMIC_METADATA_HUAWEI;
    }

    unsafe impl<'a> Extends<HdrMetadataEXT<'a>> for HdrVividDynamicMetadataHUAWEI<'a> {}

    impl Default for HdrVividDynamicMetadataHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                dynamic_metadata_size: Default::default(),
                p_dynamic_metadata: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> HdrVividDynamicMetadataHUAWEI<'a> {
        pub fn dynamic_metadata(mut self, dynamic_metadata: &'a [u8]) -> Self {
            self.dynamic_metadata_size = dynamic_metadata.len().try_into().unwrap();
            self.p_dynamic_metadata = dynamic_metadata.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceHdrVividFeaturesHUAWEI.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceHdrVividFeaturesHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub hdr_vivid: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceHdrVividFeaturesHUAWEI<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceHdrVividFeaturesHUAWEI")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("hdr_vivid", &self.hdr_vivid)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceHdrVividFeaturesHUAWEI<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceHdrVividFeaturesHUAWEI<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceHdrVividFeaturesHUAWEI<'a> {}

    impl Default for PhysicalDeviceHdrVividFeaturesHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                hdr_vivid: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceHdrVividFeaturesHUAWEI<'a> {
        pub fn hdr_vivid(mut self, hdr_vivid: bool) -> Self {
            self.hdr_vivid = hdr_vivid.into();
            self
        }
    }
}
