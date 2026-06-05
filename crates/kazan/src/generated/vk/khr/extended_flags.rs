//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_extended_flags.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_extended_flags";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCreateFlags2CreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageCreateFlags2CreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: ImageCreateFlags2KHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageCreateFlags2CreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageCreateFlags2CreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageCreateFlags2CreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_CREATE_FLAGS_2_CREATE_INFO_KHR;
    }

    unsafe impl Extends<ImageCreateInfo<'_>> for ImageCreateFlags2CreateInfoKHR<'_> {}
    unsafe impl Extends<PhysicalDeviceImageFormatInfo2<'_>> for ImageCreateFlags2CreateInfoKHR<'_> {}
    unsafe impl Extends<FramebufferAttachmentImageInfo<'_>> for ImageCreateFlags2CreateInfoKHR<'_> {}
    unsafe impl Extends<VideoFormatPropertiesKHR<'_>> for ImageCreateFlags2CreateInfoKHR<'_> {}

    impl Default for ImageCreateFlags2CreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageCreateFlags2CreateInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: ImageCreateFlags2KHR) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageUsageFlags2CreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageUsageFlags2CreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub usage: ImageUsageFlags2KHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageUsageFlags2CreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageUsageFlags2CreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("usage", &self.usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageUsageFlags2CreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_USAGE_FLAGS_2_CREATE_INFO_KHR;
    }

    unsafe impl Extends<FramebufferAttachmentImageInfo<'_>> for ImageUsageFlags2CreateInfoKHR<'_> {}
    unsafe impl Extends<ImageCreateInfo<'_>> for ImageUsageFlags2CreateInfoKHR<'_> {}
    unsafe impl Extends<PhysicalDeviceImageFormatInfo2<'_>> for ImageUsageFlags2CreateInfoKHR<'_> {}
    unsafe impl Extends<PhysicalDeviceSparseImageFormatInfo2<'_>>
        for ImageUsageFlags2CreateInfoKHR<'_>
    {
    }
    unsafe impl Extends<PhysicalDeviceVideoFormatInfoKHR<'_>> for ImageUsageFlags2CreateInfoKHR<'_> {}
    unsafe impl Extends<SurfaceCapabilities2KHR<'_>> for ImageUsageFlags2CreateInfoKHR<'_> {}
    unsafe impl Extends<SwapchainCreateInfoKHR<'_>> for ImageUsageFlags2CreateInfoKHR<'_> {}
    unsafe impl Extends<VideoFormatPropertiesKHR<'_>> for ImageUsageFlags2CreateInfoKHR<'_> {}

    impl Default for ImageUsageFlags2CreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageUsageFlags2CreateInfoKHR<'a> {
        #[inline]
        pub fn usage(mut self, usage: ImageUsageFlags2KHR) -> Self {
            self.usage = usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSharedPresentSurfaceCapabilities2KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SharedPresentSurfaceCapabilities2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shared_present_supported_usage_flags: ImageUsageFlags2KHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SharedPresentSurfaceCapabilities2KHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SharedPresentSurfaceCapabilities2KHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shared_present_supported_usage_flags",
                    &self.shared_present_supported_usage_flags,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SharedPresentSurfaceCapabilities2KHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SHARED_PRESENT_SURFACE_CAPABILITIES_2_KHR;
    }

    unsafe impl Extends<SurfaceCapabilities2KHR<'_>> for SharedPresentSurfaceCapabilities2KHR<'_> {}

    impl Default for SharedPresentSurfaceCapabilities2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shared_present_supported_usage_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SharedPresentSurfaceCapabilities2KHR<'a> {
        #[inline]
        pub fn shared_present_supported_usage_flags(
            mut self,
            shared_present_supported_usage_flags: ImageUsageFlags2KHR,
        ) -> Self {
            self.shared_present_supported_usage_flags = shared_present_supported_usage_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageViewUsage2CreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageViewUsage2CreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub usage: ImageUsageFlags2KHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageViewUsage2CreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageViewUsage2CreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("usage", &self.usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageViewUsage2CreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_USAGE_2_CREATE_INFO_KHR;
    }

    unsafe impl Extends<ImageViewCreateInfo<'_>> for ImageViewUsage2CreateInfoKHR<'_> {}

    impl Default for ImageViewUsage2CreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageViewUsage2CreateInfoKHR<'a> {
        #[inline]
        pub fn usage(mut self, usage: ImageUsageFlags2KHR) -> Self {
            self.usage = usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageStencilUsage2CreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageStencilUsage2CreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub stencil_usage: ImageUsageFlags2KHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageStencilUsage2CreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageStencilUsage2CreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stencil_usage", &self.stencil_usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageStencilUsage2CreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_STENCIL_USAGE_2_CREATE_INFO_KHR;
    }

    unsafe impl Extends<ImageCreateInfo<'_>> for ImageStencilUsage2CreateInfoKHR<'_> {}
    unsafe impl Extends<PhysicalDeviceImageFormatInfo2<'_>> for ImageStencilUsage2CreateInfoKHR<'_> {}

    impl Default for ImageStencilUsage2CreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                stencil_usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageStencilUsage2CreateInfoKHR<'a> {
        #[inline]
        pub fn stencil_usage(mut self, stencil_usage: ImageUsageFlags2KHR) -> Self {
            self.stencil_usage = stencil_usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExtendedFlagsFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExtendedFlagsFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub extended_flags: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExtendedFlagsFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExtendedFlagsFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("extended_flags", &self.extended_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExtendedFlagsFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTENDED_FLAGS_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceExtendedFlagsFeaturesKHR<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceExtendedFlagsFeaturesKHR<'_> {}

    impl Default for PhysicalDeviceExtendedFlagsFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                extended_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExtendedFlagsFeaturesKHR<'a> {
        #[inline]
        pub fn extended_flags(mut self, extended_flags: bool) -> Self {
            self.extended_flags = extended_flags.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFormatProperties4KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct FormatProperties4KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub linear_tiling_features: FormatFeatureFlags4KHR,
        pub optimal_tiling_features: FormatFeatureFlags4KHR,
        pub buffer_features: FormatFeatureFlags4KHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for FormatProperties4KHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FormatProperties4KHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("linear_tiling_features", &self.linear_tiling_features)
                .field("optimal_tiling_features", &self.optimal_tiling_features)
                .field("buffer_features", &self.buffer_features)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for FormatProperties4KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FORMAT_PROPERTIES_4_KHR;
    }

    unsafe impl Extends<FormatProperties2<'_>> for FormatProperties4KHR<'_> {}

    impl Default for FormatProperties4KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                linear_tiling_features: Default::default(),
                optimal_tiling_features: Default::default(),
                buffer_features: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> FormatProperties4KHR<'a> {
        #[inline]
        pub fn linear_tiling_features(
            mut self,
            linear_tiling_features: FormatFeatureFlags4KHR,
        ) -> Self {
            self.linear_tiling_features = linear_tiling_features;
            self
        }

        #[inline]
        pub fn optimal_tiling_features(
            mut self,
            optimal_tiling_features: FormatFeatureFlags4KHR,
        ) -> Self {
            self.optimal_tiling_features = optimal_tiling_features;
            self
        }

        #[inline]
        pub fn buffer_features(mut self, buffer_features: FormatFeatureFlags4KHR) -> Self {
            self.buffer_features = buffer_features;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFormatFeatureFlags4KHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FormatFeatureFlags4KHR(Flags64);
    vk_bitflags_wrapped!(FormatFeatureFlags4KHR, Flags64, FormatFeatureFlagBits4KHR);

    impl fmt::Debug for FormatFeatureFlags4KHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFormatFeatureFlagBits4KHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct FormatFeatureFlagBits4KHR(u64);

    impl FormatFeatureFlagBits4KHR {}

    impl fmt::Debug for FormatFeatureFlagBits4KHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageUsageFlags2KHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImageUsageFlags2KHR(Flags64);
    vk_bitflags_wrapped!(ImageUsageFlags2KHR, Flags64, ImageUsageFlagBits2KHR);

    impl fmt::Debug for ImageUsageFlags2KHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[
                (
                    ImageUsageFlagBits2KHR::TRANSFER_SRC_KHR.0,
                    "TRANSFER_SRC_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::TRANSFER_DST_KHR.0,
                    "TRANSFER_DST_KHR",
                ),
                (ImageUsageFlagBits2KHR::SAMPLED_KHR.0, "SAMPLED_KHR"),
                (ImageUsageFlagBits2KHR::STORAGE_KHR.0, "STORAGE_KHR"),
                (
                    ImageUsageFlagBits2KHR::COLOR_ATTACHMENT_KHR.0,
                    "COLOR_ATTACHMENT_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::DEPTH_STENCIL_ATTACHMENT_KHR.0,
                    "DEPTH_STENCIL_ATTACHMENT_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::TRANSIENT_ATTACHMENT_KHR.0,
                    "TRANSIENT_ATTACHMENT_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::INPUT_ATTACHMENT_KHR.0,
                    "INPUT_ATTACHMENT_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0,
                    "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::FRAGMENT_DENSITY_MAP_EXT.0,
                    "FRAGMENT_DENSITY_MAP_EXT",
                ),
                (
                    ImageUsageFlagBits2KHR::VIDEO_DECODE_DST_KHR.0,
                    "VIDEO_DECODE_DST_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::VIDEO_DECODE_SRC_KHR.0,
                    "VIDEO_DECODE_SRC_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::VIDEO_DECODE_DPB_KHR.0,
                    "VIDEO_DECODE_DPB_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::VIDEO_ENCODE_DST_KHR.0,
                    "VIDEO_ENCODE_DST_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::VIDEO_ENCODE_SRC_KHR.0,
                    "VIDEO_ENCODE_SRC_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::VIDEO_ENCODE_DPB_KHR.0,
                    "VIDEO_ENCODE_DPB_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::INVOCATION_MASK_HUAWEI.0,
                    "INVOCATION_MASK_HUAWEI",
                ),
                (
                    ImageUsageFlagBits2KHR::ATTACHMENT_FEEDBACK_LOOP_EXT.0,
                    "ATTACHMENT_FEEDBACK_LOOP_EXT",
                ),
                (
                    ImageUsageFlagBits2KHR::SAMPLE_WEIGHT_QCOM.0,
                    "SAMPLE_WEIGHT_QCOM",
                ),
                (
                    ImageUsageFlagBits2KHR::SAMPLE_BLOCK_MATCH_QCOM.0,
                    "SAMPLE_BLOCK_MATCH_QCOM",
                ),
                (
                    ImageUsageFlagBits2KHR::HOST_TRANSFER_KHR.0,
                    "HOST_TRANSFER_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::TENSOR_ALIASING_ARM.0,
                    "TENSOR_ALIASING_ARM",
                ),
                (
                    ImageUsageFlagBits2KHR::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0,
                    "VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::VIDEO_ENCODE_EMPHASIS_MAP_KHR.0,
                    "VIDEO_ENCODE_EMPHASIS_MAP_KHR",
                ),
                (
                    ImageUsageFlagBits2KHR::TILE_MEMORY_QCOM.0,
                    "TILE_MEMORY_QCOM",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageUsageFlagBits2KHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ImageUsageFlagBits2KHR(u64);

    impl ImageUsageFlagBits2KHR {
        pub const TRANSFER_SRC_KHR: Self = Self(1 << 0);
        pub const TRANSFER_DST_KHR: Self = Self(1 << 1);
        pub const SAMPLED_KHR: Self = Self(1 << 2);
        pub const STORAGE_KHR: Self = Self(1 << 3);
        pub const COLOR_ATTACHMENT_KHR: Self = Self(1 << 4);
        pub const DEPTH_STENCIL_ATTACHMENT_KHR: Self = Self(1 << 5);
        pub const TRANSIENT_ATTACHMENT_KHR: Self = Self(1 << 6);
        pub const INPUT_ATTACHMENT_KHR: Self = Self(1 << 7);
        // VK_KHR_extended_flags
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 8);
        pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(1 << 9);
        pub const VIDEO_DECODE_DST_KHR: Self = Self(1 << 10);
        pub const VIDEO_DECODE_SRC_KHR: Self = Self(1 << 11);
        pub const VIDEO_DECODE_DPB_KHR: Self = Self(1 << 12);
        pub const VIDEO_ENCODE_DST_KHR: Self = Self(1 << 13);
        pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1 << 14);
        pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1 << 15);
        pub const INVOCATION_MASK_HUAWEI: Self = Self(1 << 18);
        pub const ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(1 << 19);
        pub const SAMPLE_WEIGHT_QCOM: Self = Self(1 << 20);
        pub const SAMPLE_BLOCK_MATCH_QCOM: Self = Self(1 << 21);
        pub const HOST_TRANSFER_KHR: Self = Self(1 << 22);
        pub const TENSOR_ALIASING_ARM: Self = Self(1 << 23);
        pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 25);
        pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1 << 26);
        pub const TILE_MEMORY_QCOM: Self = Self(1 << 27);
    }

    impl fmt::Debug for ImageUsageFlagBits2KHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TRANSFER_SRC_KHR => Some("TRANSFER_SRC_KHR"),
                Self::TRANSFER_DST_KHR => Some("TRANSFER_DST_KHR"),
                Self::SAMPLED_KHR => Some("SAMPLED_KHR"),
                Self::STORAGE_KHR => Some("STORAGE_KHR"),
                Self::COLOR_ATTACHMENT_KHR => Some("COLOR_ATTACHMENT_KHR"),
                Self::DEPTH_STENCIL_ATTACHMENT_KHR => Some("DEPTH_STENCIL_ATTACHMENT_KHR"),
                Self::TRANSIENT_ATTACHMENT_KHR => Some("TRANSIENT_ATTACHMENT_KHR"),
                Self::INPUT_ATTACHMENT_KHR => Some("INPUT_ATTACHMENT_KHR"),
                Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR => {
                    Some("FRAGMENT_SHADING_RATE_ATTACHMENT_KHR")
                }
                Self::FRAGMENT_DENSITY_MAP_EXT => Some("FRAGMENT_DENSITY_MAP_EXT"),
                Self::VIDEO_DECODE_DST_KHR => Some("VIDEO_DECODE_DST_KHR"),
                Self::VIDEO_DECODE_SRC_KHR => Some("VIDEO_DECODE_SRC_KHR"),
                Self::VIDEO_DECODE_DPB_KHR => Some("VIDEO_DECODE_DPB_KHR"),
                Self::VIDEO_ENCODE_DST_KHR => Some("VIDEO_ENCODE_DST_KHR"),
                Self::VIDEO_ENCODE_SRC_KHR => Some("VIDEO_ENCODE_SRC_KHR"),
                Self::VIDEO_ENCODE_DPB_KHR => Some("VIDEO_ENCODE_DPB_KHR"),
                Self::INVOCATION_MASK_HUAWEI => Some("INVOCATION_MASK_HUAWEI"),
                Self::ATTACHMENT_FEEDBACK_LOOP_EXT => Some("ATTACHMENT_FEEDBACK_LOOP_EXT"),
                Self::SAMPLE_WEIGHT_QCOM => Some("SAMPLE_WEIGHT_QCOM"),
                Self::SAMPLE_BLOCK_MATCH_QCOM => Some("SAMPLE_BLOCK_MATCH_QCOM"),
                Self::HOST_TRANSFER_KHR => Some("HOST_TRANSFER_KHR"),
                Self::TENSOR_ALIASING_ARM => Some("TENSOR_ALIASING_ARM"),
                Self::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR => {
                    Some("VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR")
                }
                Self::VIDEO_ENCODE_EMPHASIS_MAP_KHR => Some("VIDEO_ENCODE_EMPHASIS_MAP_KHR"),
                Self::TILE_MEMORY_QCOM => Some("TILE_MEMORY_QCOM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCreateFlags2KHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImageCreateFlags2KHR(Flags64);
    vk_bitflags_wrapped!(ImageCreateFlags2KHR, Flags64, ImageCreateFlagBits2KHR);

    impl fmt::Debug for ImageCreateFlags2KHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[
                (
                    ImageCreateFlagBits2KHR::SPARSE_BINDING_KHR.0,
                    "SPARSE_BINDING_KHR",
                ),
                (
                    ImageCreateFlagBits2KHR::SPARSE_RESIDENCY_KHR.0,
                    "SPARSE_RESIDENCY_KHR",
                ),
                (
                    ImageCreateFlagBits2KHR::SPARSE_ALIASED_KHR.0,
                    "SPARSE_ALIASED_KHR",
                ),
                (
                    ImageCreateFlagBits2KHR::MUTABLE_FORMAT_KHR.0,
                    "MUTABLE_FORMAT_KHR",
                ),
                (
                    ImageCreateFlagBits2KHR::CUBE_COMPATIBLE_KHR.0,
                    "CUBE_COMPATIBLE_KHR",
                ),
                (
                    ImageCreateFlagBits2KHR::_2D_ARRAY_COMPATIBLE_KHR.0,
                    "_2D_ARRAY_COMPATIBLE_KHR",
                ),
                (
                    ImageCreateFlagBits2KHR::SPLIT_INSTANCE_BIND_REGIONS_KHR.0,
                    "SPLIT_INSTANCE_BIND_REGIONS_KHR",
                ),
                (
                    ImageCreateFlagBits2KHR::BLOCK_TEXEL_VIEW_COMPATIBLE_KHR.0,
                    "BLOCK_TEXEL_VIEW_COMPATIBLE_KHR",
                ),
                (
                    ImageCreateFlagBits2KHR::EXTENDED_USAGE_KHR.0,
                    "EXTENDED_USAGE_KHR",
                ),
                (ImageCreateFlagBits2KHR::DISJOINT_KHR.0, "DISJOINT_KHR"),
                (ImageCreateFlagBits2KHR::ALIAS_KHR.0, "ALIAS_KHR"),
                (ImageCreateFlagBits2KHR::PROTECTED_KHR.0, "PROTECTED_KHR"),
                (
                    ImageCreateFlagBits2KHR::SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT.0,
                    "SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT",
                ),
                (
                    ImageCreateFlagBits2KHR::CORNER_SAMPLED_NV.0,
                    "CORNER_SAMPLED_NV",
                ),
                (ImageCreateFlagBits2KHR::SUBSAMPLED_EXT.0, "SUBSAMPLED_EXT"),
                (
                    ImageCreateFlagBits2KHR::FRAGMENT_DENSITY_MAP_OFFSET_EXT.0,
                    "FRAGMENT_DENSITY_MAP_OFFSET_EXT",
                ),
                (
                    ImageCreateFlagBits2KHR::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0,
                    "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT",
                ),
                (
                    ImageCreateFlagBits2KHR::_2D_VIEW_COMPATIBLE_EXT.0,
                    "_2D_VIEW_COMPATIBLE_EXT",
                ),
                (
                    ImageCreateFlagBits2KHR::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT.0,
                    "MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT",
                ),
                (
                    ImageCreateFlagBits2KHR::VIDEO_PROFILE_INDEPENDENT_KHR.0,
                    "VIDEO_PROFILE_INDEPENDENT_KHR",
                ),
                (
                    ImageCreateFlagBits2KHR::ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR.0,
                    "ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCreateFlagBits2KHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ImageCreateFlagBits2KHR(u64);

    impl ImageCreateFlagBits2KHR {
        pub const SPARSE_BINDING_KHR: Self = Self(1 << 0);
        pub const SPARSE_RESIDENCY_KHR: Self = Self(1 << 1);
        pub const SPARSE_ALIASED_KHR: Self = Self(1 << 2);
        pub const MUTABLE_FORMAT_KHR: Self = Self(1 << 3);
        pub const CUBE_COMPATIBLE_KHR: Self = Self(1 << 4);
        // VK_KHR_extended_flags
        pub const _2D_ARRAY_COMPATIBLE_KHR: Self = Self(1 << 5);
        pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self(1 << 6);
        pub const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR: Self = Self(1 << 7);
        pub const EXTENDED_USAGE_KHR: Self = Self(1 << 8);
        pub const DISJOINT_KHR: Self = Self(1 << 9);
        pub const ALIAS_KHR: Self = Self(1 << 10);
        pub const PROTECTED_KHR: Self = Self(1 << 11);
        pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT: Self = Self(1 << 12);
        pub const CORNER_SAMPLED_NV: Self = Self(1 << 13);
        pub const SUBSAMPLED_EXT: Self = Self(1 << 14);
        pub const FRAGMENT_DENSITY_MAP_OFFSET_EXT: Self = Self(1 << 15);
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(1 << 16);
        pub const _2D_VIEW_COMPATIBLE_EXT: Self = Self(1 << 17);
        pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT: Self = Self(1 << 18);
        pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(1 << 20);

        // VK_KHR_maintenance11
        pub const ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR: Self = Self(1 << 22);
    }

    impl fmt::Debug for ImageCreateFlagBits2KHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SPARSE_BINDING_KHR => Some("SPARSE_BINDING_KHR"),
                Self::SPARSE_RESIDENCY_KHR => Some("SPARSE_RESIDENCY_KHR"),
                Self::SPARSE_ALIASED_KHR => Some("SPARSE_ALIASED_KHR"),
                Self::MUTABLE_FORMAT_KHR => Some("MUTABLE_FORMAT_KHR"),
                Self::CUBE_COMPATIBLE_KHR => Some("CUBE_COMPATIBLE_KHR"),
                Self::_2D_ARRAY_COMPATIBLE_KHR => Some("_2D_ARRAY_COMPATIBLE_KHR"),
                Self::SPLIT_INSTANCE_BIND_REGIONS_KHR => Some("SPLIT_INSTANCE_BIND_REGIONS_KHR"),
                Self::BLOCK_TEXEL_VIEW_COMPATIBLE_KHR => Some("BLOCK_TEXEL_VIEW_COMPATIBLE_KHR"),
                Self::EXTENDED_USAGE_KHR => Some("EXTENDED_USAGE_KHR"),
                Self::DISJOINT_KHR => Some("DISJOINT_KHR"),
                Self::ALIAS_KHR => Some("ALIAS_KHR"),
                Self::PROTECTED_KHR => Some("PROTECTED_KHR"),
                Self::SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT => {
                    Some("SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT")
                }
                Self::CORNER_SAMPLED_NV => Some("CORNER_SAMPLED_NV"),
                Self::SUBSAMPLED_EXT => Some("SUBSAMPLED_EXT"),
                Self::FRAGMENT_DENSITY_MAP_OFFSET_EXT => Some("FRAGMENT_DENSITY_MAP_OFFSET_EXT"),
                Self::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT => {
                    Some("DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT")
                }
                Self::_2D_VIEW_COMPATIBLE_EXT => Some("_2D_VIEW_COMPATIBLE_EXT"),
                Self::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT => {
                    Some("MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT")
                }
                Self::VIDEO_PROFILE_INDEPENDENT_KHR => Some("VIDEO_PROFILE_INDEPENDENT_KHR"),
                Self::ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR => {
                    Some("ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR")
                }
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkImageCreateFlags2CreateInfoKHR = ImageCreateFlags2CreateInfoKHR<'static>;
    pub type VkImageUsageFlags2CreateInfoKHR = ImageUsageFlags2CreateInfoKHR<'static>;
    pub type VkSharedPresentSurfaceCapabilities2KHR = SharedPresentSurfaceCapabilities2KHR<'static>;
    pub type VkImageViewUsage2CreateInfoKHR = ImageViewUsage2CreateInfoKHR<'static>;
    pub type VkImageStencilUsage2CreateInfoKHR = ImageStencilUsage2CreateInfoKHR<'static>;
    pub type VkPhysicalDeviceExtendedFlagsFeaturesKHR =
        PhysicalDeviceExtendedFlagsFeaturesKHR<'static>;
    pub type VkFormatProperties4KHR = FormatProperties4KHR<'static>;
    pub type VkFormatFeatureFlags4KHR = FormatFeatureFlags4KHR;
    pub type VkFormatFeatureFlagBits4KHR = FormatFeatureFlagBits4KHR;
    pub type VkImageUsageFlags2KHR = ImageUsageFlags2KHR;
    pub type VkImageUsageFlagBits2KHR = ImageUsageFlagBits2KHR;
    pub type VkImageCreateFlags2KHR = ImageCreateFlags2KHR;
    pub type VkImageCreateFlagBits2KHR = ImageCreateFlagBits2KHR;
    impl ImageCreateFlags2CreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageCreateFlags2CreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageUsageFlags2CreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageUsageFlags2CreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SharedPresentSurfaceCapabilities2KHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSharedPresentSurfaceCapabilities2KHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageViewUsage2CreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageViewUsage2CreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageStencilUsage2CreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageStencilUsage2CreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceExtendedFlagsFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceExtendedFlagsFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl FormatProperties4KHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkFormatProperties4KHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
