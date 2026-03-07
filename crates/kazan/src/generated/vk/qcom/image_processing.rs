//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_image_processing.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_image_processing";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageViewSampleWeightCreateInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageViewSampleWeightCreateInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub filter_center: Offset2D,
        pub filter_size: Extent2D,
        pub num_phases: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageViewSampleWeightCreateInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageViewSampleWeightCreateInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("filter_center", &self.filter_center)
                .field("filter_size", &self.filter_size)
                .field("num_phases", &self.num_phases)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageViewSampleWeightCreateInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM;
    }

    unsafe impl<'a> Extends<ImageViewCreateInfo<'a>> for ImageViewSampleWeightCreateInfoQCOM<'a> {}

    impl Default for ImageViewSampleWeightCreateInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                filter_center: Default::default(),
                filter_size: Default::default(),
                num_phases: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageViewSampleWeightCreateInfoQCOM<'a> {
        #[inline]
        pub fn filter_center(mut self, filter_center: Offset2D) -> Self {
            self.filter_center = filter_center;
            self
        }

        #[inline]
        pub fn filter_size(mut self, filter_size: Extent2D) -> Self {
            self.filter_size = filter_size;
            self
        }

        #[inline]
        pub fn num_phases(mut self, num_phases: u32) -> Self {
            self.num_phases = num_phases;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageProcessingFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageProcessingFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub texture_sample_weighted: Bool32,
        pub texture_box_filter: Bool32,
        pub texture_block_match: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageProcessingFeaturesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageProcessingFeaturesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("texture_sample_weighted", &self.texture_sample_weighted)
                .field("texture_box_filter", &self.texture_box_filter)
                .field("texture_block_match", &self.texture_block_match)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageProcessingFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceImageProcessingFeaturesQCOM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceImageProcessingFeaturesQCOM<'a> {}

    impl Default for PhysicalDeviceImageProcessingFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                texture_sample_weighted: Default::default(),
                texture_box_filter: Default::default(),
                texture_block_match: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageProcessingFeaturesQCOM<'a> {
        #[inline]
        pub fn texture_sample_weighted(mut self, texture_sample_weighted: bool) -> Self {
            self.texture_sample_weighted = texture_sample_weighted.into();
            self
        }

        #[inline]
        pub fn texture_box_filter(mut self, texture_box_filter: bool) -> Self {
            self.texture_box_filter = texture_box_filter.into();
            self
        }

        #[inline]
        pub fn texture_block_match(mut self, texture_block_match: bool) -> Self {
            self.texture_block_match = texture_block_match.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageProcessingPropertiesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageProcessingPropertiesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_weight_filter_phases: u32,
        pub max_weight_filter_dimension: Extent2D,
        pub max_block_match_region: Extent2D,
        pub max_box_filter_block_size: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageProcessingPropertiesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageProcessingPropertiesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_weight_filter_phases", &self.max_weight_filter_phases)
                .field(
                    "max_weight_filter_dimension",
                    &self.max_weight_filter_dimension,
                )
                .field("max_block_match_region", &self.max_block_match_region)
                .field("max_box_filter_block_size", &self.max_box_filter_block_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageProcessingPropertiesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceImageProcessingPropertiesQCOM<'a>
    {
    }

    impl Default for PhysicalDeviceImageProcessingPropertiesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_weight_filter_phases: Default::default(),
                max_weight_filter_dimension: Default::default(),
                max_block_match_region: Default::default(),
                max_box_filter_block_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageProcessingPropertiesQCOM<'a> {
        #[inline]
        pub fn max_weight_filter_phases(mut self, max_weight_filter_phases: u32) -> Self {
            self.max_weight_filter_phases = max_weight_filter_phases;
            self
        }

        #[inline]
        pub fn max_weight_filter_dimension(
            mut self,
            max_weight_filter_dimension: Extent2D,
        ) -> Self {
            self.max_weight_filter_dimension = max_weight_filter_dimension;
            self
        }

        #[inline]
        pub fn max_block_match_region(mut self, max_block_match_region: Extent2D) -> Self {
            self.max_block_match_region = max_block_match_region;
            self
        }

        #[inline]
        pub fn max_box_filter_block_size(mut self, max_box_filter_block_size: Extent2D) -> Self {
            self.max_box_filter_block_size = max_box_filter_block_size;
            self
        }
    }
}
