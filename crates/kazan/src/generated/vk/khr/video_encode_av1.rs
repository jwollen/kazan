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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1CapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeAV1CapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: VideoEncodeAV1CapabilityFlagsKHR,
        pub max_level: StdVideoAV1Level,
        pub coded_picture_alignment: Extent2D,
        pub max_tiles: Extent2D,
        pub min_tile_size: Extent2D,
        pub max_tile_size: Extent2D,
        pub superblock_sizes: VideoEncodeAV1SuperblockSizeFlagsKHR,
        pub max_single_reference_count: u32,
        pub single_reference_name_mask: u32,
        pub max_unidirectional_compound_reference_count: u32,
        pub max_unidirectional_compound_group1_reference_count: u32,
        pub unidirectional_compound_reference_name_mask: u32,
        pub max_bidirectional_compound_reference_count: u32,
        pub max_bidirectional_compound_group1_reference_count: u32,
        pub max_bidirectional_compound_group2_reference_count: u32,
        pub bidirectional_compound_reference_name_mask: u32,
        pub max_temporal_layer_count: u32,
        pub max_spatial_layer_count: u32,
        pub max_operating_points: u32,
        pub min_q_index: u32,
        pub max_q_index: u32,
        pub prefers_gop_remaining_frames: Bool32,
        pub requires_gop_remaining_frames: Bool32,
        pub std_syntax_flags: VideoEncodeAV1StdFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1CapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_AV1_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>> for VideoEncodeAV1CapabilitiesKHR<'a> {}

    impl Default for VideoEncodeAV1CapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                flags: Default::default(),
                max_level: Default::default(),
                coded_picture_alignment: Default::default(),
                max_tiles: Default::default(),
                min_tile_size: Default::default(),
                max_tile_size: Default::default(),
                superblock_sizes: Default::default(),
                max_single_reference_count: Default::default(),
                single_reference_name_mask: Default::default(),
                max_unidirectional_compound_reference_count: Default::default(),
                max_unidirectional_compound_group1_reference_count: Default::default(),
                unidirectional_compound_reference_name_mask: Default::default(),
                max_bidirectional_compound_reference_count: Default::default(),
                max_bidirectional_compound_group1_reference_count: Default::default(),
                max_bidirectional_compound_group2_reference_count: Default::default(),
                bidirectional_compound_reference_name_mask: Default::default(),
                max_temporal_layer_count: Default::default(),
                max_spatial_layer_count: Default::default(),
                max_operating_points: Default::default(),
                min_q_index: Default::default(),
                max_q_index: Default::default(),
                prefers_gop_remaining_frames: Default::default(),
                requires_gop_remaining_frames: Default::default(),
                std_syntax_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1CapabilitiesKHR<'a> {
        pub fn flags(mut self, flags: VideoEncodeAV1CapabilityFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        pub fn max_level(mut self, max_level: StdVideoAV1Level) -> Self {
            self.max_level = max_level;
            self
        }

        pub fn coded_picture_alignment(mut self, coded_picture_alignment: Extent2D) -> Self {
            self.coded_picture_alignment = coded_picture_alignment;
            self
        }

        pub fn max_tiles(mut self, max_tiles: Extent2D) -> Self {
            self.max_tiles = max_tiles;
            self
        }

        pub fn min_tile_size(mut self, min_tile_size: Extent2D) -> Self {
            self.min_tile_size = min_tile_size;
            self
        }

        pub fn max_tile_size(mut self, max_tile_size: Extent2D) -> Self {
            self.max_tile_size = max_tile_size;
            self
        }

        pub fn superblock_sizes(
            mut self,
            superblock_sizes: VideoEncodeAV1SuperblockSizeFlagsKHR,
        ) -> Self {
            self.superblock_sizes = superblock_sizes;
            self
        }

        pub fn max_single_reference_count(mut self, max_single_reference_count: u32) -> Self {
            self.max_single_reference_count = max_single_reference_count;
            self
        }

        pub fn single_reference_name_mask(mut self, single_reference_name_mask: u32) -> Self {
            self.single_reference_name_mask = single_reference_name_mask;
            self
        }

        pub fn max_unidirectional_compound_reference_count(
            mut self,
            max_unidirectional_compound_reference_count: u32,
        ) -> Self {
            self.max_unidirectional_compound_reference_count =
                max_unidirectional_compound_reference_count;
            self
        }

        pub fn max_unidirectional_compound_group1_reference_count(
            mut self,
            max_unidirectional_compound_group1_reference_count: u32,
        ) -> Self {
            self.max_unidirectional_compound_group1_reference_count =
                max_unidirectional_compound_group1_reference_count;
            self
        }

        pub fn unidirectional_compound_reference_name_mask(
            mut self,
            unidirectional_compound_reference_name_mask: u32,
        ) -> Self {
            self.unidirectional_compound_reference_name_mask =
                unidirectional_compound_reference_name_mask;
            self
        }

        pub fn max_bidirectional_compound_reference_count(
            mut self,
            max_bidirectional_compound_reference_count: u32,
        ) -> Self {
            self.max_bidirectional_compound_reference_count =
                max_bidirectional_compound_reference_count;
            self
        }

        pub fn max_bidirectional_compound_group1_reference_count(
            mut self,
            max_bidirectional_compound_group1_reference_count: u32,
        ) -> Self {
            self.max_bidirectional_compound_group1_reference_count =
                max_bidirectional_compound_group1_reference_count;
            self
        }

        pub fn max_bidirectional_compound_group2_reference_count(
            mut self,
            max_bidirectional_compound_group2_reference_count: u32,
        ) -> Self {
            self.max_bidirectional_compound_group2_reference_count =
                max_bidirectional_compound_group2_reference_count;
            self
        }

        pub fn bidirectional_compound_reference_name_mask(
            mut self,
            bidirectional_compound_reference_name_mask: u32,
        ) -> Self {
            self.bidirectional_compound_reference_name_mask =
                bidirectional_compound_reference_name_mask;
            self
        }

        pub fn max_temporal_layer_count(mut self, max_temporal_layer_count: u32) -> Self {
            self.max_temporal_layer_count = max_temporal_layer_count;
            self
        }

        pub fn max_spatial_layer_count(mut self, max_spatial_layer_count: u32) -> Self {
            self.max_spatial_layer_count = max_spatial_layer_count;
            self
        }

        pub fn max_operating_points(mut self, max_operating_points: u32) -> Self {
            self.max_operating_points = max_operating_points;
            self
        }

        pub fn min_q_index(mut self, min_q_index: u32) -> Self {
            self.min_q_index = min_q_index;
            self
        }

        pub fn max_q_index(mut self, max_q_index: u32) -> Self {
            self.max_q_index = max_q_index;
            self
        }

        pub fn prefers_gop_remaining_frames(mut self, prefers_gop_remaining_frames: bool) -> Self {
            self.prefers_gop_remaining_frames = prefers_gop_remaining_frames.into();
            self
        }

        pub fn requires_gop_remaining_frames(
            mut self,
            requires_gop_remaining_frames: bool,
        ) -> Self {
            self.requires_gop_remaining_frames = requires_gop_remaining_frames.into();
            self
        }

        pub fn std_syntax_flags(mut self, std_syntax_flags: VideoEncodeAV1StdFlagsKHR) -> Self {
            self.std_syntax_flags = std_syntax_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1QualityLevelPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeAV1QualityLevelPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub preferred_rate_control_flags: VideoEncodeAV1RateControlFlagsKHR,
        pub preferred_gop_frame_count: u32,
        pub preferred_key_frame_period: u32,
        pub preferred_consecutive_bipredictive_frame_count: u32,
        pub preferred_temporal_layer_count: u32,
        pub preferred_constant_q_index: VideoEncodeAV1QIndexKHR,
        pub preferred_max_single_reference_count: u32,
        pub preferred_single_reference_name_mask: u32,
        pub preferred_max_unidirectional_compound_reference_count: u32,
        pub preferred_max_unidirectional_compound_group1_reference_count: u32,
        pub preferred_unidirectional_compound_reference_name_mask: u32,
        pub preferred_max_bidirectional_compound_reference_count: u32,
        pub preferred_max_bidirectional_compound_group1_reference_count: u32,
        pub preferred_max_bidirectional_compound_group2_reference_count: u32,
        pub preferred_bidirectional_compound_reference_name_mask: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1QualityLevelPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<VideoEncodeQualityLevelPropertiesKHR<'a>>
        for VideoEncodeAV1QualityLevelPropertiesKHR<'a>
    {
    }

    impl Default for VideoEncodeAV1QualityLevelPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                preferred_rate_control_flags: Default::default(),
                preferred_gop_frame_count: Default::default(),
                preferred_key_frame_period: Default::default(),
                preferred_consecutive_bipredictive_frame_count: Default::default(),
                preferred_temporal_layer_count: Default::default(),
                preferred_constant_q_index: Default::default(),
                preferred_max_single_reference_count: Default::default(),
                preferred_single_reference_name_mask: Default::default(),
                preferred_max_unidirectional_compound_reference_count: Default::default(),
                preferred_max_unidirectional_compound_group1_reference_count: Default::default(),
                preferred_unidirectional_compound_reference_name_mask: Default::default(),
                preferred_max_bidirectional_compound_reference_count: Default::default(),
                preferred_max_bidirectional_compound_group1_reference_count: Default::default(),
                preferred_max_bidirectional_compound_group2_reference_count: Default::default(),
                preferred_bidirectional_compound_reference_name_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1QualityLevelPropertiesKHR<'a> {
        pub fn preferred_rate_control_flags(
            mut self,
            preferred_rate_control_flags: VideoEncodeAV1RateControlFlagsKHR,
        ) -> Self {
            self.preferred_rate_control_flags = preferred_rate_control_flags;
            self
        }

        pub fn preferred_gop_frame_count(mut self, preferred_gop_frame_count: u32) -> Self {
            self.preferred_gop_frame_count = preferred_gop_frame_count;
            self
        }

        pub fn preferred_key_frame_period(mut self, preferred_key_frame_period: u32) -> Self {
            self.preferred_key_frame_period = preferred_key_frame_period;
            self
        }

        pub fn preferred_consecutive_bipredictive_frame_count(
            mut self,
            preferred_consecutive_bipredictive_frame_count: u32,
        ) -> Self {
            self.preferred_consecutive_bipredictive_frame_count =
                preferred_consecutive_bipredictive_frame_count;
            self
        }

        pub fn preferred_temporal_layer_count(
            mut self,
            preferred_temporal_layer_count: u32,
        ) -> Self {
            self.preferred_temporal_layer_count = preferred_temporal_layer_count;
            self
        }

        pub fn preferred_constant_q_index(
            mut self,
            preferred_constant_q_index: VideoEncodeAV1QIndexKHR,
        ) -> Self {
            self.preferred_constant_q_index = preferred_constant_q_index;
            self
        }

        pub fn preferred_max_single_reference_count(
            mut self,
            preferred_max_single_reference_count: u32,
        ) -> Self {
            self.preferred_max_single_reference_count = preferred_max_single_reference_count;
            self
        }

        pub fn preferred_single_reference_name_mask(
            mut self,
            preferred_single_reference_name_mask: u32,
        ) -> Self {
            self.preferred_single_reference_name_mask = preferred_single_reference_name_mask;
            self
        }

        pub fn preferred_max_unidirectional_compound_reference_count(
            mut self,
            preferred_max_unidirectional_compound_reference_count: u32,
        ) -> Self {
            self.preferred_max_unidirectional_compound_reference_count =
                preferred_max_unidirectional_compound_reference_count;
            self
        }

        pub fn preferred_max_unidirectional_compound_group1_reference_count(
            mut self,
            preferred_max_unidirectional_compound_group1_reference_count: u32,
        ) -> Self {
            self.preferred_max_unidirectional_compound_group1_reference_count =
                preferred_max_unidirectional_compound_group1_reference_count;
            self
        }

        pub fn preferred_unidirectional_compound_reference_name_mask(
            mut self,
            preferred_unidirectional_compound_reference_name_mask: u32,
        ) -> Self {
            self.preferred_unidirectional_compound_reference_name_mask =
                preferred_unidirectional_compound_reference_name_mask;
            self
        }

        pub fn preferred_max_bidirectional_compound_reference_count(
            mut self,
            preferred_max_bidirectional_compound_reference_count: u32,
        ) -> Self {
            self.preferred_max_bidirectional_compound_reference_count =
                preferred_max_bidirectional_compound_reference_count;
            self
        }

        pub fn preferred_max_bidirectional_compound_group1_reference_count(
            mut self,
            preferred_max_bidirectional_compound_group1_reference_count: u32,
        ) -> Self {
            self.preferred_max_bidirectional_compound_group1_reference_count =
                preferred_max_bidirectional_compound_group1_reference_count;
            self
        }

        pub fn preferred_max_bidirectional_compound_group2_reference_count(
            mut self,
            preferred_max_bidirectional_compound_group2_reference_count: u32,
        ) -> Self {
            self.preferred_max_bidirectional_compound_group2_reference_count =
                preferred_max_bidirectional_compound_group2_reference_count;
            self
        }

        pub fn preferred_bidirectional_compound_reference_name_mask(
            mut self,
            preferred_bidirectional_compound_reference_name_mask: u32,
        ) -> Self {
            self.preferred_bidirectional_compound_reference_name_mask =
                preferred_bidirectional_compound_reference_name_mask;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVideoEncodeAV1FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVideoEncodeAV1FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_encode_av1: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoEncodeAV1FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVideoEncodeAV1FeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVideoEncodeAV1FeaturesKHR<'a> {}

    impl Default for PhysicalDeviceVideoEncodeAV1FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                video_encode_av1: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVideoEncodeAV1FeaturesKHR<'a> {
        pub fn video_encode_av1(mut self, video_encode_av1: bool) -> Self {
            self.video_encode_av1 = video_encode_av1.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1SessionCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeAV1SessionCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub use_max_level: Bool32,
        pub max_level: StdVideoAV1Level,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1SessionCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionCreateInfoKHR<'a>> for VideoEncodeAV1SessionCreateInfoKHR<'a> {}

    impl Default for VideoEncodeAV1SessionCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                use_max_level: Default::default(),
                max_level: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1SessionCreateInfoKHR<'a> {
        pub fn use_max_level(mut self, use_max_level: bool) -> Self {
            self.use_max_level = use_max_level.into();
            self
        }

        pub fn max_level(mut self, max_level: StdVideoAV1Level) -> Self {
            self.max_level = max_level;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1SessionParametersCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeAV1SessionParametersCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_sequence_header: *const StdVideoAV1SequenceHeader<'a>,
        pub p_std_decoder_model_info: *const StdVideoEncodeAV1DecoderModelInfo,
        pub std_operating_point_count: u32,
        pub p_std_operating_points: *const StdVideoEncodeAV1OperatingPointInfo,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1SessionParametersCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionParametersCreateInfoKHR<'a>>
        for VideoEncodeAV1SessionParametersCreateInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeAV1SessionParametersCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_sequence_header: core::ptr::null(),
                p_std_decoder_model_info: core::ptr::null(),
                std_operating_point_count: Default::default(),
                p_std_operating_points: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1SessionParametersCreateInfoKHR<'a> {
        pub fn std_sequence_header(
            mut self,
            std_sequence_header: &'a StdVideoAV1SequenceHeader<'a>,
        ) -> Self {
            self.p_std_sequence_header = std_sequence_header;
            self
        }

        pub fn std_decoder_model_info(
            mut self,
            std_decoder_model_info: &'a StdVideoEncodeAV1DecoderModelInfo,
        ) -> Self {
            self.p_std_decoder_model_info = std_decoder_model_info;
            self
        }

        pub fn std_operating_points(
            mut self,
            std_operating_points: &'a [StdVideoEncodeAV1OperatingPointInfo],
        ) -> Self {
            self.std_operating_point_count = std_operating_points.len().try_into().unwrap();
            self.p_std_operating_points = std_operating_points.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1DpbSlotInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeAV1DpbSlotInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_reference_info: *const StdVideoEncodeAV1ReferenceInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1DpbSlotInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoReferenceSlotInfoKHR<'a>> for VideoEncodeAV1DpbSlotInfoKHR<'a> {}

    impl Default for VideoEncodeAV1DpbSlotInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_reference_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1DpbSlotInfoKHR<'a> {
        pub fn std_reference_info(
            mut self,
            std_reference_info: &'a StdVideoEncodeAV1ReferenceInfo<'a>,
        ) -> Self {
            self.p_std_reference_info = std_reference_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1PictureInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeAV1PictureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub prediction_mode: VideoEncodeAV1PredictionModeKHR,
        pub rate_control_group: VideoEncodeAV1RateControlGroupKHR,
        pub constant_q_index: u32,
        pub p_std_picture_info: *const StdVideoEncodeAV1PictureInfo<'a>,
        pub reference_name_slot_indices: [i32; MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
        pub primary_reference_cdf_only: Bool32,
        pub generate_obu_extension_header: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1PictureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_AV1_PICTURE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoEncodeInfoKHR<'a>> for VideoEncodeAV1PictureInfoKHR<'a> {}

    impl Default for VideoEncodeAV1PictureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                prediction_mode: Default::default(),
                rate_control_group: Default::default(),
                constant_q_index: Default::default(),
                p_std_picture_info: core::ptr::null(),
                reference_name_slot_indices: [Default::default(); _],
                primary_reference_cdf_only: Default::default(),
                generate_obu_extension_header: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1PictureInfoKHR<'a> {
        pub fn prediction_mode(mut self, prediction_mode: VideoEncodeAV1PredictionModeKHR) -> Self {
            self.prediction_mode = prediction_mode;
            self
        }

        pub fn rate_control_group(
            mut self,
            rate_control_group: VideoEncodeAV1RateControlGroupKHR,
        ) -> Self {
            self.rate_control_group = rate_control_group;
            self
        }

        pub fn constant_q_index(mut self, constant_q_index: u32) -> Self {
            self.constant_q_index = constant_q_index;
            self
        }

        pub fn std_picture_info(
            mut self,
            std_picture_info: &'a StdVideoEncodeAV1PictureInfo<'a>,
        ) -> Self {
            self.p_std_picture_info = std_picture_info;
            self
        }

        pub fn reference_name_slot_indices(
            mut self,
            reference_name_slot_indices: [i32; MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
        ) -> Self {
            self.reference_name_slot_indices = reference_name_slot_indices;
            self
        }

        pub fn primary_reference_cdf_only(mut self, primary_reference_cdf_only: bool) -> Self {
            self.primary_reference_cdf_only = primary_reference_cdf_only.into();
            self
        }

        pub fn generate_obu_extension_header(
            mut self,
            generate_obu_extension_header: bool,
        ) -> Self {
            self.generate_obu_extension_header = generate_obu_extension_header.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1ProfileInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeAV1ProfileInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_profile: StdVideoAV1Profile,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1ProfileInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_AV1_PROFILE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoProfileInfoKHR<'a>> for VideoEncodeAV1ProfileInfoKHR<'a> {}
    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for VideoEncodeAV1ProfileInfoKHR<'a> {}

    impl Default for VideoEncodeAV1ProfileInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                std_profile: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1ProfileInfoKHR<'a> {
        pub fn std_profile(mut self, std_profile: StdVideoAV1Profile) -> Self {
            self.std_profile = std_profile;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1RateControlInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeAV1RateControlInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoEncodeAV1RateControlFlagsKHR,
        pub gop_frame_count: u32,
        pub key_frame_period: u32,
        pub consecutive_bipredictive_frame_count: u32,
        pub temporal_layer_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1RateControlInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoCodingControlInfoKHR<'a>> for VideoEncodeAV1RateControlInfoKHR<'a> {}
    unsafe impl<'a> Extends<VideoBeginCodingInfoKHR<'a>> for VideoEncodeAV1RateControlInfoKHR<'a> {}

    impl Default for VideoEncodeAV1RateControlInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                gop_frame_count: Default::default(),
                key_frame_period: Default::default(),
                consecutive_bipredictive_frame_count: Default::default(),
                temporal_layer_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1RateControlInfoKHR<'a> {
        pub fn flags(mut self, flags: VideoEncodeAV1RateControlFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        pub fn gop_frame_count(mut self, gop_frame_count: u32) -> Self {
            self.gop_frame_count = gop_frame_count;
            self
        }

        pub fn key_frame_period(mut self, key_frame_period: u32) -> Self {
            self.key_frame_period = key_frame_period;
            self
        }

        pub fn consecutive_bipredictive_frame_count(
            mut self,
            consecutive_bipredictive_frame_count: u32,
        ) -> Self {
            self.consecutive_bipredictive_frame_count = consecutive_bipredictive_frame_count;
            self
        }

        pub fn temporal_layer_count(mut self, temporal_layer_count: u32) -> Self {
            self.temporal_layer_count = temporal_layer_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1QIndexKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct VideoEncodeAV1QIndexKHR {
        pub intra_q_index: u32,
        pub predictive_q_index: u32,
        pub bipredictive_q_index: u32,
    }

    impl VideoEncodeAV1QIndexKHR {
        pub fn intra_q_index(mut self, intra_q_index: u32) -> Self {
            self.intra_q_index = intra_q_index;
            self
        }

        pub fn predictive_q_index(mut self, predictive_q_index: u32) -> Self {
            self.predictive_q_index = predictive_q_index;
            self
        }

        pub fn bipredictive_q_index(mut self, bipredictive_q_index: u32) -> Self {
            self.bipredictive_q_index = bipredictive_q_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1FrameSizeKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct VideoEncodeAV1FrameSizeKHR {
        pub intra_frame_size: u32,
        pub predictive_frame_size: u32,
        pub bipredictive_frame_size: u32,
    }

    impl VideoEncodeAV1FrameSizeKHR {
        pub fn intra_frame_size(mut self, intra_frame_size: u32) -> Self {
            self.intra_frame_size = intra_frame_size;
            self
        }

        pub fn predictive_frame_size(mut self, predictive_frame_size: u32) -> Self {
            self.predictive_frame_size = predictive_frame_size;
            self
        }

        pub fn bipredictive_frame_size(mut self, bipredictive_frame_size: u32) -> Self {
            self.bipredictive_frame_size = bipredictive_frame_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1GopRemainingFrameInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeAV1GopRemainingFrameInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub use_gop_remaining_frames: Bool32,
        pub gop_remaining_intra: u32,
        pub gop_remaining_predictive: u32,
        pub gop_remaining_bipredictive: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1GopRemainingFrameInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoBeginCodingInfoKHR<'a>>
        for VideoEncodeAV1GopRemainingFrameInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeAV1GopRemainingFrameInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                use_gop_remaining_frames: Default::default(),
                gop_remaining_intra: Default::default(),
                gop_remaining_predictive: Default::default(),
                gop_remaining_bipredictive: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1GopRemainingFrameInfoKHR<'a> {
        pub fn use_gop_remaining_frames(mut self, use_gop_remaining_frames: bool) -> Self {
            self.use_gop_remaining_frames = use_gop_remaining_frames.into();
            self
        }

        pub fn gop_remaining_intra(mut self, gop_remaining_intra: u32) -> Self {
            self.gop_remaining_intra = gop_remaining_intra;
            self
        }

        pub fn gop_remaining_predictive(mut self, gop_remaining_predictive: u32) -> Self {
            self.gop_remaining_predictive = gop_remaining_predictive;
            self
        }

        pub fn gop_remaining_bipredictive(mut self, gop_remaining_bipredictive: u32) -> Self {
            self.gop_remaining_bipredictive = gop_remaining_bipredictive;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1RateControlLayerInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeAV1RateControlLayerInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub use_min_q_index: Bool32,
        pub min_q_index: VideoEncodeAV1QIndexKHR,
        pub use_max_q_index: Bool32,
        pub max_q_index: VideoEncodeAV1QIndexKHR,
        pub use_max_frame_size: Bool32,
        pub max_frame_size: VideoEncodeAV1FrameSizeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeAV1RateControlLayerInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoEncodeRateControlLayerInfoKHR<'a>>
        for VideoEncodeAV1RateControlLayerInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeAV1RateControlLayerInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                use_min_q_index: Default::default(),
                min_q_index: Default::default(),
                use_max_q_index: Default::default(),
                max_q_index: Default::default(),
                use_max_frame_size: Default::default(),
                max_frame_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeAV1RateControlLayerInfoKHR<'a> {
        pub fn use_min_q_index(mut self, use_min_q_index: bool) -> Self {
            self.use_min_q_index = use_min_q_index.into();
            self
        }

        pub fn min_q_index(mut self, min_q_index: VideoEncodeAV1QIndexKHR) -> Self {
            self.min_q_index = min_q_index;
            self
        }

        pub fn use_max_q_index(mut self, use_max_q_index: bool) -> Self {
            self.use_max_q_index = use_max_q_index.into();
            self
        }

        pub fn max_q_index(mut self, max_q_index: VideoEncodeAV1QIndexKHR) -> Self {
            self.max_q_index = max_q_index;
            self
        }

        pub fn use_max_frame_size(mut self, use_max_frame_size: bool) -> Self {
            self.use_max_frame_size = use_max_frame_size.into();
            self
        }

        pub fn max_frame_size(mut self, max_frame_size: VideoEncodeAV1FrameSizeKHR) -> Self {
            self.max_frame_size = max_frame_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1PredictionModeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeAV1PredictionModeKHR(i32);

    impl VideoEncodeAV1PredictionModeKHR {
        pub const INTRA_ONLY_KHR: Self = Self(0);
        pub const SINGLE_REFERENCE_KHR: Self = Self(1);
        pub const UNIDIRECTIONAL_COMPOUND_KHR: Self = Self(2);
        pub const BIDIRECTIONAL_COMPOUND_KHR: Self = Self(3);
    }

    impl fmt::Debug for VideoEncodeAV1PredictionModeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INTRA_ONLY_KHR => Some("INTRA_ONLY_KHR"),
                Self::SINGLE_REFERENCE_KHR => Some("SINGLE_REFERENCE_KHR"),
                Self::UNIDIRECTIONAL_COMPOUND_KHR => Some("UNIDIRECTIONAL_COMPOUND_KHR"),
                Self::BIDIRECTIONAL_COMPOUND_KHR => Some("BIDIRECTIONAL_COMPOUND_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1RateControlGroupKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeAV1RateControlGroupKHR(i32);

    impl VideoEncodeAV1RateControlGroupKHR {
        pub const INTRA_KHR: Self = Self(0);
        pub const PREDICTIVE_KHR: Self = Self(1);
        pub const BIPREDICTIVE_KHR: Self = Self(2);
    }

    impl fmt::Debug for VideoEncodeAV1RateControlGroupKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INTRA_KHR => Some("INTRA_KHR"),
                Self::PREDICTIVE_KHR => Some("PREDICTIVE_KHR"),
                Self::BIPREDICTIVE_KHR => Some("BIPREDICTIVE_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1CapabilityFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeAV1CapabilityFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeAV1CapabilityFlagsKHR, Flags);

    impl VideoEncodeAV1CapabilityFlagsKHR {
        pub const PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR: Self =
            Self(VideoEncodeAV1CapabilityFlagBitsKHR::PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR.0);
        pub const GENERATE_OBU_EXTENSION_HEADER_KHR: Self =
            Self(VideoEncodeAV1CapabilityFlagBitsKHR::GENERATE_OBU_EXTENSION_HEADER_KHR.0);
        pub const PRIMARY_REFERENCE_CDF_ONLY_KHR: Self =
            Self(VideoEncodeAV1CapabilityFlagBitsKHR::PRIMARY_REFERENCE_CDF_ONLY_KHR.0);
        pub const FRAME_SIZE_OVERRIDE_KHR: Self =
            Self(VideoEncodeAV1CapabilityFlagBitsKHR::FRAME_SIZE_OVERRIDE_KHR.0);
        pub const MOTION_VECTOR_SCALING_KHR: Self =
            Self(VideoEncodeAV1CapabilityFlagBitsKHR::MOTION_VECTOR_SCALING_KHR.0);
        // VK_KHR_video_encode_intra_refresh
        pub const COMPOUND_PREDICTION_INTRA_REFRESH_KHR: Self =
            Self(VideoEncodeAV1CapabilityFlagBitsKHR::COMPOUND_PREDICTION_INTRA_REFRESH_KHR.0);
    }

    impl fmt::Debug for VideoEncodeAV1CapabilityFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeAV1CapabilityFlagsKHR::PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR.0,
                    "PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR",
                ),
                (
                    VideoEncodeAV1CapabilityFlagsKHR::GENERATE_OBU_EXTENSION_HEADER_KHR.0,
                    "GENERATE_OBU_EXTENSION_HEADER_KHR",
                ),
                (
                    VideoEncodeAV1CapabilityFlagsKHR::PRIMARY_REFERENCE_CDF_ONLY_KHR.0,
                    "PRIMARY_REFERENCE_CDF_ONLY_KHR",
                ),
                (
                    VideoEncodeAV1CapabilityFlagsKHR::FRAME_SIZE_OVERRIDE_KHR.0,
                    "FRAME_SIZE_OVERRIDE_KHR",
                ),
                (
                    VideoEncodeAV1CapabilityFlagsKHR::MOTION_VECTOR_SCALING_KHR.0,
                    "MOTION_VECTOR_SCALING_KHR",
                ),
                (
                    VideoEncodeAV1CapabilityFlagsKHR::COMPOUND_PREDICTION_INTRA_REFRESH_KHR.0,
                    "COMPOUND_PREDICTION_INTRA_REFRESH_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1CapabilityFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeAV1CapabilityFlagBitsKHR(u32);

    impl VideoEncodeAV1CapabilityFlagBitsKHR {
        pub const PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR: Self = Self(1 << 0);
        pub const GENERATE_OBU_EXTENSION_HEADER_KHR: Self = Self(1 << 1);
        pub const PRIMARY_REFERENCE_CDF_ONLY_KHR: Self = Self(1 << 2);
        pub const FRAME_SIZE_OVERRIDE_KHR: Self = Self(1 << 3);
        pub const MOTION_VECTOR_SCALING_KHR: Self = Self(1 << 4);
        // VK_KHR_video_encode_intra_refresh
        pub const COMPOUND_PREDICTION_INTRA_REFRESH_KHR: Self = Self(1 << 5);
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1StdFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeAV1StdFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeAV1StdFlagsKHR, Flags);

    impl VideoEncodeAV1StdFlagsKHR {
        pub const UNIFORM_TILE_SPACING_FLAG_SET_KHR: Self =
            Self(VideoEncodeAV1StdFlagBitsKHR::UNIFORM_TILE_SPACING_FLAG_SET_KHR.0);
        pub const SKIP_MODE_PRESENT_UNSET_KHR: Self =
            Self(VideoEncodeAV1StdFlagBitsKHR::SKIP_MODE_PRESENT_UNSET_KHR.0);
        pub const PRIMARY_REF_FRAME_KHR: Self =
            Self(VideoEncodeAV1StdFlagBitsKHR::PRIMARY_REF_FRAME_KHR.0);
        pub const DELTA_Q_KHR: Self = Self(VideoEncodeAV1StdFlagBitsKHR::DELTA_Q_KHR.0);
    }

    impl fmt::Debug for VideoEncodeAV1StdFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeAV1StdFlagsKHR::UNIFORM_TILE_SPACING_FLAG_SET_KHR.0,
                    "UNIFORM_TILE_SPACING_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeAV1StdFlagsKHR::SKIP_MODE_PRESENT_UNSET_KHR.0,
                    "SKIP_MODE_PRESENT_UNSET_KHR",
                ),
                (
                    VideoEncodeAV1StdFlagsKHR::PRIMARY_REF_FRAME_KHR.0,
                    "PRIMARY_REF_FRAME_KHR",
                ),
                (VideoEncodeAV1StdFlagsKHR::DELTA_Q_KHR.0, "DELTA_Q_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1StdFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeAV1StdFlagBitsKHR(u32);

    impl VideoEncodeAV1StdFlagBitsKHR {
        pub const UNIFORM_TILE_SPACING_FLAG_SET_KHR: Self = Self(1 << 0);
        pub const SKIP_MODE_PRESENT_UNSET_KHR: Self = Self(1 << 1);
        pub const PRIMARY_REF_FRAME_KHR: Self = Self(1 << 2);
        pub const DELTA_Q_KHR: Self = Self(1 << 3);
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1RateControlFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeAV1RateControlFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeAV1RateControlFlagsKHR, Flags);

    impl VideoEncodeAV1RateControlFlagsKHR {
        pub const REGULAR_GOP_KHR: Self =
            Self(VideoEncodeAV1RateControlFlagBitsKHR::REGULAR_GOP_KHR.0);
        pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self =
            Self(VideoEncodeAV1RateControlFlagBitsKHR::TEMPORAL_LAYER_PATTERN_DYADIC_KHR.0);
        pub const REFERENCE_PATTERN_FLAT_KHR: Self =
            Self(VideoEncodeAV1RateControlFlagBitsKHR::REFERENCE_PATTERN_FLAT_KHR.0);
        pub const REFERENCE_PATTERN_DYADIC_KHR: Self =
            Self(VideoEncodeAV1RateControlFlagBitsKHR::REFERENCE_PATTERN_DYADIC_KHR.0);
    }

    impl fmt::Debug for VideoEncodeAV1RateControlFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeAV1RateControlFlagsKHR::REGULAR_GOP_KHR.0,
                    "REGULAR_GOP_KHR",
                ),
                (
                    VideoEncodeAV1RateControlFlagsKHR::TEMPORAL_LAYER_PATTERN_DYADIC_KHR.0,
                    "TEMPORAL_LAYER_PATTERN_DYADIC_KHR",
                ),
                (
                    VideoEncodeAV1RateControlFlagsKHR::REFERENCE_PATTERN_FLAT_KHR.0,
                    "REFERENCE_PATTERN_FLAT_KHR",
                ),
                (
                    VideoEncodeAV1RateControlFlagsKHR::REFERENCE_PATTERN_DYADIC_KHR.0,
                    "REFERENCE_PATTERN_DYADIC_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1RateControlFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeAV1RateControlFlagBitsKHR(u32);

    impl VideoEncodeAV1RateControlFlagBitsKHR {
        pub const REGULAR_GOP_KHR: Self = Self(1 << 0);
        pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(1 << 1);
        pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(1 << 2);
        pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(1 << 3);
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1SuperblockSizeFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeAV1SuperblockSizeFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeAV1SuperblockSizeFlagsKHR, Flags);

    impl VideoEncodeAV1SuperblockSizeFlagsKHR {
        pub const _64_KHR: Self = Self(VideoEncodeAV1SuperblockSizeFlagBitsKHR::_64_KHR.0);
        pub const _128_KHR: Self = Self(VideoEncodeAV1SuperblockSizeFlagBitsKHR::_128_KHR.0);
    }

    impl fmt::Debug for VideoEncodeAV1SuperblockSizeFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (VideoEncodeAV1SuperblockSizeFlagsKHR::_64_KHR.0, "_64_KHR"),
                (VideoEncodeAV1SuperblockSizeFlagsKHR::_128_KHR.0, "_128_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeAV1SuperblockSizeFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeAV1SuperblockSizeFlagBitsKHR(u32);

    impl VideoEncodeAV1SuperblockSizeFlagBitsKHR {
        pub const _64_KHR: Self = Self(1 << 0);
        pub const _128_KHR: Self = Self(1 << 1);
    }
}
