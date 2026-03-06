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
    pub struct PhysicalDeviceImageProcessing2FeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub texture_block_match2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageProcessing2FeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceImageProcessing2FeaturesQCOM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceImageProcessing2FeaturesQCOM<'a> {}
    impl Default for PhysicalDeviceImageProcessing2FeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                texture_block_match2: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceImageProcessing2FeaturesQCOM<'a> {
        pub fn texture_block_match2(mut self, texture_block_match2: bool) -> Self {
            self.texture_block_match2 = texture_block_match2.into();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceImageProcessing2PropertiesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_block_match_window: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageProcessing2PropertiesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceImageProcessing2PropertiesQCOM<'a>
    {
    }
    impl Default for PhysicalDeviceImageProcessing2PropertiesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_block_match_window: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceImageProcessing2PropertiesQCOM<'a> {
        pub fn max_block_match_window(mut self, max_block_match_window: Extent2D) -> Self {
            self.max_block_match_window = max_block_match_window;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SamplerBlockMatchWindowCreateInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub window_extent: Extent2D,
        pub window_compare_mode: BlockMatchWindowCompareModeQCOM,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SamplerBlockMatchWindowCreateInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM;
    }
    unsafe impl<'a> Extends<SamplerCreateInfo<'a>> for SamplerBlockMatchWindowCreateInfoQCOM<'a> {}
    impl Default for SamplerBlockMatchWindowCreateInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                window_extent: Default::default(),
                window_compare_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SamplerBlockMatchWindowCreateInfoQCOM<'a> {
        pub fn window_extent(mut self, window_extent: Extent2D) -> Self {
            self.window_extent = window_extent;
            self
        }
        pub fn window_compare_mode(
            mut self,
            window_compare_mode: BlockMatchWindowCompareModeQCOM,
        ) -> Self {
            self.window_compare_mode = window_compare_mode;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BlockMatchWindowCompareModeQCOM(i32);
    impl BlockMatchWindowCompareModeQCOM {
        pub const MIN_QCOM: Self = Self(0);
        pub const MAX_QCOM: Self = Self(1);
    }
    impl fmt::Debug for BlockMatchWindowCompareModeQCOM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MIN_QCOM => Some("MIN_QCOM"),
                Self::MAX_QCOM => Some("MAX_QCOM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}
