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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_encode_rgb_conversion: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'a>
    {
    }

    impl Default for PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                video_encode_rgb_conversion: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'a> {
        pub fn video_encode_rgb_conversion(mut self, video_encode_rgb_conversion: bool) -> Self {
            self.video_encode_rgb_conversion = video_encode_rgb_conversion.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRgbConversionCapabilitiesVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeRgbConversionCapabilitiesVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub rgb_models: VideoEncodeRgbModelConversionFlagsVALVE,
        pub rgb_ranges: VideoEncodeRgbRangeCompressionFlagsVALVE,
        pub x_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
        pub y_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeRgbConversionCapabilitiesVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>>
        for VideoEncodeRgbConversionCapabilitiesVALVE<'a>
    {
    }

    impl Default for VideoEncodeRgbConversionCapabilitiesVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                rgb_models: Default::default(),
                rgb_ranges: Default::default(),
                x_chroma_offsets: Default::default(),
                y_chroma_offsets: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeRgbConversionCapabilitiesVALVE<'a> {
        pub fn rgb_models(mut self, rgb_models: VideoEncodeRgbModelConversionFlagsVALVE) -> Self {
            self.rgb_models = rgb_models;
            self
        }

        pub fn rgb_ranges(mut self, rgb_ranges: VideoEncodeRgbRangeCompressionFlagsVALVE) -> Self {
            self.rgb_ranges = rgb_ranges;
            self
        }

        pub fn x_chroma_offsets(
            mut self,
            x_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
        ) -> Self {
            self.x_chroma_offsets = x_chroma_offsets;
            self
        }

        pub fn y_chroma_offsets(
            mut self,
            y_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
        ) -> Self {
            self.y_chroma_offsets = y_chroma_offsets;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeProfileRgbConversionInfoVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeProfileRgbConversionInfoVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub perform_encode_rgb_conversion: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeProfileRgbConversionInfoVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE;
    }

    unsafe impl<'a> Extends<VideoProfileInfoKHR<'a>> for VideoEncodeProfileRgbConversionInfoVALVE<'a> {}

    impl Default for VideoEncodeProfileRgbConversionInfoVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                perform_encode_rgb_conversion: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeProfileRgbConversionInfoVALVE<'a> {
        pub fn perform_encode_rgb_conversion(
            mut self,
            perform_encode_rgb_conversion: bool,
        ) -> Self {
            self.perform_encode_rgb_conversion = perform_encode_rgb_conversion.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeSessionRgbConversionCreateInfoVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeSessionRgbConversionCreateInfoVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub rgb_model: VideoEncodeRgbModelConversionFlagBitsVALVE,
        pub rgb_range: VideoEncodeRgbRangeCompressionFlagBitsVALVE,
        pub x_chroma_offset: VideoEncodeRgbChromaOffsetFlagBitsVALVE,
        pub y_chroma_offset: VideoEncodeRgbChromaOffsetFlagBitsVALVE,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeSessionRgbConversionCreateInfoVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE;
    }

    unsafe impl<'a> Extends<VideoSessionCreateInfoKHR<'a>>
        for VideoEncodeSessionRgbConversionCreateInfoVALVE<'a>
    {
    }

    impl Default for VideoEncodeSessionRgbConversionCreateInfoVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                rgb_model: Default::default(),
                rgb_range: Default::default(),
                x_chroma_offset: Default::default(),
                y_chroma_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeSessionRgbConversionCreateInfoVALVE<'a> {
        pub fn rgb_model(mut self, rgb_model: VideoEncodeRgbModelConversionFlagBitsVALVE) -> Self {
            self.rgb_model = rgb_model;
            self
        }

        pub fn rgb_range(mut self, rgb_range: VideoEncodeRgbRangeCompressionFlagBitsVALVE) -> Self {
            self.rgb_range = rgb_range;
            self
        }

        pub fn x_chroma_offset(
            mut self,
            x_chroma_offset: VideoEncodeRgbChromaOffsetFlagBitsVALVE,
        ) -> Self {
            self.x_chroma_offset = x_chroma_offset;
            self
        }

        pub fn y_chroma_offset(
            mut self,
            y_chroma_offset: VideoEncodeRgbChromaOffsetFlagBitsVALVE,
        ) -> Self {
            self.y_chroma_offset = y_chroma_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRgbModelConversionFlagsVALVE.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeRgbModelConversionFlagsVALVE(Flags);
    vk_bitflags_wrapped!(VideoEncodeRgbModelConversionFlagsVALVE, Flags);

    impl VideoEncodeRgbModelConversionFlagsVALVE {
        pub const RGB_IDENTITY_VALVE: Self =
            Self(VideoEncodeRgbModelConversionFlagBitsVALVE::RGB_IDENTITY_VALVE.0);
        pub const YCBCR_IDENTITY_VALVE: Self =
            Self(VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_IDENTITY_VALVE.0);
        pub const YCBCR_709_VALVE: Self =
            Self(VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_709_VALVE.0);
        pub const YCBCR_601_VALVE: Self =
            Self(VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_601_VALVE.0);
        pub const YCBCR_2020_VALVE: Self =
            Self(VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_2020_VALVE.0);
    }

    impl fmt::Debug for VideoEncodeRgbModelConversionFlagsVALVE {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeRgbModelConversionFlagsVALVE::RGB_IDENTITY_VALVE.0,
                    "RGB_IDENTITY_VALVE",
                ),
                (
                    VideoEncodeRgbModelConversionFlagsVALVE::YCBCR_IDENTITY_VALVE.0,
                    "YCBCR_IDENTITY_VALVE",
                ),
                (
                    VideoEncodeRgbModelConversionFlagsVALVE::YCBCR_709_VALVE.0,
                    "YCBCR_709_VALVE",
                ),
                (
                    VideoEncodeRgbModelConversionFlagsVALVE::YCBCR_601_VALVE.0,
                    "YCBCR_601_VALVE",
                ),
                (
                    VideoEncodeRgbModelConversionFlagsVALVE::YCBCR_2020_VALVE.0,
                    "YCBCR_2020_VALVE",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRgbModelConversionFlagBitsVALVE.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeRgbModelConversionFlagBitsVALVE(u32);

    impl VideoEncodeRgbModelConversionFlagBitsVALVE {
        pub const RGB_IDENTITY_VALVE: Self = Self(1 << 0);
        pub const YCBCR_IDENTITY_VALVE: Self = Self(1 << 1);
        pub const YCBCR_709_VALVE: Self = Self(1 << 2);
        pub const YCBCR_601_VALVE: Self = Self(1 << 3);
        pub const YCBCR_2020_VALVE: Self = Self(1 << 4);
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRgbRangeCompressionFlagsVALVE.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeRgbRangeCompressionFlagsVALVE(Flags);
    vk_bitflags_wrapped!(VideoEncodeRgbRangeCompressionFlagsVALVE, Flags);

    impl VideoEncodeRgbRangeCompressionFlagsVALVE {
        pub const FULL_RANGE_VALVE: Self =
            Self(VideoEncodeRgbRangeCompressionFlagBitsVALVE::FULL_RANGE_VALVE.0);
        pub const NARROW_RANGE_VALVE: Self =
            Self(VideoEncodeRgbRangeCompressionFlagBitsVALVE::NARROW_RANGE_VALVE.0);
    }

    impl fmt::Debug for VideoEncodeRgbRangeCompressionFlagsVALVE {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeRgbRangeCompressionFlagsVALVE::FULL_RANGE_VALVE.0,
                    "FULL_RANGE_VALVE",
                ),
                (
                    VideoEncodeRgbRangeCompressionFlagsVALVE::NARROW_RANGE_VALVE.0,
                    "NARROW_RANGE_VALVE",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRgbRangeCompressionFlagBitsVALVE.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeRgbRangeCompressionFlagBitsVALVE(u32);

    impl VideoEncodeRgbRangeCompressionFlagBitsVALVE {
        pub const FULL_RANGE_VALVE: Self = Self(1 << 0);
        pub const NARROW_RANGE_VALVE: Self = Self(1 << 1);
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRgbChromaOffsetFlagsVALVE.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeRgbChromaOffsetFlagsVALVE(Flags);
    vk_bitflags_wrapped!(VideoEncodeRgbChromaOffsetFlagsVALVE, Flags);

    impl VideoEncodeRgbChromaOffsetFlagsVALVE {
        pub const COSITED_EVEN_VALVE: Self =
            Self(VideoEncodeRgbChromaOffsetFlagBitsVALVE::COSITED_EVEN_VALVE.0);
        pub const MIDPOINT_VALVE: Self =
            Self(VideoEncodeRgbChromaOffsetFlagBitsVALVE::MIDPOINT_VALVE.0);
    }

    impl fmt::Debug for VideoEncodeRgbChromaOffsetFlagsVALVE {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeRgbChromaOffsetFlagsVALVE::COSITED_EVEN_VALVE.0,
                    "COSITED_EVEN_VALVE",
                ),
                (
                    VideoEncodeRgbChromaOffsetFlagsVALVE::MIDPOINT_VALVE.0,
                    "MIDPOINT_VALVE",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRgbChromaOffsetFlagBitsVALVE.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeRgbChromaOffsetFlagBitsVALVE(u32);

    impl VideoEncodeRgbChromaOffsetFlagBitsVALVE {
        pub const COSITED_EVEN_VALVE: Self = Self(1 << 0);
        pub const MIDPOINT_VALVE: Self = Self(1 << 1);
    }
}
