//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_video_encode_quantization_map.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_encode_quantization_map";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeQuantizationMapCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeQuantizationMapCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_quantization_map_extent: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeQuantizationMapCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeQuantizationMapCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_quantization_map_extent",
                    &self.max_quantization_map_extent,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeQuantizationMapCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>>
        for VideoEncodeQuantizationMapCapabilitiesKHR<'a>
    {
    }

    impl Default for VideoEncodeQuantizationMapCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_quantization_map_extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeQuantizationMapCapabilitiesKHR<'a> {
        #[inline]
        pub fn max_quantization_map_extent(
            mut self,
            max_quantization_map_extent: Extent2D,
        ) -> Self {
            self.max_quantization_map_extent = max_quantization_map_extent;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264QuantizationMapCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264QuantizationMapCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_qp_delta: i32,
        pub max_qp_delta: i32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264QuantizationMapCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264QuantizationMapCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("min_qp_delta", &self.min_qp_delta)
                .field("max_qp_delta", &self.max_qp_delta)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264QuantizationMapCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>>
        for VideoEncodeH264QuantizationMapCapabilitiesKHR<'a>
    {
    }

    impl Default for VideoEncodeH264QuantizationMapCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                min_qp_delta: Default::default(),
                max_qp_delta: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264QuantizationMapCapabilitiesKHR<'a> {
        #[inline]
        pub fn min_qp_delta(mut self, min_qp_delta: i32) -> Self {
            self.min_qp_delta = min_qp_delta;
            self
        }

        #[inline]
        pub fn max_qp_delta(mut self, max_qp_delta: i32) -> Self {
            self.max_qp_delta = max_qp_delta;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265QuantizationMapCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH265QuantizationMapCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_qp_delta: i32,
        pub max_qp_delta: i32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH265QuantizationMapCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH265QuantizationMapCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("min_qp_delta", &self.min_qp_delta)
                .field("max_qp_delta", &self.max_qp_delta)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265QuantizationMapCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>>
        for VideoEncodeH265QuantizationMapCapabilitiesKHR<'a>
    {
    }

    impl Default for VideoEncodeH265QuantizationMapCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                min_qp_delta: Default::default(),
                max_qp_delta: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH265QuantizationMapCapabilitiesKHR<'a> {
        #[inline]
        pub fn min_qp_delta(mut self, min_qp_delta: i32) -> Self {
            self.min_qp_delta = min_qp_delta;
            self
        }

        #[inline]
        pub fn max_qp_delta(mut self, max_qp_delta: i32) -> Self {
            self.max_qp_delta = max_qp_delta;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1QuantizationMapCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeAV1QuantizationMapCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_q_index_delta: i32,
        pub max_q_index_delta: i32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeAV1QuantizationMapCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeAV1QuantizationMapCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("min_q_index_delta", &self.min_q_index_delta)
                .field("max_q_index_delta", &self.max_q_index_delta)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1QuantizationMapCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>>
        for VideoEncodeAV1QuantizationMapCapabilitiesKHR<'a>
    {
    }

    impl Default for VideoEncodeAV1QuantizationMapCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                min_q_index_delta: Default::default(),
                max_q_index_delta: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1QuantizationMapCapabilitiesKHR<'a> {
        #[inline]
        pub fn min_q_index_delta(mut self, min_q_index_delta: i32) -> Self {
            self.min_q_index_delta = min_q_index_delta;
            self
        }

        #[inline]
        pub fn max_q_index_delta(mut self, max_q_index_delta: i32) -> Self {
            self.max_q_index_delta = max_q_index_delta;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoFormatQuantizationMapPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoFormatQuantizationMapPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub quantization_map_texel_size: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoFormatQuantizationMapPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoFormatQuantizationMapPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "quantization_map_texel_size",
                    &self.quantization_map_texel_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoFormatQuantizationMapPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<VideoFormatPropertiesKHR<'a>>
        for VideoFormatQuantizationMapPropertiesKHR<'a>
    {
    }

    impl Default for VideoFormatQuantizationMapPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                quantization_map_texel_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoFormatQuantizationMapPropertiesKHR<'a> {
        #[inline]
        pub fn quantization_map_texel_size(
            mut self,
            quantization_map_texel_size: Extent2D,
        ) -> Self {
            self.quantization_map_texel_size = quantization_map_texel_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoFormatH265QuantizationMapPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoFormatH265QuantizationMapPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub compatible_ctb_sizes: VideoEncodeH265CtbSizeFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoFormatH265QuantizationMapPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoFormatH265QuantizationMapPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("compatible_ctb_sizes", &self.compatible_ctb_sizes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoFormatH265QuantizationMapPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<VideoFormatPropertiesKHR<'a>>
        for VideoFormatH265QuantizationMapPropertiesKHR<'a>
    {
    }

    impl Default for VideoFormatH265QuantizationMapPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                compatible_ctb_sizes: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoFormatH265QuantizationMapPropertiesKHR<'a> {
        #[inline]
        pub fn compatible_ctb_sizes(
            mut self,
            compatible_ctb_sizes: VideoEncodeH265CtbSizeFlagsKHR,
        ) -> Self {
            self.compatible_ctb_sizes = compatible_ctb_sizes;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoFormatAV1QuantizationMapPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoFormatAV1QuantizationMapPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub compatible_superblock_sizes: VideoEncodeAV1SuperblockSizeFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoFormatAV1QuantizationMapPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoFormatAV1QuantizationMapPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "compatible_superblock_sizes",
                    &self.compatible_superblock_sizes,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoFormatAV1QuantizationMapPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<VideoFormatPropertiesKHR<'a>>
        for VideoFormatAV1QuantizationMapPropertiesKHR<'a>
    {
    }

    impl Default for VideoFormatAV1QuantizationMapPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                compatible_superblock_sizes: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoFormatAV1QuantizationMapPropertiesKHR<'a> {
        #[inline]
        pub fn compatible_superblock_sizes(
            mut self,
            compatible_superblock_sizes: VideoEncodeAV1SuperblockSizeFlagsKHR,
        ) -> Self {
            self.compatible_superblock_sizes = compatible_superblock_sizes;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeQuantizationMapInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeQuantizationMapInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub quantization_map: ImageView,
        pub quantization_map_extent: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeQuantizationMapInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeQuantizationMapInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("quantization_map", &self.quantization_map)
                .field("quantization_map_extent", &self.quantization_map_extent)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeQuantizationMapInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoEncodeInfoKHR<'a>> for VideoEncodeQuantizationMapInfoKHR<'a> {}

    impl Default for VideoEncodeQuantizationMapInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                quantization_map: Default::default(),
                quantization_map_extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeQuantizationMapInfoKHR<'a> {
        #[inline]
        pub fn quantization_map(mut self, quantization_map: ImageView) -> Self {
            self.quantization_map = quantization_map;
            self
        }

        #[inline]
        pub fn quantization_map_extent(mut self, quantization_map_extent: Extent2D) -> Self {
            self.quantization_map_extent = quantization_map_extent;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeQuantizationMapSessionParametersCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub quantization_map_texel_size: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeQuantizationMapSessionParametersCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "quantization_map_texel_size",
                    &self.quantization_map_texel_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionParametersCreateInfoKHR<'a>>
        for VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                quantization_map_texel_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'a> {
        #[inline]
        pub fn quantization_map_texel_size(
            mut self,
            quantization_map_texel_size: Extent2D,
        ) -> Self {
            self.quantization_map_texel_size = quantization_map_texel_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_encode_quantization_map: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "video_encode_quantization_map",
                    &self.video_encode_quantization_map,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                video_encode_quantization_map: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'a> {
        #[inline]
        pub fn video_encode_quantization_map(
            mut self,
            video_encode_quantization_map: bool,
        ) -> Self {
            self.video_encode_quantization_map = video_encode_quantization_map.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkVideoEncodeQuantizationMapCapabilitiesKHR =
        VideoEncodeQuantizationMapCapabilitiesKHR<'static>;
    pub type VkVideoEncodeH264QuantizationMapCapabilitiesKHR =
        VideoEncodeH264QuantizationMapCapabilitiesKHR<'static>;
    pub type VkVideoEncodeH265QuantizationMapCapabilitiesKHR =
        VideoEncodeH265QuantizationMapCapabilitiesKHR<'static>;
    pub type VkVideoEncodeAV1QuantizationMapCapabilitiesKHR =
        VideoEncodeAV1QuantizationMapCapabilitiesKHR<'static>;
    pub type VkVideoFormatQuantizationMapPropertiesKHR =
        VideoFormatQuantizationMapPropertiesKHR<'static>;
    pub type VkVideoFormatH265QuantizationMapPropertiesKHR =
        VideoFormatH265QuantizationMapPropertiesKHR<'static>;
    pub type VkVideoFormatAV1QuantizationMapPropertiesKHR =
        VideoFormatAV1QuantizationMapPropertiesKHR<'static>;
    pub type VkVideoEncodeQuantizationMapInfoKHR = VideoEncodeQuantizationMapInfoKHR<'static>;
    pub type VkVideoEncodeQuantizationMapSessionParametersCreateInfoKHR =
        VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'static>;
    pub type VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR =
        PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'static>;
    impl VideoEncodeQuantizationMapCapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeQuantizationMapCapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264QuantizationMapCapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoEncodeH264QuantizationMapCapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH265QuantizationMapCapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoEncodeH265QuantizationMapCapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeAV1QuantizationMapCapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoEncodeAV1QuantizationMapCapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoFormatQuantizationMapPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoFormatQuantizationMapPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoFormatH265QuantizationMapPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoFormatH265QuantizationMapPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoFormatAV1QuantizationMapPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoFormatAV1QuantizationMapPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeQuantizationMapInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeQuantizationMapInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoEncodeQuantizationMapSessionParametersCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
